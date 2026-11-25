#!/usr/bin/env node
/**
 * iris-v4-mcp — MCP server for InterSystems IRIS over the Atelier REST API **v4**.
 *
 * Built for this project's local HealthConnect 2020.1.1 (WSL) instance, whose Atelier
 * API maxes out at v4. The official iris-agentic-dev MCP requires Atelier v8
 * (IRIS 2023.1+) and cannot connect to it; this server re-implements its core tool
 * surface on the v4 API only. Zero npm dependencies; Node >= 18.
 *
 * Connection resolution (first match wins):
 *   1. Env: IRIS_HOST, IRIS_WEB_PORT, IRIS_SCHEME, IRIS_USERNAME, IRIS_PASSWORD, IRIS_NAMESPACE
 *   2. Workspace .vscode/settings.json → objectscript.conn (server ref + ns)
 *   3. VS Code user settings.json → intersystems.servers[IRIS_SERVER_NAME || "jhip"]
 *      (host/port/user/password — same discovery the official server uses)
 *   4. Defaults: http://localhost:52773, ns USER, _SYSTEM
 *
 * The WSL NAT IP changes across reboots: when a request fails at the network level,
 * the server re-resolves it via `wsl.exe hostname -I` and retries once.
 *
 * ObjectScript execution ("iris_execute") has no v4 REST route; it is implemented via
 * a tiny SqlProc helper class (JHIP.MCP.Run → CALL JHIP_MCP.Run_Exec(?)) that this
 * server installs into the target namespace on first use. The Atelier query action
 * captures the executed code's Write output and returns it in the console array.
 * Repo copy of the helper: src/JHIP/MCP/Run.cls.
 *
 * CLI: node iris-v4-mcp.mjs [--check | --install-helper | --verbose]
 *   --check            resolve config, probe IRIS, print a report (no MCP)
 *   --install-helper   PUT+compile the exec helper into the default namespace, exit
 *   --verbose          log every HTTP call to stderr
 */
import readline from 'node:readline';
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = path.resolve(SCRIPT_DIR, '..');

// ---------------------------------------------------------------------------
// The exec helper class. KEEP IN SYNC with src/JHIP/MCP/Run.cls.
// ---------------------------------------------------------------------------
const HELPER_DOC = 'JHIP.MCP.Run.cls';
const HELPER_CALL = 'JHIP_MCP.Run_Exec';
const HELPER_SOURCE = `Class JHIP.MCP.Run Extends %RegisteredObject
{

/// Exec helper for the iris-v4-mcp MCP server (Atelier API v4, IRIS/HealthConnect 2020.1).
/// Invoke as: CALL JHIP_MCP.Run_Exec(?) with the ObjectScript code as the parameter.
/// Output written by the code is captured by the Atelier query action and returned in
/// its console array. Installed and kept in sync by mcp/iris-v4-mcp.mjs; this file is
/// the repo copy — the server embeds the same source at mcp/iris-v4-mcp.mjs
/// (HELPER_SOURCE) and syncs it into a namespace on use.
ClassMethod Exec(code As %String = "") As %String [ SqlProc ]
{
	If (code = "") Write "*** EXECUTION FAILED ***: no code provided", ! Quit ""
	Try {
		Set tFlat = ..Flatten(code)
		XECUTE tFlat
	} Catch e {
		Write !, "*** EXECUTION FAILED ***: "_e.AsSystemError(), !
	}
	Quit ""
}

/// Collapse multi-line ObjectScript into the single line that XECUTE requires.
///
/// XECUTE takes one or more comma-separated *arguments*, each executed as a line; a
/// newline inside a single argument is a <SYNTAX> error. That is why handing raw code
/// straight to XECUTE only ever worked for one-liners. ObjectScript uses a newline purely
/// as a statement separator, and a space separates commands just as well, so joining the
/// lines with a space is semantically equivalent — apart from two things that would
/// change the meaning and are handled first:
///   * \`//\` and \`;\` comments are stripped, or the join would comment out the rest;
///   * blank lines and \`#;\` comments are dropped (a lone \`#\` is never valid ObjectScript,
///     so what a \`#;\` comment leaves behind is dropped too).
/// String literals are tracked so \`//\` in "http://x" and \`;\` in "a;b" survive. Lines
/// ending in \`_\` need no special case: \`Set x = "a" _\` joined with \`"b"\` stays valid.
ClassMethod Flatten(pCode As %String) As %String
{
	Set tOut = ""
	Set tText = $Translate(pCode, $Char(13), "")
	Set tCount = $Length(tText, $Char(10))
	For tI = 1:1:tCount {
		Set tLine = ..StripComment($Piece(tText, $Char(10), tI))
		Set tBare = $Translate(tLine, " "_$Char(9))
		If (tBare = "") Continue
		If (tBare = "#") Continue
		Set tOut = $Select(tOut = "": tLine, 1: tOut _" "_tLine)
	}
	Quit tOut
}

/// Remove a trailing \`//\` or \`;\` comment, ignoring both inside string literals.
/// An escaped quote (\`""\`) closes and immediately reopens the literal, so the toggle
/// below stays correct without a special case.
///
/// The scan runs to the end and keeps its answer in a variable: \`Quit value\` inside a
/// nested block is a <COMMAND> error, and a flag is cheaper to follow than restructuring
/// the loop around it.
ClassMethod StripComment(pLine As %String) As %String
{
	Set tOut = pLine
	Set tInStr = 0
	Set tDone = 0
	Set tLen = $Length(pLine)
	Set tI = 1
	While ((tI <= tLen) && (tDone = 0)) {
		Set tCh = $Extract(pLine, tI)
		If (tCh = """") {
			Set tInStr = 'tInStr
		} ElseIf (tInStr = 0) {
			If (tCh = ";") {
				Set tOut = $Extract(pLine, 1, tI - 1)
				Set tDone = 1
			} ElseIf (tCh = "/") {
				If ($Extract(pLine, tI + 1) = "/") {
					Set tOut = $Extract(pLine, 1, tI - 1)
					Set tDone = 1
				}
			}
		}
		Set tI = tI + 1
	}
	Quit tOut
}

}`;

const EXEC_FAIL_MARKER = '*** EXECUTION FAILED ***';
// This file is CRLF, so the template literal above carries \r\n. Atelier stores one
// document line per array entry, so normalise once here instead of pushing stray CRs.
const HELPER_LINES = HELPER_SOURCE.replace(/\r\n/g, '\n').split('\n');
const MAX_OUTPUT_CHARS = 150_000;
const MAX_ROWS_SHOWN = 500;

const VERBOSE = process.argv.includes('--verbose');
const log = (...a) => process.stderr.write(a.join(' ') + '\n');

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------
/** Parse JSON that may contain VS Code style // comments and trailing commas. */
function parseLooseJson(text) {
  try { return JSON.parse(text); } catch { /* fall through */ }
  // Strip comments (outside strings) and trailing commas.
  let out = '', inStr = false, esc = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i], next = text[i + 1];
    if (esc) { out += c; esc = false; continue; }
    if (c === '\\' && inStr) { out += c; esc = true; continue; }
    if (c === '"') { inStr = !inStr; out += c; continue; }
    if (inStr) { out += c; continue; }
    if (c === '/' && next === '/') { while (i < text.length && text[i] !== '\n') i++; continue; }
    if (c === '/' && next === '*') { i += 2; while (i < text.length && !(text[i] === '*' && text[i + 1] === '/')) i++; i++; continue; }
    out += c;
  }
  // Remove trailing commas before } or ]
  out = out.replace(/,(\s*[}\]])/g, '$1');
  return JSON.parse(out);
}

function readJsonIfExists(p) {
  try { return parseLooseJson(readFileSync(p, 'utf8')); } catch { return null; }
}

let cachedCfg = null;
let liveHost = null; // set when the WSL IP refresh succeeded

function resolveConfig() {
  const env = process.env;
  const ws = readJsonIfExists(path.join(PROJECT_ROOT, '.vscode', 'settings.json')) || {};
  const user = readJsonIfExists(path.join(env.APPDATA || '', 'Code', 'User', 'settings.json')) || {};
  const serverKey = env.IRIS_SERVER_NAME || ws['objectscript.conn']?.server || 'jhip';
  const srv = user['intersystems.servers']?.[serverKey];
  const wsConn = ws['objectscript.conn'] || {};
  const cfg = {
    host: env.IRIS_HOST || srv?.webServer?.host || wsConn.host || 'localhost',
    port: Number(env.IRIS_WEB_PORT || srv?.webServer?.port || wsConn.port || 52773),
    scheme: env.IRIS_SCHEME || srv?.webServer?.scheme || 'http',
    username: env.IRIS_USERNAME || srv?.username || wsConn.username || '_SYSTEM',
    password: env.IRIS_PASSWORD || srv?.password || wsConn.password || 'SYS',
    namespace: env.IRIS_NAMESPACE || wsConn.ns || 'USER',
    serverKey,
  };
  cfg.source = {
    host: env.IRIS_HOST ? 'env' : (srv ? `vscode:intersystems.servers.${serverKey}` : (wsConn.host ? 'vscode:objectscript.conn' : 'default')),
    namespace: env.IRIS_NAMESPACE ? 'env' : (wsConn.ns ? 'vscode:objectscript.conn' : 'default'),
    credentials: env.IRIS_PASSWORD ? 'env' : (srv?.password ? `vscode:intersystems.servers.${serverKey}` : 'default'),
  };
  return cfg;
}

function getConfig() {
  if (!cachedCfg) cachedCfg = resolveConfig();
  return cachedCfg;
}

/** Current effective host (liveHost overrides the configured one after a WSL refresh). */
function effectiveHost() {
  const cfg = getConfig();
  return liveHost || cfg.host;
}

function baseUrl() {
  const cfg = getConfig();
  return `${cfg.scheme}://${effectiveHost()}:${cfg.port}/api/atelier/v4`;
}

/** Ask WSL for its NAT IP (it changes across reboots). Returns null on failure. */
function wslIp() {
  try {
    const r = spawnSync('wsl.exe', ['hostname', '-I'], { encoding: 'utf8', timeout: 15000 });
    const ip = (r.stdout || '').trim().split(/\s+/)[0];
    return /^\d+\.\d+\.\d+\.\d+$/.test(ip) ? ip : null;
  } catch { return null; }
}

// ---------------------------------------------------------------------------
// Atelier v4 HTTP client
// ---------------------------------------------------------------------------
class AtelierError extends Error {
  constructor(message, { status, detail } = {}) {
    super(message);
    this.name = 'AtelierError';
    this.status = status;
    this.detail = detail;
  }
}

function extractErrors(body) {
  const errs = body?.status?.errors;
  if (!Array.isArray(errs) || errs.length === 0) return null;
  return errs.map((e) => e.error || e.summary || JSON.stringify(e)).join('\n');
}

async function irisFetch(method, urlPath, { body, contentType = 'application/json', timeoutMs = 120000 } = {}) {
  const cfg = getConfig();
  const auth = 'Basic ' + Buffer.from(`${cfg.username}:${cfg.password}`).toString('base64');
  const doFetch = async (host) => {
    const url = `${cfg.scheme}://${host}:${cfg.port}/api/atelier/v4${urlPath}`;
    const ctl = new AbortController();
    const t = setTimeout(() => ctl.abort(), timeoutMs);
    try {
      VERBOSE && log(`>> ${method} ${url}`);
      const res = await fetch(url, {
        method, signal: ctl.signal,
        headers: { Authorization: auth, ...(body !== undefined ? { 'Content-Type': contentType } : {}) },
        body: body !== undefined ? body : undefined,
      });
      const text = await res.text();
      VERBOSE && log(`<< ${res.status} (${text.length}b)`);
      let json = null;
      try { json = text ? JSON.parse(text) : null; } catch { /* non-JSON */ }
      return { res, json, text };
    } finally { clearTimeout(t); }
  };

  let host = effectiveHost();
  let r = await doFetch(host).catch((e) => ({ networkError: e }));

  // Network-level failure: maybe the WSL IP rotated. Re-resolve and retry once.
  if (r.networkError && host !== 'localhost' && host !== '127.0.0.1') {
    const ip = wslIp();
    if (ip && ip !== host) {
      log(`[iris-v4-mcp] network failure to ${host} (${r.networkError.cause?.code || r.networkError.message}); WSL IP is ${ip} — retrying`);
      host = ip;
      r = await doFetch(host).catch((e) => ({ networkError: e }));
      if (!r.networkError) liveHost = ip;
    }
  }
  if (r.networkError) {
    const code = r.networkError.cause?.code || '';
    throw new AtelierError(
      `Cannot reach IRIS at ${cfg.scheme}://${host}:${cfg.port} (${code || r.networkError.message}). ` +
      `Hint: the WSL IP changes across reboots; run 'wsl hostname -I' and compare with the configured host.`,
      { status: 0 });
  }
  const { res, json, text } = r;
  if (!res.ok) {
    const detail = extractErrors(json) || text?.slice(0, 500) || res.statusText;
    throw new AtelierError(`HTTP ${res.status}: ${detail}`, { status: res.status, detail: json });
  }
  const errs = extractErrors(json);
  if (errs && res.status !== 304) throw new AtelierError(errs, { status: res.status, detail: json });
  return json;
}

// Convenience wrappers -------------------------------------------------------
const api = {
  query: (ns, sql, params = []) =>
    irisFetch('POST', `/${ns}/action/query`, { body: JSON.stringify({ query: sql, parameters: params }) }),
  getDoc: (ns, name) => irisFetch('GET', `/${ns}/doc/${encodeURIComponent(name)}`),
  putDoc: (ns, name, lines) =>
    irisFetch('PUT', `/${ns}/doc/${encodeURIComponent(name)}?ignoreConflict=1`, {
      body: JSON.stringify({ enc: false, content: lines }),
    }),
  deleteDoc: (ns, name) => irisFetch('DELETE', `/${ns}/doc/${encodeURIComponent(name)}`),
  docnames: (ns, cat = '*', type = null, filter = null) => {
    let p = `/${ns}/docnames${cat && cat !== '*' ? '/' + encodeURIComponent(cat) : ''}`;
    if (type) p += '/' + encodeURIComponent(type);
    if (filter) p += '?filter=' + encodeURIComponent(filter);
    return irisFetch('GET', p);
  },
  compile: (ns, targets, flags = 'cuk') =>
    irisFetch('POST', `/${ns}/action/compile?flags=${encodeURIComponent(flags)}&source=0`, {
      body: JSON.stringify(targets),
    }),
  search: (ns, params) => {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v !== undefined && v !== null) q.set(k, String(v));
    return irisFetch('GET', `/${ns}/action/search?` + q.toString());
  },
};

// serverInfo needs the versionless root; do it directly (with one WSL retry):
async function serverRootInfo() {
  const cfg = getConfig();
  const attempt = async (host) => {
    const auth = 'Basic ' + Buffer.from(`${cfg.username}:${cfg.password}`).toString('base64');
    try {
      const res = await fetch(`${cfg.scheme}://${host}:${cfg.port}/api/atelier/`, { headers: { Authorization: auth } });
      const json = await res.json().catch(() => null);
      if (!res.ok) return { ok: false, http: res.status };
      return { ok: true, info: json?.result?.content || null };
    } catch (e) {
      return { ok: false, error: String(e.cause?.code || e.message) };
    }
  };
  let r = await attempt(effectiveHost());
  if (!r.ok) {
    const ip = wslIp();
    if (ip && ip !== effectiveHost()) {
      r = await attempt(ip);
      if (r.ok) liveHost = ip;
    }
  }
  return r;
}

// ---------------------------------------------------------------------------
// Exec helper bootstrap + execution
// ---------------------------------------------------------------------------
const helperReady = new Set(); // namespaces where the helper is known-good

/**
 * Compare two document bodies, ignoring line-ending and storage normalisation.
 *
 * Trailing whitespace is ignored on purpose: IRIS rewrites an empty doc-comment line
 * (`///`) as `/// ` when it stores a class, so a byte-exact comparison never matched and
 * every check re-pushed and re-compiled the helper. Trailing whitespace in ObjectScript
 * source is never significant — a string literal cannot span lines, so anything after the
 * last token on a line is whitespace either way.
 */
function sameLines(a, b) {
  const norm = (arr) => {
    const out = arr.map((l) => String(l).replace(/\r$/, '').replace(/\s+$/, ''));
    while (out.length && out[out.length - 1] === '') out.pop();
    return out;
  };
  const x = norm(a);
  const y = norm(b);
  return x.length === y.length && x.every((line, i) => line === y[i]);
}

/**
 * Make sure the namespace holds the current HELPER_SOURCE.
 *
 * The old check only asked "does the document exist?" — so once a namespace had *any*
 * version of the helper, an edited HELPER_SOURCE was never pushed again: the server kept
 * running stale ObjectScript while the repo said otherwise, and `--install-helper` (which
 * goes through this same function) silently did nothing. Comparing the bodies is what
 * makes an edit here actually reach the server.
 *
 * @returns {'present'|'installed'|'updated'} what the namespace needed.
 */
async function ensureHelper(ns, { force = false } = {}) {
  if (!force && helperReady.has(ns)) return 'present';
  const doc = await api.getDoc(ns, HELPER_DOC).catch((e) => {
    if (e.status === 404) return null;
    throw e;
  });
  const have = Array.isArray(doc?.result?.content) ? doc.result.content : null;
  const want = HELPER_LINES;
  if (!force && have && sameLines(have, want)) {
    helperReady.add(ns);
    return 'present';
  }
  await api.putDoc(ns, HELPER_DOC, want);
  const r = await api.compile(ns, [HELPER_DOC]);
  const failed = (r?.result?.content || []).some((d) => d.status && String(d.status).includes('ERROR'));
  if (failed) throw new AtelierError(`Helper class ${HELPER_DOC} failed to compile`, { detail: r });
  helperReady.add(ns);
  return have ? 'updated' : 'installed';
}

/** Run ObjectScript, return { output, failed }. */
async function execObjectScript(ns, code) {
  await ensureHelper(ns);
  const r = await api.query(ns, `CALL ${HELPER_CALL}(?)`, [code]);
  let lines = Array.isArray(r?.console) ? r.console : [];
  while (lines.length && lines[0] === '') lines.shift();
  while (lines.length && lines[lines.length - 1] === '') lines.pop();
  const output = lines.join('\n');
  const failed = lines.some((l) => l.includes(EXEC_FAIL_MARKER));
  return { output, failed };
}

/** Quote a string for embedding inside an ObjectScript string literal. */
const cosQ = (s) => '"' + String(s).replace(/"/g, '""') + '"';

// ---------------------------------------------------------------------------
// Output helpers
// ---------------------------------------------------------------------------
function cap(text, limit = MAX_OUTPUT_CHARS) {
  if (text.length <= limit) return text;
  return text.slice(0, limit) + `\n… [output truncated at ${limit} chars]`;
}

function fmtRows(rows) {
  if (!Array.isArray(rows)) return '';
  if (rows.length === 0) return '(no rows)';
  let out = rows.slice(0, MAX_ROWS_SHOWN).map((r) => JSON.stringify(r)).join('\n');
  if (rows.length > MAX_ROWS_SHOWN) out += `\n… [${rows.length - MAX_ROWS_SHOWN} more rows not shown]`;
  return out;
}

const textResult = (text, isError = false) => ({ content: [{ type: 'text', text: cap(text) }], ...(isError ? { isError: true } : {}) });

// ---------------------------------------------------------------------------
// Tools
// ---------------------------------------------------------------------------
const NS_PARAM = {
  type: 'string',
  description: 'IRIS namespace (default: the configured one, JHIP).',
};

const TOOLS = [
  {
    name: 'check_config',
    description:
      'Show the resolved IRIS connection (host, port, namespace, username, where each value came from), ' +
      'probe the server (IRIS version, Atelier API version, available namespaces), and check the exec helper. ' +
      'Run this first when anything fails.',
    inputSchema: { type: 'object', properties: {}, additionalProperties: false },
    handler: async () => {
      const cfg = getConfig();
      const root = await serverRootInfo();
      const lines = [
        `connection: ${cfg.scheme}://${effectiveHost()}:${cfg.port}  (configured host: ${cfg.host}${liveHost ? ` — live host overridden to ${liveHost} after WSL IP refresh` : ''})`,
        `namespace: ${cfg.namespace}   user: ${cfg.username}`,
        `config sources: host=${cfg.source.host}, namespace=${cfg.source.namespace}, credentials=${cfg.source.credentials}`,
      ];
      if (!root.ok) {
        lines.push(`server probe FAILED: ${root.http ? 'HTTP ' + root.http : root.error}`);
        if (liveHost === null) {
          const ip = wslIp();
          lines.push(ip ? `wsl hostname -I → ${ip} (if it differs from the configured host, update the VS Code jhip server entry)` : 'wsl hostname -I → unavailable');
        }
        return textResult(lines.join('\n'), true);
      }
      const i = root.info || {};
      lines.push(`server: ${i.version || '?'}`);
      lines.push(`atelier api version: ${i.api} (this MCP requires exactly 4)`);
      lines.push(`features: ${(i.features || []).map((f) => `${f.name}=${f.enabled}`).join(', ')}`);
      lines.push(`namespaces: ${(i.namespaces || []).join(', ')}`);
      let helper = 'unknown';
      try { const act = await ensureHelper(cfg.namespace); helper = `${act} (${HELPER_DOC}, callable as ${HELPER_CALL})`; }
      catch (e) { helper = `NOT working: ${e.message}`; }
      lines.push(`exec helper (${cfg.namespace}): ${helper}`);
      lines.push(`api level check: ${i.api === 4 ? 'OK — Atelier v4' : `UNEXPECTED: api=${i.api}`}`);
      return textResult(lines.join('\n'));
    },
  },
  {
    name: 'iris_query',
    description:
      'Run SQL in a namespace via the Atelier query action. SELECT/CALL return rows as JSON lines. ' +
      'Schema rule: a class Package.Sub.Class maps to SQL table Package_Sub.Class (dots in the package become ' +
      'underscores); e.g. Ens.MessageHeader but Ens_Config.Item, Ens_Util.Log. Always use TOP n for large tables — ' +
      'rows are capped at 500 in the output. Non-SELECT statements (INSERT/UPDATE/DELETE/DDL) execute but return ' +
      'no rows and no rowcount via this API; verify with a follow-up SELECT. Parameters: use ? and pass params.',
    inputSchema: {
      type: 'object',
      properties: {
        sql: { type: 'string', description: 'SQL statement, e.g. "SELECT TOP 20 ID, TimeCreated FROM Ens.MessageHeader ORDER BY ID DESC".' },
        params: { type: 'array', items: {}, description: 'Values for ? placeholders, in order.' },
        namespace: NS_PARAM,
      },
      required: ['sql'],
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const r = await api.query(ns, args.sql, args.params || []);
      const rows = r?.result?.content;
      const parts = [];
      if (Array.isArray(rows)) {
        if (rows.length === 0) {
          parts.push('(statement executed — no result set. Non-SELECT statements return no rows and no rowcount via Atelier v4.)');
        } else {
          parts.push(`${rows.length} row(s):`);
          parts.push(fmtRows(rows));
        }
      }
      const console_ = (r?.console || []).filter((l) => l !== '');
      if (console_.length) parts.push('console:\n' + console_.join('\n'));
      return textResult(parts.join('\n\n'));
    },
  },
  {
    name: 'iris_execute',
    description:
      'Execute arbitrary ObjectScript in a namespace and return captured Write output (runs XECUTE inside a ' +
      'SqlProc helper, CALL JHIP_MCP.Run_Exec). Quirks in this context: use $ZNSPACE to read the namespace ' +
      '($ZNAMESPACE and $ZLEVEL throw <SYNTAX> here); no interactive Read; avoid HALT. ' +
      'Examples: \'Write ##class(Ens.Director).GetActiveProductionName(), !\' ; ' +
      '\'Do ##class(Ens.Director).GetProductionStatus(.n,.s) Write n,!,s,!\' ; ' +
      '\'Set b=##class("My.Msg").%OpenId(33) ZWrite b\'',
    inputSchema: {
      type: 'object',
      properties: { code: { type: 'string', description: 'ObjectScript to execute.' }, namespace: NS_PARAM },
      required: ['code'],
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const { output, failed } = await execObjectScript(ns, args.code);
      return textResult(output === '' ? '(no output)' : output, failed);
    },
  },
  {
    name: 'iris_doc',
    description:
      'Get, put (create/overwrite) or delete a server document (class .cls, routine .mac/.int/.inc, etc.). ' +
      'Names are server doc names like JHIP.Application.BP.Check.cls. put does NOT compile — follow with iris_compile. ' +
      'Binary docs (e.g. some CSP) are returned base64-decoded if valid UTF-8, else reported as binary.',
    inputSchema: {
      type: 'object',
      properties: {
        name: { type: 'string', description: 'Document name, e.g. JHIP.MCP.Run.cls or MyRoutine.mac.' },
        mode: { type: 'string', enum: ['get', 'put', 'delete'], description: 'Default get.' },
        content: { type: 'string', description: 'Full new content for mode=put.' },
        namespace: NS_PARAM,
      },
      required: ['name'],
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const mode = args.mode || 'get';
      if (mode === 'get') {
        const r = await api.getDoc(ns, args.name);
        const d = r?.result || {};
        const header = `${d.name}  [${d.cat}, db=${d.db}, ts=${d.ts}]`;
        let text;
        if (d.enc) {
          const buf = Buffer.from(Array.isArray(d.content) ? d.content.join('') : String(d.content), 'base64');
          try { text = new TextDecoder('utf-8', { fatal: true }).decode(buf); }
          catch { return textResult(`${header}\nbinary document (${buf.length} bytes), not UTF-8 text.`, false); }
        } else {
          text = Array.isArray(d.content) ? d.content.join('\n') : String(d.content ?? '');
        }
        return textResult(`${header}\n${text}`);
      }
      if (mode === 'put') {
        if (typeof args.content !== 'string') return textResult('mode=put requires "content".', true);
        let lines = args.content.split('\n').map((l) => l.replace(/\r$/, ''));
        if (lines.length && lines[lines.length - 1] === '') lines.pop();
        const r = await api.putDoc(ns, args.name, lines);
        // Atelier v4 reports document-text failures in result.status (HTTP 200, status.errors empty) —
        // e.g. "ERROR #5001: Couldn't set text ... Class name is invalid" for some include layouts.
        const putError = r?.result?.status;
        if (putError) return textResult(`ERROR saving ${args.name} in ${ns}: ${putError}`, true);
        return textResult(`saved ${args.name} in ${ns} (ts=${r?.result?.ts}). Compile it with iris_compile when ready.`);
      }
      const r = await api.deleteDoc(ns, args.name);
      return textResult(`deleted ${args.name} from ${ns}.`);
    },
  },
  {
    name: 'iris_docnames',
    description:
      'List server documents by category. Categories: CLS (classes), RTN (routines .mac/.int/.inc), CSP, OTH, * (all). ' +
      'filter uses SQL LIKE syntax with % as wildcard (e.g. "JHIP.%" or "Ens.Config.%"); type narrows RTN (mac/int/inc). ' +
      'Note: a namespace also sees mapped system databases (IRISLIB/HSLIB/ENSLIB) — use filter to focus on your code.',
    inputSchema: {
      type: 'object',
      properties: {
        category: { type: 'string', enum: ['*', 'CLS', 'RTN', 'CSP', 'OTH'], description: 'Default *.' },
        type: { type: 'string', description: 'File type for RTN category: mac, int, inc, bas.' },
        filter: { type: 'string', description: "SQL LIKE pattern on the name, e.g. 'JHIP.%'." },
        generated: { type: 'boolean', description: 'Include generated docs (default false).' },
        limit: { type: 'integer', description: 'Max names shown (default 200, hard cap 5000).' },
        namespace: NS_PARAM,
      },
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const r = await api.docnames(ns, args.category || '*', args.type || null, args.filter || null);
      const list = r?.result?.content || [];
      const limit = Math.min(Math.max(args.limit ?? 200, 1), 5000);
      const shown = list.slice(0, limit).map((d) => `${d.name}  [${d.cat}, ${d.db}${d.gen ? ', generated' : ''}]`);
      let out = `${list.length} document(s)${args.filter ? ` matching '${args.filter}'` : ''} in ${ns}:\n` + shown.join('\n');
      if (list.length > limit) out += `\n… [${list.length - limit} more, raise "limit" to see]`;
      return textResult(out);
    },
  },
  {
    name: 'iris_compile',
    description:
      'Compile server documents. Compile flags default "cuk" (same as the VS Code ObjectScript extension). ' +
      'The response contains the compile console log and per-document status. Use after iris_doc put or direct edits.',
    inputSchema: {
      type: 'object',
      properties: {
        targets: { type: 'array', items: { type: 'string' }, description: 'Document names, e.g. ["JHIP.Application.BP.Check.cls"].' },
        flags: { type: 'string', description: 'Compile qualifiers (default cuk).' },
        namespace: NS_PARAM,
      },
      required: ['targets'],
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const r = await api.compile(ns, args.targets, args.flags || 'cuk');
      const parts = [];
      const console_ = (r?.console || []).filter((l) => l !== '');
      if (console_.length) parts.push('compile log:\n' + console_.join('\n'));
      const content = r?.result?.content || [];
      const bad = content.filter((d) => d.status && String(d.status).match(/error/i));
      parts.push(bad.length ? `FAILED for ${bad.length} doc(s):\n${JSON.stringify(bad, null, 1)}` : `compiled ${args.targets.length} doc(s)`);
      const failed = bad.length > 0 || console_.some((l) => /error/i.test(l));
      return textResult(parts.join('\n\n'), failed);
    },
  },
  {
    name: 'iris_search',
    description:
      'Server-side content search across source documents (Atelier action/search). query is the text to find ' +
      '(or a regex when regex=true — regex matches whole lines); files is a comma-separated mask list, ' +
      'default "*.cls,*.mac,*.int,*.inc". sys=true includes system (%) docs. Returns per-document matches with member/line.',
    inputSchema: {
      type: 'object',
      properties: {
        query: { type: 'string', description: 'Text or regex to search for.' },
        files: { type: 'string', description: 'Comma-separated file masks (default *.cls,*.mac,*.int,*.inc).' },
        regex: { type: 'boolean', description: 'Treat query as a regular expression (default false).' },
        case: { type: 'boolean', description: 'Case-sensitive (with plain text).' },
        word: { type: 'boolean', description: 'Whole-word match (with plain text).' },
        wild: { type: 'boolean', description: 'Wildcard match (with plain text).' },
        sys: { type: 'boolean', description: 'Include system (%-prefixed) documents.' },
        gen: { type: 'boolean', description: 'Include generated documents.' },
        max: { type: 'integer', description: 'Max results (default 100).' },
        namespace: NS_PARAM,
      },
      required: ['query'],
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const r = await api.search(ns, {
        query: args.query,
        files: args.files || '*.cls,*.mac,*.int,*.inc',
        regex: args.regex ? 1 : 0,
        ...(args.case !== undefined ? { case: args.case ? 1 : 0 } : {}),
        ...(args.word !== undefined ? { word: args.word ? 1 : 0 } : {}),
        ...(args.wild !== undefined ? { wild: args.wild ? 1 : 0 } : {}),
        sys: args.sys ? 1 : 0,
        gen: args.gen ? 1 : 0,
        max: args.max || 100,
      });
      const results = Array.isArray(r?.result) ? r.result : (r?.result?.content || []);
      const console_ = (r?.console || []).filter((l) => l !== '');
      const lines = results.map((d) =>
        `${d.doc}:\n` + (d.matches || []).map((m) => `  ${m.line ? 'line ' + m.line + ': ' : (m.member ? m.member + ': ' : '')}${m.text}`).join('\n'));
      const out = [console_.join('\n'), lines.join('\n\n')].filter(Boolean).join('\n\n');
      return textResult(out || '(no matches)');
    },
  },
  {
    name: 'iris_production',
    description:
      'Interoperability production tools for this HealthConnect namespace. Actions: ' +
      'status (running production + state), items (config items of a production, default the active one), ' +
      'event_log (Ens_Util.Log, newest first), messages (Ens.MessageHeader traffic, newest first, filter by session_id), ' +
      'start/stop/restart (require confirm:true). Message bodies: follow up with iris_execute, e.g. ' +
      '\'Set h=##class(Ens.MessageHeader).%OpenId(58) Set b=##class(h.MessageBodyClassName).%OpenId(h.MessageBodyId) ZWrite b\'.',
    inputSchema: {
      type: 'object',
      properties: {
        action: { type: 'string', enum: ['status', 'items', 'event_log', 'messages', 'start', 'stop', 'restart'], description: 'Default status.' },
        production: { type: 'string', description: 'Production config name (items/start), e.g. JHIP.Production.JHIPV3. Defaults to the active one.' },
        session_id: { type: 'integer', description: 'Filter event_log/messages by session.' },
        type: { type: 'integer', description: 'Filter event_log by type (0=info,2=error,4=trace…).' },
        limit: { type: 'integer', description: 'Row cap for items/event_log/messages (default 20, max 200).' },
        confirm: { type: 'boolean', description: 'Required true for start/stop/restart.' },
        namespace: NS_PARAM,
      },
      additionalProperties: false,
    },
    handler: async (args) => {
      const ns = args.namespace || getConfig().namespace;
      const action = args.action || 'status';
      const limit = Math.min(Math.max(args.limit ?? 20, 1), 200);
      if (action === 'status') {
        const code = 'Do ##class(Ens.Director).GetProductionStatus(.n,.s) Write "production=" _ n, !, "state=" _ s, !, "running=" _ ##class(Ens.Director).IsProductionRunning(.rn), !, "runningname=" _ rn, !';
        const { output, failed } = await execObjectScript(ns, code);
        return textResult(output + '\n(state: 0=stopped, 1=running, 2=troubled, 3=suspended; isrunning is the most reliable signal)', failed);
      }
      if (action === 'items') {
        let prod = args.production;
        if (!prod) {
          const { output } = await execObjectScript(ns, 'Write ##class(Ens.Director).GetActiveProductionName(), !');
          prod = output.trim();
        }
        const r = await api.query(ns,
          'SELECT Name, ClassName, Enabled FROM Ens_Config.Item WHERE Production = ? ORDER BY Enabled DESC, Name',
          [prod]);
        const rows = r?.result?.content || [];
        const shown = rows.slice(0, limit);
        return textResult(`${rows.length} config item(s) in ${prod}:\n` +
          shown.map((x) => `${x.Enabled ? '[x]' : '[ ]'} ${x.Name}  (${x.ClassName})`).join('\n') +
          (rows.length > limit ? `\n… [${rows.length - limit} more — raise "limit" to see]` : ''));
      }
      if (action === 'event_log') {
        let sql = `SELECT TOP ${limit} ID, TimeLogged, Type, ConfigName, SessionId, SourceClass, SourceMethod, Text FROM Ens_Util.Log`;
        const where = [], params = [];
        if (args.session_id !== undefined) { where.push('SessionId = ?'); params.push(args.session_id); }
        if (args.type !== undefined) { where.push('Type = ?'); params.push(args.type); }
        if (where.length) sql += ' WHERE ' + where.join(' AND ');
        sql += ' ORDER BY ID DESC';
        const r = await api.query(ns, sql, params);
        return textResult(fmtRows(r?.result?.content || []) + '\n(Type: 0=info,2=error,4=trace; full row detail is in each JSON line.)');
      }
      if (action === 'messages') {
        let sql = `SELECT TOP ${limit} ID, SessionId, TimeCreated, SourceConfigName, TargetConfigName, MessageBodyClassName, MessageBodyId, Status, IsError FROM Ens.MessageHeader`;
        if (args.session_id !== undefined) sql += ' WHERE SessionId = ' + Number(args.session_id);
        sql += ' ORDER BY ID DESC';
        const r = await api.query(ns, sql);
        return textResult(fmtRows(r?.result?.content || []));
      }
      // start / stop / restart — mutating, gated on confirm
      if (args.confirm !== true) {
        return textResult(`Production ${action} is a mutating operation. Re-run with confirm:true to proceed.`, true);
      }
      const calls = {
        start: (p) => `Set sc=##class(Ens.Director).StartProduction(${p ? cosQ(p) : ''})`,
        stop: () => 'Set sc=##class(Ens.Director).StopProduction()',
        restart: () => 'Set sc=##class(Ens.Director).RestartProduction()',
      };
      const code = calls[action](args.production) + ' If sc Write "OK", ! Else Write $System.Status.GetErrorText(sc), !';
      const { output, failed } = await execObjectScript(ns, code);
      return textResult(`${action} production: ${output}`, failed);
    },
  },
];

// ---------------------------------------------------------------------------
// MCP stdio server
// ---------------------------------------------------------------------------
const SERVER_INFO = { name: 'iris-v4-mcp', version: '1.0.0' };

function rpcResult(id, result) { process.stdout.write(JSON.stringify({ jsonrpc: '2.0', id, result }) + '\n'); }
function rpcError(id, code, message) { process.stdout.write(JSON.stringify({ jsonrpc: '2.0', id, error: { code, message } }) + '\n'); }

async function handleMessage(msg) {
  if (!msg || typeof msg !== 'object') return;
  const { id, method, params } = msg;
  const isRequest = id !== undefined && id !== null;
  try {
    switch (method) {
      case 'initialize':
        rpcResult(id, {
          protocolVersion: params?.protocolVersion || '2024-11-05',
          capabilities: { tools: { listChanged: false } },
          serverInfo: SERVER_INFO,
        });
        return;
      case 'notifications/initialized':
      case 'notifications/cancelled':
      case 'notifications/progress':
        return; // no response
      case 'ping':
        rpcResult(id, {});
        return;
      case 'tools/list':
        rpcResult(id, {
          tools: TOOLS.map(({ name, description, inputSchema }) => ({ name, description, inputSchema })),
        });
        return;
      case 'tools/call': {
        const name = params?.name;
        const tool = TOOLS.find((t) => t.name === name);
        if (!tool) { rpcResult(id, { content: [{ type: 'text', text: `Unknown tool: ${name}. Available: ${TOOLS.map((t) => t.name).join(', ')}` }], isError: true }); return; }
        let out;
        try { out = await tool.handler(params?.arguments || {}); }
        catch (e) {
          const msgTxt = e instanceof AtelierError
            ? `IRIS error: ${e.message}`
            : `${e.constructor.name}: ${e.message}`;
          out = { content: [{ type: 'text', text: msgTxt }], isError: true };
        }
        rpcResult(id, out);
        return;
      }
      default:
        if (isRequest) rpcError(id, -32601, `Method not found: ${method}`);
        return;
    }
  } catch (e) {
    if (isRequest) rpcError(id, -32603, `Internal error: ${e.message}`);
  }
}

// ---------------------------------------------------------------------------
// CLI modes
// ---------------------------------------------------------------------------
async function cliCheck() {
  const fake = { name: 'check_config', handler: TOOLS[0].handler };
  const out = await fake.handler({});
  const text = out.content[0].text;
  console.log(text);
  // Let the event loop drain (process.exit trips a libuv assertion on Windows
  // while undici keep-alive sockets are still closing).
  process.exitCode = text.includes('FAILED') || text.includes('NOT working') || text.includes('UNEXPECTED') ? 1 : 0;
}

async function cliInstallHelper() {
  const ns = getConfig().namespace;
  // force: --install-helper is the repair command, so it must push even when the body
  // already matches (a class can look identical and still be broken, e.g. after a failed
  // compile). Routine use goes through ensureHelper's content comparison instead.
  const act = await ensureHelper(ns, { force: true });
  console.log(`${HELPER_DOC} ${act} in ${ns} (callable as CALL ${HELPER_CALL}(?)).`);
}

if (process.argv.includes('--check')) {
  cliCheck().catch((e) => { console.error(String(e)); process.exitCode = 1; });
} else if (process.argv.includes('--install-helper')) {
  cliInstallHelper().catch((e) => { console.error(String(e)); process.exitCode = 1; });
} else {
  const rl = readline.createInterface({ input: process.stdin, terminal: false });
  rl.on('line', (line) => {
    const s = line.trim();
    if (!s) return;
    let msg;
    try { msg = JSON.parse(s); } catch { log('[iris-v4-mcp] non-JSON line ignored'); return; }
    handleMessage(msg).catch((e) => log('[iris-v4-mcp] handler error:', e.message));
  });
  VERBOSE && log(`[iris-v4-mcp] stdio server up (${SERVER_INFO.name} ${SERVER_INFO.version})`);
}
