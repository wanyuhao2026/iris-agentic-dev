# iris-v4-mcp

MCP server connecting AI coding agents (Claude Code, DeepSeek Harness, …) to the project's local
InterSystems IRIS / HealthConnect **2020.1.1** instance (WSL) over the **Atelier REST API v4** —
the newest API that server has.

## Why this exists

The official [`iris-agentic-dev`](https://github.com/intersystems-community/iris-agentic-dev) MCP
requires IRIS 2023.1+ and calls `/api/atelier/v8/...` endpoints. The WSL instance reports
`"api": 4` (v8 endpoints return 404), so the official server cannot connect. This server
re-implements the core of its tool surface on v4 only. Zero npm dependencies; Node >= 18.

## Tools

| Tool | What it does | v4 mechanism |
|---|---|---|
| `check_config` | Connection diagnostics | `GET /api/atelier/` + helper check |
| `iris_query` | SQL (rows as JSON) | `POST /{ns}/action/query` |
| `iris_execute` | Run ObjectScript, capture Write output | SqlProc helper (below) |
| `iris_doc` | get / put / delete a class or routine | `GET/PUT/DELETE /{ns}/doc/{name}` |
| `iris_docnames` | List docs (CLS/RTN/CSP/OTH), SQL-LIKE filter | `GET /{ns}/docnames/{cat}` |
| `iris_compile` | Compile documents | `POST /{ns}/action/compile` |
| `iris_search` | Server-side content search | `GET /{ns}/action/search` |
| `iris_production` | status / items / event_log / messages / start / stop / restart | exec helper + SQL |

## ObjectScript execution (the v4 gap)

Atelier v4 has no code-execution route (that arrived with later API versions). `iris_execute`
works through a one-method SqlProc helper class, `JHIP.MCP.Run` (repo copy:
[src/JHIP/MCP/Run.cls](../src/JHIP/MCP/Run.cls)):

```
CALL JHIP_MCP.Run_Exec(?)          -- ? = the ObjectScript code
```

The Atelier query action captures everything the code `Write`s and returns it in the response
`console` array, which is what the tool shows.

The server keeps the helper in sync by **comparing its body**, not by asking whether the
document exists: on each use it fetches `JHIP.MCP.Run.cls` and pushes + compiles only when the
source differs, so editing [src/JHIP/MCP/Run.cls](../src/JHIP/MCP/Run.cls) actually reaches the
server. (Trailing whitespace is ignored in that comparison — IRIS stores an empty `///` line as
`/// `, which made a byte-exact check re-push and re-compile on every call.) Keep the repo `.cls`
and the `HELPER_SOURCE` in the `.mjs` in sync when you edit it; `--install-helper` forces the push
when you want a repair regardless of the comparison.

### Multi-line code

`XECUTE` takes one or more comma-separated arguments, each executed as a line — a newline inside
a single argument is a `<SYNTAX>` error, which is why handing raw code straight to `XECUTE` only
ever worked for one-liners. `iris_execute` therefore runs your code through `Flatten` first, which
joins the lines with spaces (a newline is only a statement separator in ObjectScript, so this is
semantically equivalent) after:

- stripping `//` and `;` comments — otherwise the join would comment out the rest of the code;
- dropping blank lines and `#;` comments;
- leaving string literals alone, so `"http://x"`, `"a;b"` and `"he said ""hi"" // x"` survive.

Multi-line blocks, `Try { ... } Catch { ... }`, indented bodies and lines ending in `_` all work.

### Known quirks in the exec context

- Use `$ZNSPACE` to read the namespace — `$ZNAMESPACE` and `$ZLEVEL` throw `<SYNTAX>` here.
- No interactive `Read`; avoid `HALT` (it would kill the worker process).
- Errors are written as `*** EXECUTION FAILED ***: ...` and mark the tool result as failed.
- The code runs through `XECUTE`, so an error's `+n^JHIP.MCP.Run.1` line points into the
  **generated** routine, not into your snippet — it locates the failure in the helper, not in
  the code you sent.
- **`$$$` macros do not work** — `XECUTE` runs no preprocessor, so `$$$ISOK(sc)`, `$$$ISERR(sc)`
  and `$$$OK` are `<SYNTAX>` errors even on a single line, and `#include` is unavailable. Use the
  runtime equivalents: `$System.Status.IsOK(sc)` / `$System.Status.IsError(sc)` / `1`. (The
  [AGENTS.md](../AGENTS.md) snippets are written for compiled class methods, where the macros are
  the right form — translate them when pasting into `iris_execute`.)

## Connection resolution

1. Env vars: `IRIS_HOST`, `IRIS_WEB_PORT`, `IRIS_SCHEME`, `IRIS_USERNAME`, `IRIS_PASSWORD`, `IRIS_NAMESPACE`
2. Workspace [.vscode/settings.json](../.vscode/settings.json) → `objectscript.conn` (server ref + ns)
3. VS Code user settings → `intersystems.servers[IRIS_SERVER_NAME || "jhip"]` (host, port, credentials)
4. Defaults: `http://localhost:52773`, ns `USER`

Credentials for this **test** instance (`_SYSTEM` / `jiahemk`) are documented in
[AGENTS.md](../AGENTS.md); the server still resolves them from the VS Code settings the
ObjectScript extension uses (env vars → workspace `objectscript.conn` → user
`intersystems.servers`), so nothing secrets-critical lives in the server code itself.

**WSL IP rotation:** the WSL NAT IP changes across reboots. When a request fails at the network
level, the server runs `wsl.exe hostname -I`, switches to the current IP and retries once
(`check_config` shows both the configured and the live host). Update the `jhip` entry in VS Code
user settings when prompted, so the ObjectScript extension stays in sync.

## SQL schema naming

Class `Package.Sub.Class` → SQL table `Package_Sub.Class` (package dots become underscores in the
schema; the class short name is the table). Examples: `Ens.MessageHeader` (1-level package stays
as-is), `Ens_Config.Item`, `Ens_Util.Log`. `INFORMATION_SCHEMA.ROUTINES` is available for catalog
queries. Non-SELECT statements execute but return no rowcount via this API.

## CLI

```powershell
node mcp/iris-v4-mcp.mjs --check             # connection report, exits 1 on failure
node mcp/iris-v4-mcp.mjs --install-helper    # (re)install the exec helper
node mcp/iris-v4-mcp.mjs --verbose           # MCP mode + HTTP logging to stderr
```

## Registration

### Claude Code

[.mcp.json](../.mcp.json) registers the server at project scope as `iris-v4`. Approve it when
Claude Code asks after restart. Skills that teach ObjectScript/Ensemble patterns alongside it live
in [.agents/skills/](../.agents/skills/) — including `iris-v4-mcp`, which documents this server's
tool surface and quirks for the agent. Because Claude Code only auto-discovers project skills
under `.claude/skills/`, that path holds a generated mirror; regenerate it with
`pwsh -File scripts/sync-agent-skills.ps1` after editing the source.

### DeepSeek Harness (DSH)

DSH does not read `.mcp.json`; it mounts MCP servers from its own profile patch. This server is
registered there as the `mcp-iris-v4` row (`@deepseek-ai/dsh-mcp-client`, stdio), so its tools
appear as `mcp__iris-v4__<tool>` in every DSH session:

```
%USERPROFILE%\.dsh\profiles\<profile>\cordis.patch.yml
```

The row pins the absolute `node.exe` path, the absolute script path, `cwd`, and
`IRIS_SERVER_NAME=jhip` / `IRIS_NAMESPACE=JHIP`; credentials are still resolved by the server
from the VS Code settings (see [Connection resolution](#connection-resolution)), so nothing
secret lives in the DSH profile. It deliberately omits `failOnStartupError`: with WSL/IRIS down
the row still activates and the tools report a connection error per call — the stdio child stays
alive, so the server recovers by itself once IRIS is reachable again (no DSH restart needed).
The same row is listed under **Settings → Skills & MCP**, where it can be disabled, reconfigured,
or reconnected.

The DSH instance also serves the local docs mirror and the production tools this repo's
[AGENTS.md](../AGENTS.md) describes; that document is the workflow reference once the tools are
visible. The ObjectScript skills it points at live in the tool-neutral
[.agents/skills/](../.agents/skills/) directory.
