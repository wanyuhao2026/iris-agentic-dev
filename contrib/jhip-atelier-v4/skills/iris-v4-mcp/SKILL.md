---
name: iris-v4-mcp
description: The local iris-v4 MCP server for this project's HealthConnect 2020.1 (WSL) instance — tool surface, v4 quirks, and everyday workflows (query, exec, compile loop, production tracing). Use whenever an IRIS MCP tool is called or fails, or when other skills mention iris-agentic-dev tools.
trigger: Before calling any iris_* MCP tool, or when an iris_* tool errors
---

# iris-v4-mcp — this project's IRIS MCP server

This repo's IRIS is **InterSystems HealthConnect 2020.1.1** in local WSL. Its Atelier REST API
maxes out at **v4**, so the official `iris-agentic-dev` MCP (needs v8 / IRIS 2023.1+) cannot
connect. Instead the `iris-v4` MCP server (see [mcp/](../../../mcp/README.md)) provides a v4-only
tool set. Other installed skills may reference official tools by name — the ones that exist here:
`check_config`, `iris_query`, `iris_execute`, `iris_doc`, `iris_docnames`, `iris_compile`,
`iris_search`, `iris_production`. Anything else (symbols, ws_*, globals admin, source control)
does not exist here — don't call it.

## Connection facts

- Host = the WSL IP (changes across reboots; the server auto-refreshes via `wsl hostname -I`),
  port 52773, namespace `JHIP` (default), user `_SYSTEM` — resolved from VS Code settings.
- If every tool fails: call `check_config` first. It shows the live host and diagnostics.

## Quirks that matter

- **`iris_execute` runs via a SqlProc helper** (`CALL JHIP_MCP.Run_Exec(?)`, class
  `JHIP.MCP.Run`). Output = whatever the code `Write`s. The helper is auto-synced into a
  namespace on use, by comparing its body — edit `src/JHIP/MCP/Run.cls` and the next call
  pushes it; `--install-helper` forces the push.
- **Multi-line code is supported**: lines are joined with spaces before `XECUTE`, after
  stripping `//` and `;` comments (string literals are protected) and dropping blank /
  `#;` lines. Blocks, `Try { } Catch { }` and `_` continuations work.
- In exec code: use **`$ZNSPACE`** to read the namespace — `$ZNAMESPACE` and `$ZLEVEL` throw
  `<SYNTAX>` in this context. No interactive `Read`. Avoid `HALT`.
- **No `$$$` macros and no `#include`** — `XECUTE` runs no preprocessor, so `$$$ISOK` /
  `$$$ISERR` / `$$$OK` are `<SYNTAX>` errors (even single-line). Use
  `$System.Status.IsOK(sc)` / `$System.Status.IsError(sc)` / `1` instead. AGENTS.md snippets
  use the macros because they are written for compiled classes — translate before pasting.
- **SQL table naming**: class `Pkg.Sub.Class` → table `Pkg_Sub.Class` (package dots become
  underscores). One-level packages stay dotted: `Ens.MessageHeader`, but `Ens_Config.Item`,
  `Ens_Util.Log`, `Ens_MessageHeader` is WRONG. `INFORMATION_SCHEMA.ROUTINES` works.
- `iris_query` returns no rowcount for INSERT/UPDATE/DELETE (Atelier v4 drops it) — verify
  with a follow-up SELECT. Always `TOP n` big tables (output caps at 500 rows).
- Compile flags default `cuk` like the VS Code extension. Server console messages may appear
  in **German** (instance locale) — that is normal.
- `iris_doc` put does not compile — chain `iris_doc` (mode=put) → `iris_compile`.
- `iris_search`: `files` is required server-side (defaults to `*.cls,*.mac,*.int,*.inc`);
  `sys=true` includes %-prefixed system docs.
- `iris_production` start/stop/restart require `confirm: true`.

## Everyday workflows

**Compile loop** — `iris_docnames` (filter `JHIP.%`) → `iris_doc` (get) → edit → `iris_doc`
(put) → `iris_compile` (`targets: ["Pkg.Name.cls"]`) — read the console log for errors.

**Trace a message** — `iris_production` messages (newest `Ens.MessageHeader` rows) → pick the
ID → `iris_execute`:
`Set h=##class(Ens.MessageHeader).%OpenId(58) Set b=##class(h.MessageBodyClassName).%OpenId(h.MessageBodyId) ZWrite b`
(stream properties: `Write b.StreamProp.Read()`).

**Production ops** — `iris_production` status / items / event_log (Ens_Util.Log; Type 2=error)
/ messages. Start/stop/restart need confirm. Item settings:
`iris_execute` `Do ##class(Ens.Director).GetHostSettings("ItemName",.s) ZWrite s`.

**Explore code** — `iris_search` (content) beats fetching classes blind; `iris_docnames` with
a `JHIP.%`-style filter lists project classes (the namespace also sees IRISLIB/HSLIB/ENSLIB
system docs — always filter).
