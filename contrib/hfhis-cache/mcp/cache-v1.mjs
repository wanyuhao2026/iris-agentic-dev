#!/usr/bin/env node
/**
 * cache-v1-mcp — MCP server for InterSystems Caché over the Atelier REST API **v1**.
 *
 * Built for the hfhis-8.17 project's Caché 2016.2.3 instance (10.146.8.17:57772,
 * namespace DHC-APP), whose Atelier API tops out at v1. The upstream
 * iris-agentic-dev MCP targets v8 and cannot drive it; this server re-implements a
 * useful subset on v1 alone. Zero npm dependencies; Node >= 18.
 *
 * What v1 actually offers (measured on 2016.2.3, not assumed):
 *   GET    /api/atelier/                          version + api level + features
 *   GET    /v1/{ns}/docnames[/{cat}[/{type}]]     CLS | RTN/MAC | RTN/INT | RTN/INC | CSP | OTH
 *   GET    /v1/{ns}/doc/{name}                    source as an array of lines
 *   PUT    /v1/{ns}/doc/{name}                    save; body {enc:false, content:[lines]}
 *   DELETE /v1/{ns}/doc/{name}
 *   POST   /v1/{ns}/action/query                  SQL; response carries `console` + `result.content`
 *   POST   /v1/{ns}/action/compile?flags=cuk      body: array of document names
 *
 * What v1 does NOT offer, despite the upstream tool surface implying otherwise:
 *   * /action/search — 404 on this build. There is no server-side code search, and no
 *     substitute: $preprocess (which Atelier's own text reader needs) does not exist on
 *     2016.2.3, %Dictionary.CompiledMethod.Implementation and MethodDefinition.Implementation
 *     are both empty, and ^oddDEF holds method metadata but no bodies. Name lookup goes
 *     through cache_docnames instead.
 *   * /action/execute — so ObjectScript runs through a helper class installed into the
 *     namespace (CacheV1.MCP.Run), invoked as CALL CacheV1_MCP.Run_Exec(?). The query
 *     action captures whatever the code writes and returns it in `console`.
 *
 * Two failure modes this server is built around, both observed here:
 *   * CSP session leak. Every request without a session cookie opens a new CSP session and
 *     holds a license unit until it times out. Caché ignores `?IRISLogout=end` (that is an
 *     IRIS-only parameter) and honours `?CacheLogout=end`, so this server keeps a cookie jar
 *     and ends its session on exit. Leaked sessions are what drove /api/* to 503 before.
 *   * Redirect-to-login. A Caché gateway answers expired credentials with a 302 to an HTML
 *     login page. Following it yields 200 text/html, which parses to null and would read as
 *     an empty success. Every request here uses redirect:'manual' and rejects non-JSON.
 *
 * CLI: node cache-v1.mjs [--check | --install-helper | --sync [--hook] [files...] | --logout | --verbose]
 *   --check            resolve config, probe the server, exercise the helper, print a report
 *   --install-helper   PUT + compile the helper into the default namespace, then exit
 *   --sync             push changed .cls/.mac/.int/.inc files and compile them
 *   --hook             read the editor hook payload on stdin and sync the files it names
 *   --logout           end this server's CSP session and exit
 *   --verbose          log every HTTP call to stderr
 */
import readline from 'node:readline';
import { readFileSync, existsSync, statSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));

/**
 * Find the project this server belongs to: the nearest ancestor holding .cache-v1.toml or
 * .git. Walking up beats a fixed ../.. so the same file works from the deployed location
 * (.tools/cache-v1/) and from a vendored checkout (contrib/hfhis-cache/mcp/).
 */
function findProjectRoot(start) {
  let dir = start;
  for (let i = 0; i < 12; i++) {
    if (existsSync(path.join(dir, '.cache-v1.toml')) || existsSync(path.join(dir, '.git'))) return dir;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return path.resolve(start, '..', '..');
}

const PROJECT_ROOT = findProjectRoot(SCRIPT_DIR);
const CONFIG_FILE = path.join(PROJECT_ROOT, '.cache-v1.toml');

// ---------------------------------------------------------------------------
// The exec helper class. KEEP IN SYNC with mcp/CacheV1/MCP/Run.cls.
// ---------------------------------------------------------------------------
const HELPER_DOC = 'CacheV1.MCP.Run.cls';
const HELPER_CALL = 'CacheV1_MCP.Run_Exec';
const HELPER_COMPILE_CALL = 'CacheV1_MCP.Run_Compile';
const HELPER_SOURCE = `Class CacheV1.MCP.Run Extends %RegisteredObject
{

/// Exec helper for the cache-v1-mcp MCP server (Atelier API v1, Caché 2016.2.3).
/// Invoke as: CALL CacheV1_MCP.Run_Exec(?) with the ObjectScript code as the parameter.
/// Output written by the code is captured by the Atelier query action and returned in
/// its console array. Installed and kept in sync by mcp/cache-v1.mjs; this class is the
/// repo copy - the server embeds the same source (HELPER_SOURCE) and syncs it on use.
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

/// Compile documents and report per-name status in a locale-independent form.
///
/// The Atelier compile action on 2016.2.3 returns an empty result.content and puts its
/// messages in console as localised prose ("Kompilierung ... erfolgreich abgeschlossen"),
/// so parsing that response is not portable. $System.OBJ.Compile returns a %Status and an
/// error log, which is.
///
/// pNames is a comma-separated list of document names; pFlags is a compile qualifier set.
/// A .cls suffix is stripped: $System.OBJ.Compile wants the bare class name and answers
/// "class does not exist" for the document form. Routine extensions (.mac/.int/.inc) are
/// left alone, because those are the names the compiler expects for routines.
ClassMethod Compile(pNames As %String = "", pFlags As %String = "cuk") As %String [ SqlProc ]
{
	Set tCount = $Length(pNames, ",")
	For tI = 1:1:tCount {
		Set tDoc = $Piece(pNames, ",", tI)
		If (tDoc = "") Continue
		Set tName = tDoc
		If ($ZConvert($Extract(tDoc, *-3, *), "L") = ".cls") {
			Set tName = $Extract(tDoc, 1, *-4)
		}
		Kill tErr
		Set tSC = $System.OBJ.Compile(tName, pFlags, .tErr)
		If ($System.Status.IsOK(tSC)) {
			Write "OK ", tDoc, !
		} Else {
			Write "ERROR ", tDoc, ": ", $System.Status.GetErrorText(tSC), !
		}
		Set tJ = ""
		For {
			Set tJ = $Order(tErr(tJ))
			Quit:(tJ = "")
			Write "  ", tErr(tJ), !
		}
	}
	Quit ""
}

/// Collapse multi-line ObjectScript into the single line that XECUTE requires.
///
/// XECUTE takes one or more comma-separated arguments, each executed as a line; a newline
/// inside a single argument is a <SYNTAX> error. ObjectScript uses a newline purely as a
/// statement separator, and a space separates commands just as well, so joining the lines
/// with a space is semantically equivalent - apart from two things that would change the
/// meaning and are handled first:
///   * // and ; comments are stripped, or the join would comment out the rest;
///   * blank lines and #; comments are dropped.
/// String literals are tracked so // in "http://x" and ; in "a;b" survive.
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

/// Remove a trailing // or ; comment, ignoring both inside string literals.
/// An escaped quote ("") closes and immediately reopens the literal, so the toggle below
/// stays correct without a special case.
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
// This file may be CRLF; Atelier stores one document line per array entry, so normalise
// once here instead of pushing stray CRs.
const HELPER_LINES = HELPER_SOURCE.replace(/\r\n/g, '\n').split('\n');
const MAX_OUTPUT_CHARS = 150_000;
const MAX_ROWS_SHOWN = 500;
const MAX_DOCNAMES = 5000;

const VERBOSE = process.argv.includes('--verbose');
const log = (...a) => process.stderr.write(a.join(' ') + '\n');

// ---------------------------------------------------------------------------
// Configuration: .cache-v1.toml
// ---------------------------------------------------------------------------
/**
 * Parse the subset of TOML this file needs: top-level scalars, [section], and
 * [[array.of.tables]]. Deliberately not a general TOML implementation - the config is
 * written by hand and this keeps the server dependency-free.
 */
function parseToml(text) {
  const root = {};
  let table = root;
  for (const raw of text.split(/\r?\n/)) {
    const line = stripTomlComment(raw).trim();
    if (!line) continue;
    let m = /^\[\[(.+)\]\]$/.exec(line);
    if (m) {
      const keys = m[1].trim().split('.').map((k) => k.trim());
      let t = root;
      for (let i = 0; i < keys.length - 1; i++) { t[keys[i]] ??= {}; t = t[keys[i]]; }
      const last = keys[keys.length - 1];
      if (!Array.isArray(t[last])) t[last] = [];
      const obj = {};
      t[last].push(obj);
      table = obj;
      continue;
    }
    m = /^\[(.+)\]$/.exec(line);
    if (m) {
      const keys = m[1].trim().split('.').map((k) => k.trim());
      let t = root;
      for (const k of keys) { t[k] ??= {}; t = t[k]; }
      table = t;
      continue;
    }
    const eq = line.indexOf('=');
    if (eq < 0) continue;
    const key = line.slice(0, eq).trim().replace(/^"(.*)"$/, '$1');
    table[key] = parseTomlValue(line.slice(eq + 1).trim());
  }
  return root;
}

/** Drop a # comment, respecting quotes. */
function stripTomlComment(line) {
  let inStr = false, quote = '';
  for (let i = 0; i < line.length; i++) {
    const c = line[i];
    if (inStr) {
      if (c === '\\' && quote === '"') { i++; continue; }
      if (c === quote) inStr = false;
      continue;
    }
    if (c === '"' || c === "'") { inStr = true; quote = c; continue; }
    if (c === '#') return line.slice(0, i);
  }
  return line;
}

function parseTomlValue(v) {
  if (v === 'true') return true;
  if (v === 'false') return false;
  if (/^[+-]?\d+$/.test(v)) return Number(v);
  if (/^[+-]?\d*\.\d+$/.test(v)) return Number(v);
  if (v.startsWith('"') && v.endsWith('"') && v.length >= 2) {
    return v.slice(1, -1).replace(/\\"/g, '"').replace(/\\n/g, '\n').replace(/\\t/g, '\t').replace(/\\\\/g, '\\');
  }
  if (v.startsWith("'") && v.endsWith("'") && v.length >= 2) return v.slice(1, -1);
  if (v.startsWith('[') && v.endsWith(']')) {
    return v.slice(1, -1).split(',').map((s) => s.trim()).filter(Boolean).map(parseTomlValue);
  }
  return v;
}

/** Parse JSON that may contain VS Code style // comments and trailing commas. */
function parseLooseJson(text) {
  try { return JSON.parse(text); } catch { /* fall through */ }
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
  out = out.replace(/,(\s*[}\]])/g, '$1');
  return JSON.parse(out);
}

function readJsonIfExists(p) {
  try { return parseLooseJson(readFileSync(p, 'utf8')); } catch { return null; }
}

let cachedCfg = null;

function resolveConfig() {
  const env = process.env;
  const toml = existsSync(CONFIG_FILE) ? parseToml(readFileSync(CONFIG_FILE, 'utf8')) : {};
  const ws = readJsonIfExists(path.join(PROJECT_ROOT, '.vscode', 'settings.json')) || {};
  const wsConn = ws['objectscript.conn'] || {};
  const userSettings = readJsonIfExists(path.join(env.APPDATA || '', 'Code', 'User', 'settings.json')) || {};
  const serverKey = env.CACHE_V1_SERVER_NAME || wsConn.server || 'hfhis-817';
  const srv = userSettings['intersystems.servers']?.[serverKey];

  const pick = (envVal, tomlVal, wsVal, srvVal, dflt) => {
    if (envVal !== undefined && envVal !== '') return [envVal, 'env'];
    if (tomlVal !== undefined && tomlVal !== '') return [tomlVal, 'config'];
    if (wsVal !== undefined && wsVal !== '') return [wsVal, 'vscode:objectscript.conn'];
    if (srvVal !== undefined && srvVal !== '') return [srvVal, `vscode:intersystems.servers.${serverKey}`];
    return [dflt, 'default'];
  };

  const [host, hostSrc] = pick(env.CACHE_V1_HOST, toml.host, wsConn.host, srv?.webServer?.host, 'localhost');
  const [port, portSrc] = pick(env.CACHE_V1_PORT, toml.web_port, wsConn.port, srv?.webServer?.port, 57772);
  const [scheme, schemeSrc] = pick(env.CACHE_V1_SCHEME, toml.scheme, srv?.webServer?.scheme, undefined, 'http');
  const [username, userSrc] = pick(env.CACHE_V1_USERNAME, toml.username, wsConn.username, srv?.username, '_SYSTEM');
  const [password, passSrc] = pick(env.CACHE_V1_PASSWORD, toml.password, wsConn.password, srv?.password, 'SYS');
  const [namespace, nsSrc] = pick(env.CACHE_V1_NAMESPACE, toml.namespace, wsConn.ns, undefined, 'USER');

  return {
    host, port: Number(port), scheme, username, password, namespace, serverKey,
    writeToolsEnabled: toml.write_tools_enabled !== false,
    syncFlags: toml.sync?.flags || 'cuk',
    webRoots: Array.isArray(toml.sync?.web_roots) ? toml.sync.web_roots : [],
    source: { host: hostSrc, port: portSrc, scheme: schemeSrc, username: userSrc, password: passSrc, namespace: nsSrc },
    configFile: existsSync(CONFIG_FILE) ? CONFIG_FILE : null,
  };
}

function getConfig() {
  if (!cachedCfg) cachedCfg = resolveConfig();
  return cachedCfg;
}

const baseUrl = () => {
  const c = getConfig();
  return `${c.scheme}://${c.host}:${c.port}/api/atelier`;
};

// ---------------------------------------------------------------------------
// HTTP client: cookie jar + explicit logout + no redirect following
// ---------------------------------------------------------------------------
class AtelierError extends Error {
  constructor(message, { status, detail } = {}) {
    super(message);
    this.name = 'AtelierError';
    this.status = status;
    this.detail = detail;
  }
}

/**
 * CSP session cookies, keyed by name.
 *
 * The name is not fixed: Caché issues CSPSESSIONID-SP-<port>-UP-<webapp>- plus CSPWSERVERID,
 * so the port and web-app path are part of it. Keying by full name (rather than looking for
 * one expected name) survives that, and survives the name changing if the gateway moves.
 */
const cookies = new Map();
let sessionEstablished = false;
let loggedOut = false;

function cookieHeader() {
  return cookies.size ? [...cookies].map(([k, v]) => `${k}=${v}`).join('; ') : null;
}

/** Read Set-Cookie from a response. getSetCookie() exists on Node 20+; fall back below. */
function absorbCookies(res) {
  let list = [];
  if (typeof res.headers.getSetCookie === 'function') {
    list = res.headers.getSetCookie();
  } else {
    const combined = res.headers.get('set-cookie');
    // Split on commas that begin a new name=value pair, not on the comma inside Expires.
    if (combined) list = combined.split(/,(?=\s*[^;,=\s]+=)/);
  }
  for (const c of list) {
    const pair = c.split(';')[0];
    const i = pair.indexOf('=');
    if (i <= 0) continue;
    const name = pair.slice(0, i).trim();
    const value = pair.slice(i + 1).trim();
    if (/^CSPSESSIONID/i.test(name) || /^CSPWSERVERID/i.test(name)) cookies.set(name, value);
  }
  if (cookies.size) sessionEstablished = true;
}

function authHeader() {
  const c = getConfig();
  return 'Basic ' + Buffer.from(`${c.username}:${c.password}`).toString('base64');
}

/**
 * One Atelier call. Throws AtelierError on HTTP failure, on a transport error, and on a
 * response that is not JSON - the last one matters because a gateway answers expired
 * credentials with 302 to an HTML login page, and a followed redirect turns that into
 * "200 text/html", which would otherwise read as an empty success.
 */
async function irisFetch(method, urlPath, { body, contentType = 'application/json', timeoutMs = 120000 } = {}) {
  const cfg = getConfig();
  const url = `${baseUrl()}${urlPath}`;
  const headers = { Authorization: authHeader(), Accept: 'application/json' };
  const cookie = cookieHeader();
  if (cookie) headers.Cookie = cookie;
  if (body !== undefined) headers['Content-Type'] = contentType;

  const ctl = new AbortController();
  const timer = setTimeout(() => ctl.abort(), timeoutMs);
  let res, text;
  try {
    VERBOSE && log(`>> ${method} ${url}`);
    res = await fetch(url, { method, headers, body, signal: ctl.signal, redirect: 'manual' });
    absorbCookies(res);
    text = await res.text();
    VERBOSE && log(`<< ${res.status} (${text.length}b)`);
  } catch (e) {
    const code = e.cause?.code || e.name || '';
    throw new AtelierError(
      `Cannot reach Caché at ${cfg.scheme}://${cfg.host}:${cfg.port} (${code || e.message}). ` +
      `Check the host/port in ${cfg.configFile || '.cache-v1.toml'} and that the instance is running.`,
      { status: 0 },
    );
  } finally {
    clearTimeout(timer);
  }

  if (res.status >= 300 && res.status < 400) {
    throw new AtelierError(
      `HTTP ${res.status} redirect to ${res.headers.get('location') || '(no location)'}. ` +
      `A Caché gateway redirects to its login page when the credentials are rejected; ` +
      `check username/password for namespace ${cfg.namespace}.`,
      { status: res.status },
    );
  }

  const responseType = res.headers.get('content-type') || '';
  let json = null;
  if (text) {
    try { json = JSON.parse(text); } catch { /* handled below */ }
  }

  if (!res.ok) {
    const detail = extractErrors(json) || text?.slice(0, 400) || res.statusText;
    throw new AtelierError(`HTTP ${res.status}: ${detail}`, { status: res.status, detail: json });
  }

  if (json === null && text) {
    // 200 with a body that is not JSON: an HTML page (gateway error, login form) or a
    // truncated response. Never treat it as an empty result.
    const isHtml = /^\s*</.test(text) || /text\/html/i.test(responseType);
    throw new AtelierError(
      isHtml
        ? `The server returned an HTML page instead of JSON (${text.length} bytes). ` +
          `That is a CSP gateway error page or a login form, not an Atelier response. ` +
          `Heavy SQL and compile requests are the usual trigger on this instance.`
        : `The server returned ${text.length} bytes that are not JSON. First 200 characters: ${text.slice(0, 200)}`,
      { status: res.status },
    );
  }

  const errs = extractErrors(json);
  if (errs) throw new AtelierError(errs, { status: res.status, detail: json });
  return json;
}

function extractErrors(body) {
  const errs = body?.status?.errors;
  if (!Array.isArray(errs) || errs.length === 0) return null;
  return errs
    .map((e) => {
      // Caché localises these (this instance answers in German): keep the stable
      // error id/code and append the localised text so the message stays diagnosable.
      const code = e.code ? `#${e.code} ` : '';
      const text = e.error || e.summary || JSON.stringify(e);
      return `${code}${text}`;
    })
    .join('\n');
}

/**
 * End this server's CSP session.
 *
 * Caché honours CacheLogout; IRIS honours IRISLogout. Sending the wrong one is silent, and
 * the session then holds a license unit until it times out - which is what took this
 * instance's /api/* down before. 401 is the documented answer to a successful logout.
 */
async function logout() {
  if (loggedOut || !sessionEstablished) return;
  loggedOut = true;
  const cfg = getConfig();
  const headers = { Authorization: authHeader() };
  const cookie = cookieHeader();
  if (cookie) headers.Cookie = cookie;
  try {
    const res = await fetch(`${baseUrl()}/?CacheLogout=end`, {
      headers, redirect: 'manual', signal: AbortSignal.timeout(5000),
    });
    VERBOSE && log(`<< logout ${res.status}`);
  } catch { /* best effort: never let logout failure mask the real result */ }
  cookies.clear();
}

// ---------------------------------------------------------------------------
// API wrappers
// ---------------------------------------------------------------------------
/**
 * Encode a document name for the URL path.
 *
 * Both halves of this were measured against the instance, and both are surprising:
 *
 *   %Activate.Enum.cls   raw `%` -> 400 #16006 "invalid document name"
 *                        `%25`   -> 200, db CACHELIB
 *   dthealth/web/x.js    raw `/` -> 200/201, db @FS
 *                        `%2F`   -> 404 from the gateway
 *
 * So a `%` must be encoded and a `/` must not. Encoding the whole name breaks web documents;
 * encoding nothing breaks every system class. Encoding segment by segment satisfies both.
 */
function encodeDocName(name) {
  return String(name).split('/').map(encodeURIComponent).join('/');
}

const api = {
  root: () => irisFetch('GET', '/'),
  query: (ns, sql, params = []) =>
    irisFetch('POST', `/v1/${encodeURIComponent(ns)}/action/query`, {
      body: JSON.stringify({ query: sql, parameters: params }),
    }),
  getDoc: (ns, name) => irisFetch('GET', `/v1/${encodeURIComponent(ns)}/doc/${encodeDocName(name)}`),
  // ignoreConflict: without it, overwriting a document that already exists answers 409 and
  // echoes the current body back. v1 honours the same parameter v4 does.
  putDoc: (ns, name, lines) =>
    irisFetch('PUT', `/v1/${encodeURIComponent(ns)}/doc/${encodeDocName(name)}?ignoreConflict=1`, {
      body: JSON.stringify({ enc: false, content: lines }),
    }),
  deleteDoc: (ns, name) => irisFetch('DELETE', `/v1/${encodeURIComponent(ns)}/doc/${encodeDocName(name)}`),
  docnames: (ns, cat, type) => {
    let p = `/v1/${encodeURIComponent(ns)}/docnames`;
    if (cat && cat !== '*') p += '/' + encodeURIComponent(cat);
    if (type) p += '/' + encodeURIComponent(type);
    return irisFetch('GET', p, { timeoutMs: 180000 });
  },
  compile: (ns, targets, flags = 'cuk') =>
    irisFetch('POST', `/v1/${encodeURIComponent(ns)}/action/compile?flags=${encodeURIComponent(flags)}`, {
      body: JSON.stringify(targets),
    }),
};

// ---------------------------------------------------------------------------
// Exec helper bootstrap + execution
// ---------------------------------------------------------------------------
const helperReady = new Set();

/**
 * Compare two document bodies, ignoring line-ending and storage normalisation.
 *
 * Trailing whitespace is ignored on purpose: Caché rewrites an empty doc-comment line
 * (`///`) as `/// ` when it stores a class, so a byte-exact comparison never matches and
 * every check would re-push and re-compile the helper. Trailing whitespace in ObjectScript
 * source is never significant - a string literal cannot span lines.
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
 * Comparing bodies (not just existence) is what makes an edit here actually reach the
 * server: otherwise a namespace with any older version would keep running stale
 * ObjectScript while this file said otherwise.
 *
 * @returns {'present'|'installed'|'updated'}
 */
async function ensureHelper(ns, { force = false } = {}) {
  if (!force && helperReady.has(ns)) return 'present';
  const doc = await api.getDoc(ns, HELPER_DOC).catch((e) => {
    if (e.status === 404) return null;
    throw e;
  });
  const have = Array.isArray(doc?.result?.content) ? doc.result.content : null;
  if (!force && have && sameLines(have, HELPER_LINES)) {
    helperReady.add(ns);
    return 'present';
  }
  await api.putDoc(ns, HELPER_DOC, HELPER_LINES);
  // Compile through the helper's own Compile method once it exists, so the answer is a
  // %Status rather than localised console prose. On first install it cannot exist yet, so
  // fall back to the Atelier compile action and trust it when it reports no errors.
  const r = await api.compile(ns, [HELPER_DOC], 'cuk');
  const failed = (r?.result?.content || []).some((d) => d.status && String(d.status).includes('ERROR'));
  if (failed) throw new AtelierError(`Helper class ${HELPER_DOC} failed to compile`, { detail: r });
  helperReady.add(ns);
  return have ? 'updated' : 'installed';
}

/** Run ObjectScript, return { output, failed }. */
async function execObjectScript(ns, code) {
  await ensureHelper(ns);
  const r = await api.query(ns, `CALL ${HELPER_CALL}(?)`, [code]);
  const lines = Array.isArray(r?.console) ? r.console : [];
  while (lines.length && lines[0] === '') lines.shift();
  while (lines.length && lines[lines.length - 1] === '') lines.pop();
  const output = lines.join('\n');
  const failed = lines.some((l) => l.includes(EXEC_FAIL_MARKER));
  return { output, failed };
}

/** Compile documents through the helper and return its structured report. */
async function compileDocuments(ns, targets, flags) {
  await ensureHelper(ns);
  const r = await api.query(ns, `CALL ${HELPER_COMPILE_CALL}(?,?)`, [targets.join(','), flags]);
  const lines = Array.isArray(r?.console) ? r.console : [];
  while (lines.length && lines[0] === '') lines.shift();
  while (lines.length && lines[lines.length - 1] === '') lines.pop();
  const output = lines.join('\n');
  const failed = lines.some((l) => l.startsWith('ERROR '));
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

const textResult = (text, isError = false) => ({
  content: [{ type: 'text', text: cap(text) }],
  ...(isError ? { isError: true } : {}),
});

const NS_PARAM = {
  type: 'string',
  description: 'Caché namespace (default: the configured one, DHC-APP).',
};

// ---------------------------------------------------------------------------
// Tools
// ---------------------------------------------------------------------------
const TOOLS = [
  {
    name: 'cache_check_config',
    description:
      'Show the resolved Caché connection (host, port, namespace, username, and where each value came from), ' +
      'probe the server (Caché version, Atelier API version, enabled features), and check the exec helper. ' +
      'Run this first when anything fails.',
    inputSchema: { type: 'object', properties: {}, additionalProperties: false },
    handler: async () => {
      const cfg = getConfig();
      const lines = [
        'Configuration',
        `  config file : ${cfg.configFile || '(none — using .vscode/settings.json and defaults)'}`,
        `  server      : ${cfg.scheme}://${cfg.host}:${cfg.port}  [${cfg.source.host}]`,
        `  namespace   : ${cfg.namespace}  [${cfg.source.namespace}]`,
        `  username    : ${cfg.username}  [${cfg.source.username}]`,
        `  password    : ${'*'.repeat(String(cfg.password).length)}  [${cfg.source.password}]`,
        `  write tools : ${cfg.writeToolsEnabled ? 'enabled' : 'DISABLED (write_tools_enabled = false)'}`,
        '',
      ];

      let ok = true;
      try {
        const root = await api.root();
        const c = root?.result?.content || {};
        const feats = (c.features || []).map((f) => `${f.name}=${f.enabled}`).join(' ');
        lines.push('Server', `  version : ${c.version || '(unknown)'}`, `  api     : v${c.api ?? '?'}`);
        if (feats) lines.push(`  features: ${feats}`);
        if (Number(c.api) !== 1) {
          lines.push(`  WARNING : this server speaks Atelier v1 only; the instance reports v${c.api}.`);
          ok = false;
        }
        lines.push('  status  : OK');
      } catch (e) {
        lines.push('Server', `  status  : FAILED — ${e.message}`);
        ok = false;
      }

      lines.push('');
      try {
        const act = await ensureHelper(cfg.namespace);
        const probe = await execObjectScript(cfg.namespace, 'Write "helper-ok",!');
        const good = probe.output.includes('helper-ok');
        lines.push(
          'Exec helper',
          `  class   : ${HELPER_DOC} (${act})`,
          `  call    : CALL ${HELPER_CALL}(?)`,
          `  status  : ${good ? 'OK' : 'FAILED — no output captured in console'}`,
        );
        if (!good) ok = false;
      } catch (e) {
        lines.push('Exec helper', `  status  : FAILED — ${e.message}`);
        ok = false;
      }

      lines.push('', ok ? 'All checks passed.' : 'One or more checks FAILED — see above.');
      return textResult(lines.join('\n'), !ok);
    },
  },
  {
    name: 'cache_query',
    description:
      'Run SQL in the namespace and return the rows. This instance answers in German, so error text is ' +
      'localised; the stable error code is included. Heavy queries can make the CSP gateway return an HTML ' +
      'error page instead of JSON — keep queries bounded (TOP n, narrow WHERE).',
    inputSchema: {
      type: 'object',
      properties: {
        sql: { type: 'string', description: 'SQL statement. Use ? placeholders and pass params.' },
        params: { type: 'array', items: {}, description: 'Values for the ? placeholders, in order.' },
        namespace: NS_PARAM,
      },
      required: ['sql'],
      additionalProperties: false,
    },
    handler: async ({ sql, params = [], namespace }) => {
      const ns = namespace || getConfig().namespace;
      const r = await api.query(ns, sql, params);
      const rows = r?.result?.content ?? [];
      const consoleLines = Array.isArray(r?.console) ? r.console.filter((l) => l !== '') : [];
      let out = fmtRows(rows);
      if (consoleLines.length) out += `\n\nconsole:\n${consoleLines.join('\n')}`;
      return textResult(out);
    },
  },
  {
    name: 'cache_execute',
    description:
      'Execute ObjectScript in the namespace and return everything it wrote. Multi-line code is joined into ' +
      'the single line XECUTE needs; // and ; comments are stripped first, string literals respected. ' +
      'Use Write to produce output. Runtime errors come back as "*** EXECUTION FAILED ***: <error>".',
    inputSchema: {
      type: 'object',
      properties: {
        code: { type: 'string', description: 'ObjectScript to execute.' },
        namespace: NS_PARAM,
      },
      required: ['code'],
      additionalProperties: false,
    },
    handler: async ({ code, namespace }) => {
      const ns = namespace || getConfig().namespace;
      const { output, failed } = await execObjectScript(ns, code);
      return textResult(output || '(no output)', failed);
    },
  },
  {
    name: 'cache_execute_method',
    description:
      'Call a class method and return its return value. Arguments are passed as ObjectScript string literals ' +
      'unless raw is true, in which case each argument is spliced in as an ObjectScript expression.',
    inputSchema: {
      type: 'object',
      properties: {
        className: { type: 'string', description: 'Class name, e.g. web.DHCBL.CT.CTLoc.' },
        methodName: { type: 'string', description: 'Class method name.' },
        args: { type: 'array', items: { type: 'string' }, description: 'Arguments, in order.' },
        raw: { type: 'boolean', description: 'Treat each argument as an ObjectScript expression, not a string literal.' },
        namespace: NS_PARAM,
      },
      required: ['className', 'methodName'],
      additionalProperties: false,
    },
    handler: async ({ className, methodName, args = [], raw = false, namespace }) => {
      const ns = namespace || getConfig().namespace;
      const argList = args.map((a) => (raw ? String(a) : cosQ(a))).join(',');
      const code = [
        `Set tR = $ClassMethod(${cosQ(className)}, ${cosQ(methodName)}${argList ? ',' + argList : ''})`,
        'Write $Get(tR), !',
      ].join('\n');
      const { output, failed } = await execObjectScript(ns, code);
      return textResult(`${className}.${methodName} returned:\n${output || '(empty)'}`, failed);
    },
  },
  {
    name: 'cache_doc',
    description:
      'Read, write or delete one server document (class .cls, routine .mac/.int/.inc, or a CSP/other file). ' +
      'Writing saves the source but does not compile it — call cache_compile next.',
    inputSchema: {
      type: 'object',
      properties: {
        action: { type: 'string', enum: ['get', 'put', 'delete'], description: 'Operation to perform.' },
        name: { type: 'string', description: 'Document name, e.g. web.DHCBL.CT.CTLoc.cls or Foo.mac.' },
        content: {
          type: 'string',
          description: 'For put: the full new source. Sent as lines; CRLF is normalised.',
        },
        namespace: NS_PARAM,
      },
      required: ['action', 'name'],
      additionalProperties: false,
    },
    handler: async ({ action, name, content, namespace }) => {
      const ns = namespace || getConfig().namespace;
      if (action === 'get') {
        const r = await api.getDoc(ns, name);
        const lines = r?.result?.content;
        if (!Array.isArray(lines)) return textResult(`(no content for ${name})`, true);
        return textResult(lines.join('\n'));
      }
      if (action === 'put') {
        if (typeof content !== 'string') return textResult('put requires content (a string).', true);
        const lines = content.replace(/\r\n/g, '\n').split('\n');
        if (lines.length && lines[lines.length - 1] === '') lines.pop();
        await api.putDoc(ns, name, lines);
        return textResult(`Saved ${name} (${lines.length} lines) in ${ns}. It is NOT compiled yet — call cache_compile.`);
      }
      if (action === 'delete') {
        await api.deleteDoc(ns, name);
        return textResult(`Deleted ${name} from ${ns}.`);
      }
      return textResult(`Unknown action: ${action}`, true);
    },
  },
  {
    name: 'cache_docnames',
    description:
      'List document names in the namespace. There is no server-side code search on Atelier v1 (the ' +
      '/action/search route returns 404 on this build), so this is how you find things by name: fetch the ' +
      'category once and filter with pattern. Use category CLS for classes, RTN with type mac/int/inc for ' +
      'routines, CSP or OTH for web and other files.',
    inputSchema: {
      type: 'object',
      properties: {
        category: {
          type: 'string',
          enum: ['CLS', 'RTN', 'CSP', 'OTH', '*'],
          description: 'Document category (default CLS).',
        },
        type: { type: 'string', enum: ['mac', 'int', 'inc'], description: 'Routine type; only meaningful with category RTN.' },
        pattern: {
          type: 'string',
          description: 'Case-insensitive filter applied to the names after fetching (substring, or a regex when regex is true).',
        },
        regex: { type: 'boolean', description: 'Treat pattern as a regular expression.' },
        limit: { type: 'integer', description: 'Maximum names to return (default 200).' },
        namespace: NS_PARAM,
      },
      additionalProperties: false,
    },
    handler: async ({ category = 'CLS', type, pattern, regex = false, limit = 200, namespace }) => {
      const ns = namespace || getConfig().namespace;
      const r = await api.docnames(ns, category, type);
      const all = (r?.result?.content || []).map((d) => (typeof d === 'string' ? d : d.name)).filter(Boolean);
      let names = all;
      if (pattern) {
        const test = regex
          ? (() => {
              let re;
              try { re = new RegExp(pattern, 'i'); } catch (e) { throw new AtelierError(`Invalid regex: ${e.message}`); }
              return (n) => re.test(n);
            })()
          : (n) => n.toLowerCase().includes(String(pattern).toLowerCase());
        names = all.filter(test);
      }
      const shown = names.slice(0, Math.min(limit, MAX_DOCNAMES));
      let out = `${shown.join('\n') || '(none)'}`;
      out += `\n\n${names.length} match(es)${pattern ? ` for ${JSON.stringify(pattern)}` : ''} of ${all.length} in ${category}${type ? '/' + type : ''}.`;
      if (names.length > shown.length) out += ` Showing the first ${shown.length}; raise limit to see more.`;
      return textResult(out);
    },
  },
  {
    name: 'cache_compile',
    description:
      'Compile documents and report per-name status. Goes through $System.OBJ.Compile inside the helper, so the ' +
      'result is a %Status rather than this instance\'s localised console prose. Each line starts with OK or ERROR.',
    inputSchema: {
      type: 'object',
      properties: {
        targets: { type: 'array', items: { type: 'string' }, description: 'Document names, e.g. ["web.DHCBL.CT.CTLoc.cls"].' },
        flags: { type: 'string', description: 'Compile qualifiers (default cuk).' },
        namespace: NS_PARAM,
      },
      required: ['targets'],
      additionalProperties: false,
    },
    handler: async ({ targets, flags, namespace }) => {
      const ns = namespace || getConfig().namespace;
      const f = flags || getConfig().syncFlags;
      if (!Array.isArray(targets) || targets.length === 0) return textResult('targets must be a non-empty array.', true);
      const { output, failed } = await compileDocuments(ns, targets, f);
      return textResult(output || '(no output)', failed);
    },
  },
  {
    name: 'cache_global',
    description:
      'Read, write, kill or list globals. The global argument takes subscripts: ^CacheTemp.Foo("a",1). ' +
      'set and kill are destructive and require confirm:true.',
    inputSchema: {
      type: 'object',
      properties: {
        action: { type: 'string', enum: ['get', 'set', 'kill', 'order'], description: 'Operation.' },
        global: { type: 'string', description: 'Global reference with optional subscripts, e.g. ^Foo("a",1).' },
        value: { type: 'string', description: 'For set: the value to assign.' },
        limit: { type: 'integer', description: 'For order: how many nodes to list (default 50).' },
        confirm: { type: 'boolean', description: 'Required true for set and kill.' },
        namespace: NS_PARAM,
      },
      required: ['action', 'global'],
      additionalProperties: false,
    },
    handler: async ({ action, global: gbl, value, limit = 50, confirm, namespace }) => {
      const ns = namespace || getConfig().namespace;
      if (!/^\^[A-Za-z%][A-Za-z0-9%]*/.test(gbl)) {
        return textResult('global must look like ^Name or ^Name(sub1,sub2).', true);
      }
      if (action === 'get') {
        const { output, failed } = await execObjectScript(ns, `ZWrite ${gbl}`);
        return textResult(output || '(undefined)', failed);
      }
      if (action === 'order') {
        const code = [
          // The global name goes in as a string: `Set tRef = ^Foo` would evaluate the
          // global and raise <UNDEFINED> when it does not exist yet.
          `Set tRef = ${cosQ(gbl)}`,
          'Set tN = 0',
          'Set tSub = ""',
          'For {',
          '  Set tSub = $Order(@tRef@(tSub))',
          '  Quit:(tSub = "")',
          `  Quit:(tN >= ${Number(limit)})`,
          '  Set tN = tN + 1',
          '  Write tSub, " = ", $Get(@tRef@(tSub)), !',
          '}',
          'Write "[", tN, " node(s)]", !',
        ].join('\n');
        const { output, failed } = await execObjectScript(ns, code);
        return textResult(output || '(no nodes)', failed);
      }
      if (action === 'set' || action === 'kill') {
        if (confirm !== true) {
          return textResult(
            `${action} on ${gbl} is destructive. Re-run with confirm:true to proceed.`,
            true,
          );
        }
        const code = action === 'set' ? `Set ${gbl} = ${cosQ(value ?? '')}` : `Kill ${gbl}`;
        const { output, failed } = await execObjectScript(ns, `${code} Write "done",!`);
        return textResult(`${action} ${gbl}: ${output || 'done'}`, failed);
      }
      return textResult(`Unknown action: ${action}`, true);
    },
  },
  {
    name: 'cache_info',
    description:
      'Report the Caché version, Atelier API level, enabled features, the current namespace and its database ' +
      'directories, and which capabilities this build supports.',
    inputSchema: {
      type: 'object',
      properties: { namespace: NS_PARAM },
      additionalProperties: false,
    },
    handler: async ({ namespace }) => {
      const ns = namespace || getConfig().namespace;
      const root = await api.root();
      const c = root?.result?.content || {};
      const lines = [
        `Caché version : ${c.version || '(unknown)'}`,
        `Atelier API   : v${c.api ?? '?'}`,
        `Features      : ${(c.features || []).map((f) => `${f.name}=${f.enabled}`).join(', ') || '(none reported)'}`,
        `Namespace     : ${ns}`,
      ];
      const probe = await execObjectScript(
        ns,
        [
          'Write "namespace: ", $Namespace, !',
          'Write "routine db: ", ##class(%SYS.Namespace).GetRoutineDest($Namespace), !',
          'Write "globals db: ", ##class(%SYS.Namespace).GetGlobalDest($Namespace), !',
        ].join('\n'),
      );
      if (probe.output) lines.push('', probe.output);
      lines.push(
        '',
        'Supported by this server (verified on 2016.2.3):',
        '  SQL query, ObjectScript exec, class method call, document get/put/delete,',
        '  docnames listing, compile, globals, table info, storage resolution, file sync.',
        'Not available on Atelier v1:',
        '  /action/search (404), /action/execute (absent), $preprocess (absent),',
        '  %Dictionary.*.Implementation (empty). There is no code-body search.',
      );
      return textResult(lines.join('\n'));
    },
  },
  {
    name: 'cache_table_info',
    description:
      'Describe the SQL table behind a class: the SQL table name, its properties (with SQL field names) and its ' +
      'indices. Start here before writing SQL — in this namespace 5,842 persistent classes have a SQL table ' +
      'name that differs from the class name, so the class name usually will not work in a FROM clause.',
    inputSchema: {
      type: 'object',
      properties: {
        className: { type: 'string', description: 'Class name, e.g. User.CTLoc.' },
        namespace: NS_PARAM,
      },
      required: ['className'],
      additionalProperties: false,
    },
    handler: async ({ className, namespace }) => {
      const ns = namespace || getConfig().namespace;
      const cls = await api.query(
        ns,
        'SELECT Name, SqlTableName, SqlSchemaName, ClassType, Abstract FROM %Dictionary.CompiledClass WHERE Name = ?',
        [className],
      );
      const props = await api.query(
        ns,
        'SELECT Name, Type, SqlFieldName FROM %Dictionary.PropertyDefinition WHERE Parent = ?',
        [className],
      );
      const idx = await api.query(
        ns,
        'SELECT Name, Type, Properties FROM %Dictionary.IndexDefinition WHERE Parent = ?',
        [className],
      );
      const c = cls?.result?.content?.[0];
      const p = props?.result?.content ?? [];
      const i = idx?.result?.content ?? [];
      if (p.length === 0 && i.length === 0) {
        return textResult(
          `${className} has no properties and no indices, so it is not a SQL table. Business-logic classes in ` +
          `this codebase usually name their real table in a method comment (look for "Table：Xxx.Yyy") — ` +
          `try cache_table_info on that class instead.`,
        );
      }

      // The class name and the SQL table name are not the same thing, and here they usually
      // differ: User.CTLoc is queried as CT_Loc. Reporting only the class name sent people
      // into SQLCODE -30 ("table not found") on a class that plainly exists.
      const table = c?.SqlTableName || className.split('.').pop();
      const schema = c?.SqlSchemaName || '';
      const qualified = schema ? `${schema}.${table}` : table;
      const lines = [`${className}`, `  SQL table : ${qualified}`];
      if (table !== className.split('.').pop()) {
        lines.push(`  note      : the table name differs from the class name — use ${qualified} in FROM.`);
      }
      lines.push(`  kind      : ${c?.ClassType || '(not persistent)'}${c?.Abstract ? ', abstract' : ''}`);
      lines.push(`  ${p.length} property(ies), ${i.length} index(es)`, '', 'Properties:');
      if (p.length === 0) lines.push('  (none)');
      for (const r of p) {
        lines.push(`  ${r.Name}  type=${r.Type ?? '?'}  sqlField=${r.SqlFieldName ?? r.Name}`);
      }
      lines.push('', 'Indices:');
      if (i.length === 0) lines.push('  (none)');
      for (const r of i) {
        lines.push(`  ${r.Name}  type=${r.Type ?? '?'}  on=${r.Properties ?? '?'}`);
      }
      return textResult(lines.join('\n'));
    },
  },
  {
    name: 'cache_resolve_storage',
    description:
      'Map a class to the globals that hold its data. Reads the class source and reports the Storage block: ' +
      'DataLocation / IdLocation / StreamLocation for a native persistent class, or the SQL map Global for a ' +
      '%CacheSQLStorage class. Use this before inspecting data with cache_global.',
    inputSchema: {
      type: 'object',
      properties: {
        className: { type: 'string', description: 'Class name, e.g. User.CTLoc.' },
        namespace: NS_PARAM,
      },
      required: ['className'],
      additionalProperties: false,
    },
    handler: async ({ className, namespace }) => {
      const ns = namespace || getConfig().namespace;
      // Parsed from the source rather than from %Dictionary.StorageDefinition: on 2016.2.3
      // that object's DataLocation is a multidimensional property that cannot be read from
      // inside XECUTE, and the SQL projection has no DataLocation column at all. The source
      // carries the same information in a form the developer already reads in Studio.
      const r = await api.getDoc(ns, `${className}.cls`);
      const src = r?.result?.content;
      if (!Array.isArray(src)) return textResult(`Could not read ${className}.cls`, true);

      const tag = (name) => {
        const out = [];
        const re = new RegExp(`<${name}>([^<]*)</${name}>`, 'g');
        for (const m of src.join('\n').matchAll(re)) if (m[1].trim()) out.push(m[1].trim());
        return [...new Set(out)];
      };
      const dataLoc = tag('DataLocation');
      const idLoc = tag('IdLocation');
      const streamLoc = tag('StreamLocation');
      const globals = tag('Global');
      const storages = src
        .map((l) => l.trim())
        .filter((l) => /^Storage\s+\S+/.test(l))
        .map((l) => l.replace(/\s*\{$/, ''));

      if (!storages.length) {
        return textResult(
          `${className} has no Storage block. It is not a persistent or SQL-mapped class — most likely a ` +
          `business-logic class whose table lives elsewhere (check a comment or a property for the real table).`,
        );
      }

      const lines = [`${className}`, `  storages: ${storages.join(', ')}`];
      // The SQL table name is worth stating here too: it is what cache_query needs, and it
      // frequently differs from the class name.
      const cls = await api.query(
        ns,
        'SELECT SqlTableName, SqlSchemaName FROM %Dictionary.CompiledClass WHERE Name = ?',
        [className],
      );
      const c = cls?.result?.content?.[0];
      if (c?.SqlTableName) {
        const qualified = c.SqlSchemaName ? `${c.SqlSchemaName}.${c.SqlTableName}` : c.SqlTableName;
        lines.splice(1, 0, `  SQL table: ${qualified}`);
      }
      if (dataLoc.length) lines.push(`  data   : ${dataLoc.join(', ')}`);
      if (idLoc.length) lines.push(`  id     : ${idLoc.join(', ')}`);
      if (streamLoc.length) lines.push(`  stream : ${streamLoc.join(', ')}`);
      if (globals.length) lines.push(`  sql map: ${globals.join(', ')}`);
      if (!dataLoc.length && !globals.length) {
        lines.push('  (no global mapping found in the Storage block — this class may inherit its storage)');
      }
      return textResult(lines.join('\n'));
    },
  },
  {
    name: 'cache_sync',
    description:
      'Push local source files to the server and compile the code ones. Classes are named from their Class ' +
      'declaration (src/ABN/Foo.cls -> ABN.Foo.cls), routines by file name, and web assets by path ' +
      '(src/dthealth/web/x.js -> dthealth/web/x.js). Use this after editing a file instead of pushing by hand.',
    inputSchema: {
      type: 'object',
      properties: {
        files: { type: 'array', items: { type: 'string' }, description: 'Absolute or project-relative file paths.' },
        compile: { type: 'boolean', description: 'Compile after pushing (default true).' },
        namespace: NS_PARAM,
      },
      additionalProperties: false,
    },
    handler: async ({ files = [], compile = true, namespace }) => {
      const ns = namespace || getConfig().namespace;
      if (files.length === 0) {
        return textResult(
          'No files given. Pass the files to sync, or use the --sync CLI mode, which reads them from the ' +
          'editor hook payload.',
          true,
        );
      }
      const r = await syncFiles(ns, files, compile);
      return textResult(r.text, r.compileFailed);
    },
  },
];

// ---------------------------------------------------------------------------
// File sync
// ---------------------------------------------------------------------------
const SYNC_CODE_EXTENSIONS = new Set(['.cls', '.mac', '.int', '.inc']);
// Extensions that are web assets rather than ObjectScript. Kept as an allow-list so a stray
// .md or .json under src/ is not mistaken for a page and pushed over a real one.
const SYNC_WEB_EXTENSIONS = new Set([
  '.js', '.css', '.csp', '.html', '.htm', '.xml', '.json', '.txt', '.map',
  '.png', '.gif', '.jpg', '.jpeg', '.ico', '.svg', '.bmp',
  '.woff', '.woff2', '.ttf', '.eot', '.otf',
]);

/**
 * Map a local file to the server document name it corresponds to.
 *
 * The basename alone is not enough, in either direction, and both cases occur here.
 *
 * Classes live under src/ in a tree that mirrors the package, but the same basename appears
 * in several packages - there are three different CTLoc.cls files - so the name comes from
 * the file's own `Class Pkg.Name` declaration. That is what the compiler goes by, and it
 * stays right wherever the file sits.
 *
 * Routines carry their name on the first line as `ROUTINE <name>`, and it can be dotted and
 * can disagree with the file name: 17 routines here have dotted names, and files such as
 * BalanceDetails.mac declare `ROUTINE DHCWL.Account.BalanceDetails`. Pushing by basename
 * would create a new stray routine instead of updating the real one.
 */
function docNameFor(file, text = '') {
  const base = path.basename(file);
  const ext = path.extname(base).toLowerCase();
  if (!SYNC_CODE_EXTENSIONS.has(ext)) return null;

  if (ext === '.cls') {
    // Anchored at the start of a line so a mention inside a string or a comment cannot match.
    const declared = text.match(/^Class\s+([%\w][\w.]*)/m);
    if (declared) return `${declared[1]}.cls`;
    const rel = path.relative(PROJECT_ROOT, file).replace(/\\/g, '/');
    const noSrc = rel.replace(/^src\//, '').replace(/\.cls$/i, '');
    return `${noSrc.replace(/\//g, '.')}.cls`;
  }

  // Routines: `ROUTINE name` on line 1, optionally followed by `[Type=INC]`.
  const routine = text.match(/^ROUTINE\s+([%\w][\w.]*)/m);
  if (routine) return `${routine[1]}${ext}`;
  return base;
}

/**
 * Where a local file lands on the server.
 *
 * Two different naming schemes exist and they were measured, not guessed:
 *
 *   code  .cls            -> Pkg.Name.cls        (dotted, from the Class declaration)
 *         .mac/.int/.inc  -> FileName.mac        (routines are addressed by file name)
 *   web   .js/.css/.csp/… -> dthealth/web/x.js   (path form, no leading slash)
 *
 * The web form is real: `PUT /doc/dthealth/web/probe.js` answers 201 with db "@FS" and reads
 * back, and a path-form .csp does the same. A bare `CacheV1Probe.csp` is refused with
 * #16006 "invalid document name", and a leading slash 404s, so the path must be relative and
 * unescaped. Every one of this project's 48,443 web assets sits under src/dthealth/web, so
 * stripping the "src/" prefix is enough to reproduce the server path.
 *
 * Returns { docName, kind } or null when the file is not something this server can push.
 */
function targetFor(abs, text = '') {
  const ext = path.extname(abs).toLowerCase();
  const rel = path.relative(PROJECT_ROOT, abs).replace(/\\/g, '/');

  if (SYNC_CODE_EXTENSIONS.has(ext)) {
    return { docName: docNameFor(abs, text), kind: 'code' };
  }

  // Web assets: map a configured root, else fall back to "strip src/".
  for (const root of getConfig().webRoots) {
    const local = String(root.local || '').replace(/\\/g, '/').replace(/\/$/, '');
    const server = String(root.server || '').replace(/^\//, '').replace(/\/$/, '');
    if (local && rel.startsWith(local + '/')) {
      return { docName: `${server}/${rel.slice(local.length + 1)}`, kind: 'web' };
    }
  }
  if (rel.startsWith('src/') && !SYNC_CODE_EXTENSIONS.has(ext)) {
    // Only treat it as a web asset if it looks like one; a stray .md under src/ is not.
    if (SYNC_WEB_EXTENSIONS.has(ext)) return { docName: rel.replace(/^src\//, ''), kind: 'web' };
  }
  return null;
}

/**
 * Push files to the server, then compile the code ones.
 *
 * Web assets are pushed but not compiled - there is nothing to compile - which is why the
 * two kinds are tracked apart.
 *
 * Returns the parts separately as well as the rendered text, because hook mode must report
 * compile errors and nothing else: a hook that spoke up for every .md edit would be noise.
 */
async function syncFiles(ns, files, compile = true) {
  const pushed = [];
  const compiled = [];
  const skipped = [];
  for (const f of files) {
    const abs = path.isAbsolute(f) ? f : path.join(PROJECT_ROOT, f);
    if (!existsSync(abs) || !statSync(abs).isFile()) { skipped.push(`${f} (not found)`); continue; }
    const text = readFileSync(abs, 'utf8').replace(/\r\n/g, '\n');
    const target = targetFor(abs, text);
    if (!target) { skipped.push(`${path.basename(abs)} (not a document this server can push)`); continue; }
    const lines = text.split('\n');
    if (lines.length && lines[lines.length - 1] === '') lines.pop();
    try {
      await api.putDoc(ns, target.docName, lines);
      pushed.push(target.docName);
      if (target.kind === 'code') compiled.push(target.docName);
    } catch (e) {
      skipped.push(`${target.docName} (${e.message})`);
    }
  }

  const lines = [];
  if (pushed.length) {
    lines.push(`Pushed ${pushed.length} document(s) to ${ns}:`, ...pushed.map((d) => `  ${d}`));
  }
  if (skipped.length) {
    lines.push(`${skipped.length} skipped:`, ...skipped.map((d) => `  ${d}`));
  }

  let compileOutput = '';
  let compileFailed = false;
  if (compiled.length && compile) {
    const r = await compileDocuments(ns, compiled, getConfig().syncFlags);
    compileOutput = r.output;
    compileFailed = r.failed;
    lines.push('', 'Compile:', compileOutput || '(no output)');
    if (compileFailed) lines.push('', 'Compile reported errors — fix them before relying on this code.');
  }

  return { text: lines.join('\n') || 'Nothing to sync.', pushed, skipped, compileOutput, compileFailed };
}

/**
 * Read the editor hook payload from stdin and return the file paths it names.
 *
 * Two shapes arrive here: Claude Code sends JSON with tool_input.file_path, and the Codex
 * hook script sends an apply_patch command whose "*** Update File:" markers name the files.
 */
function filesFromHookPayload(raw) {
  if (!raw || !raw.trim()) return [];
  let evt = null;
  try { evt = JSON.parse(raw); } catch { return []; }
  const out = new Set();
  const fp = evt?.tool_input?.file_path || evt?.tool_input?.path;
  if (typeof fp === 'string') out.add(fp);
  const cmd = evt?.tool_input?.command;
  if (typeof cmd === 'string') {
    for (const m of cmd.matchAll(/^\*\*\* (?:Add File|Update File|Move to): (.+?)\r?$/gm)) {
      out.add(m[1].trim());
    }
  }
  return [...out];
}

// ---------------------------------------------------------------------------
// MCP stdio server
// ---------------------------------------------------------------------------
const SERVER_INFO = { name: 'cache-v1', version: '1.0.0' };

const rpcResult = (id, result) => process.stdout.write(JSON.stringify({ jsonrpc: '2.0', id, result }) + '\n');
const rpcError = (id, code, message) => process.stdout.write(JSON.stringify({ jsonrpc: '2.0', id, error: { code, message } }) + '\n');

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
          instructions:
            'Caché 2016.2.3 over Atelier v1. Start with cache_check_config if anything fails. ' +
            'There is no code-body search on this API version: use cache_docnames to find names. ' +
            'cache_doc put does not compile — call cache_compile afterwards.',
        });
        return;
      case 'notifications/initialized':
      case 'notifications/cancelled':
      case 'notifications/progress':
        return;
      case 'ping':
        rpcResult(id, {});
        return;
      case 'tools/list':
        rpcResult(id, { tools: TOOLS.map(({ name, description, inputSchema }) => ({ name, description, inputSchema })) });
        return;
      case 'tools/call': {
        const name = params?.name;
        const tool = TOOLS.find((t) => t.name === name);
        if (!tool) {
          rpcResult(id, {
            content: [{ type: 'text', text: `Unknown tool: ${name}. Available: ${TOOLS.map((t) => t.name).join(', ')}` }],
            isError: true,
          });
          return;
        }
        let out;
        try {
          out = await tool.handler(params?.arguments || {});
        } catch (e) {
          const text = e instanceof AtelierError ? `Caché error: ${e.message}` : `${e.constructor.name}: ${e.message}`;
          out = { content: [{ type: 'text', text }], isError: true };
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
  const out = await TOOLS[0].handler({});
  const text = out.content[0].text;
  console.log(text);
  // Never process.exit(): undici keep-alive sockets are still closing and exit() trips a
  // libuv assertion on Windows. Set the code and let the loop drain.
  process.exitCode = text.includes('FAILED') ? 1 : 0;
}

async function cliInstallHelper() {
  const ns = getConfig().namespace;
  // force: --install-helper is the repair command, so it must push even when the body
  // already matches (a class can look identical and still be broken after a failed compile).
  const act = await ensureHelper(ns, { force: true });
  const probe = await execObjectScript(ns, 'Write "helper-ok",!');
  console.log(`${HELPER_DOC} ${act} in ${ns} (CALL ${HELPER_CALL}(?)).`);
  console.log(`Self-test: ${probe.output.includes('helper-ok') ? 'OK' : 'FAILED'}`);
  if (!probe.output.includes('helper-ok')) process.exitCode = 1;
}

async function cliSync() {
  const ns = getConfig().namespace;
  let files = process.argv.slice(2).filter((a) => !a.startsWith('--'));
  if (process.argv.includes('--hook')) {
    const chunks = [];
    for await (const chunk of process.stdin) chunks.push(chunk);
    const fromHook = filesFromHookPayload(Buffer.concat(chunks).toString('utf8'));
    files = files.concat(fromHook);
  }
  if (files.length === 0) {
    // A hook that fires on a file this server cannot push must stay silent: printing on
    // every edit would be noise in the editor's transcript.
    if (process.argv.includes('--hook')) return;
    console.log('No files to sync.');
    return;
  }
  const result = await syncFiles(ns, files, true);
  if (process.argv.includes('--hook')) {
    // A hook runs after every edit, so it must speak only when something went wrong. A file
    // the server cannot push is not a problem — it is most edits. A compile error is.
    if (result.compileFailed) {
      log(`[cache-v1] ${result.pushed.join(', ')}\n${result.compileOutput}`);
      process.exitCode = 1;
    }
    return;
  }
  console.log(result.text);
}

if (process.argv.includes('--check')) {
  cliCheck()
    .then(() => logout())
    .catch((e) => { console.error(String(e)); process.exitCode = 1; return logout(); });
} else if (process.argv.includes('--install-helper')) {
  cliInstallHelper()
    .then(() => logout())
    .catch((e) => { console.error(String(e)); process.exitCode = 1; return logout(); });
} else if (process.argv.includes('--sync')) {
  cliSync()
    .then(() => logout())
    .catch((e) => { console.error(String(e)); process.exitCode = 1; return logout(); });
} else if (process.argv.includes('--logout')) {
  logout().then(() => console.log('CSP session ended.'));
} else {
  const rl = readline.createInterface({ input: process.stdin, terminal: false });
  rl.on('line', (line) => {
    const s = line.trim();
    if (!s) return;
    let msg;
    try { msg = JSON.parse(s); } catch { log('[cache-v1] non-JSON line ignored'); return; }
    handleMessage(msg).catch((e) => log('[cache-v1] handler error:', e.message));
  });
  rl.on('close', () => { logout().then(() => { process.exitCode = 0; }); });
  for (const sig of ['SIGINT', 'SIGTERM']) {
    process.on(sig, () => { logout().then(() => { process.exitCode = 0; }); });
  }
  VERBOSE && log(`[cache-v1] stdio server up (${SERVER_INFO.name} ${SERVER_INFO.version})`);
}
