---
name: objectscript-navigation
description: "Use when you must find, read or change code in the DHC-APP namespace on this Caché 2016.2.3 instance through the cache-v1 MCP server and you do not already know where the code lives. Covers locating a class, routine, table or global by name; deciding which class actually owns a table; tracing what calls what; repairing a class whose callers cannot be enumerated; and any edit under src/ where the server copy is the authority. This build has no code-body search, so the name-first workflow here is the only way in."
---

# Navigating and repairing a large Caché codebase with no code search

Namespace `DHC-APP`, Caché 2016.2.3 (Build 907_11_20753U), 34,197 class documents, Atelier REST
**v1 only**. Every limit below was measured on this instance; do not assume a capability not listed.

## What this build cannot do

There is no code-body search, and you cannot build one. These four routes are dead — do not
spend calls re-testing them:

- `/action/search` returns **404** on this build.
- `$preprocess` does not exist on 2016.2.3, so the Atelier source reader returns 0 lines.
- `%Dictionary.CompiledMethod.Implementation` and `%Dictionary.MethodDefinition.Implementation`
  are empty on every row.
- `^oddDEF(class,"m",method,...)` holds the comment, the formal spec and the return type — no body.

So "where is this symbol used?" has no query. Build the name list, narrow to a few candidates,
read those documents, reason. Reading is the budget you spend — spend little.

Also absent: **no Ensemble.** Zero `Ens.*` classes exist (`ENSEMBLE: enabled:false`), so never
look for a production, routing rule or message map.

## The twelve tools

| Tool                    | Use it for                                                            |
| ----------------------- | --------------------------------------------------------------------- |
| `cache_check_config`    | Resolve host/port/namespace, probe the server. No arguments.          |
| `cache_info`            | Version, API level, feature flags, namespace databases.               |
| `cache_docnames`        | **The only discovery tool.** List names by category + `pattern`.      |
| `cache_doc`             | `action=get` to read one document, `put` to save, `delete` to remove. |
| `cache_compile`         | Compile a `targets` array; reports per-name `OK`/`ERROR`.             |
| `cache_table_info`      | Properties, indices and the SQL table name of a class.                |
| `cache_resolve_storage` | Storage block: `DataLocation` / `IdLocation` / SQL map `<Global>`.    |
| `cache_query`           | SQL rows. Bound every statement with `TOP n`.                         |
| `cache_execute`         | Run ObjectScript, read what it `Write`s.                              |
| `cache_execute_method`  | Call one class method and get its return value.                       |
| `cache_global`          | `get`/`order` a global to see real data. `set`/`kill` need `confirm`. |
| `cache_sync`            | Push local `.cls`/`.mac`/`.int`/`.inc` files and compile them.        |

## Navigation workflow, in order

**1. Prove the connection once per session.** `cache_check_config`, no arguments. If it reports `FAILED`, stop — every later call is a guess about which server answered.

**2. Record what the instance is.** `cache_info`; its feature line is where `ENSEMBLE: enabled:false` shows up. Both calls are once-per-session, not once-per-task.

**3. Build the name list before you read anything.** `cache_docnames` is the only way to find a
document, and it filters *after* the server enumerates the whole category, so a CLS call costs
about 13 seconds and a licence unit however narrow your pattern. Once per category per session:

```
cache_docnames(category="CLS", pattern="web.DHCBL.CT", limit=5000)
cache_docnames(category="CLS", pattern="^web\.DHCBL\.CT\.CT[A-Z]", regex=true, limit=5000)
cache_docnames(category="RTN", type="mac", pattern="DHCWL")
```

Names are document names, so a class ends in `.cls` (`web.DHCBL.CT.CTLoc.cls`) — anchor regex on
that. Categories: `CLS`, `RTN` (add `type` of `mac`/`int`/`inc`), `CSP`, `OTH`.

**4. Narrow to one or two candidates, then read.** `cache_doc(action="get",
name="web.DHCBL.CT.CTLoc.cls")`. Header first (`Class ... Extends ...`), then `Property` and
`Index`, then methods. Never fetch a document you have not narrowed to by name.

**5. Decide what you are holding, and get its SQL name.**
`cache_table_info(className="web.DHCBL.CT.CTLoc")`. Zero properties *and* zero indices means a
business-logic class — next section. Otherwise it is persistent, and the `SQL table:` line it
prints is what every `FROM` clause needs.

**6. Map the table to its storage.** `cache_resolve_storage(className="User.CTLoc")` gives the
`data:` global, the `id:` global, any SQL map `<Global>` and the same `SQL table:` line. Read real
rows from the global it names, e.g. `cache_global(action="order", global="^CTLoc", limit=5)`.

**7. Ask SQL for columns and sanity checks, not the class.** Use the table name step 5 printed —
`cache_query(sql="SELECT TOP 5 * FROM SQLUser.CT_Loc")`, never `FROM User.CTLoc`, where `USER` is
a reserved word (SQLCODE -1). Bound it, always.

**8. Ask the dictionary for what the source cannot answer cheaply.** Scoped by `Parent`,
`%Dictionary.MethodDefinition` gives method names, formal specs, return types and comments —
everything `^oddDEF` holds except the body. Read the columns once with
`cache_query(sql="SELECT TOP 1 * FROM %Dictionary.MethodDefinition WHERE Parent = ?",
params=["web.DHCBL.CT.CTLoc"])`, then filter. `Implementation` comes back empty — expected.

**9. Write the finding down before you act.** Class, table, global, entry point, name list — the next session resumes from it instead of paying 13 seconds again.

**Starting from a symptom instead.** Begin at an entry point you can name — the CSP page, the
`.mac` routine, the web-service class — then walk forward, reading every `##class(Pkg.Class)` it
names. Forward walking beats guessing package names backwards.

## Three names for one thing: business class, persistent class, SQL table

`web.DHCBL.CT.CTLoc` has **0 properties and no storage** — nothing to SELECT. It fronts the
persistent class `User.CTLoc` (103 properties, has storage), whose SQL table is `SQLUser.CT_Loc`
with columns `CTLOC_RowID`, `CTLOC_Code`, `CTLOC_Desc`. A logic class is a facade; persistence
lives in another class, often in another package. **A class name is not a table name** — 5,842
persistent classes here have a `SqlTableName` differing from the class's short name.

Recovery when `cache_table_info` returns nothing:

1. Read the source: `cache_doc(action="get", name="web.DHCBL.CT.CTLoc.cls")`.
2. Look for the class in a method comment — this codebase writes it with a fullwidth colon,
   `Table：User.CTLoc`. Search the text you already fetched; no extra call.
3. No comment? Form a hypothesis: drop the layer segment (`CT`, `BO`, `DAO`, `BL`) and try the
   rest as a class in `User` — `web.DHCBL.CT.CTLoc` → `User.CTLoc`. Hypotheses are cheap.
4. Confirm with `cache_table_info(className="User.CTLoc")`: a property count proves it is the
   persistent class, and its `SQL table:` line names what `FROM` needs.
5. Only then resolve the global: `cache_resolve_storage(className="User.CTLoc")`.

Never report "the table does not exist" because a logic class had no properties — and never
because `FROM User.CTLoc` failed, which says nothing about the class.

## Changing code whose callers you cannot find

You cannot enumerate call sites or subclasses here. Do not pretend otherwise, and do not let a
compile result stand in for the check:

- **Compiling proves nothing about callers.** A call to a method that no longer exists still
  compiles and fails only at run time with `<METHOD DOES NOT EXIST>`.
- **Add, never replace.** New trailing parameters get defaults (`pMode As %String = ""`).
- **Rename by wrapping.** Add the new member; keep the old one as a one-line wrapper calling it.
  Delete the wrapper only when you can prove nobody calls it — which you cannot, so do not.
- **Prefer changing behaviour inside an existing signature**, and if an incompatible change is
  genuinely required, say so and stop: "callers cannot be enumerated on this build" is the answer.

## Repair procedure

1. Fetch the server copy of every file you intend to touch and compare it with the local one. The
   server holds the current state; on unexplained drift or an encoding difference, stop and report.
2. Read the class before changing it. The live definition is the truth, not the filename.
3. Change the local file; the `PostToolUse` hook pushes and compiles edited `src/` files. If you
   did not edit through a hook-covered tool, run `cache_sync(files=["src/web/DHCBL/CT/CTLoc.cls"])`.
4. Editing on the server directly: `cache_doc(action="put", name="web.DHCBL.CT.CTLoc.cls",
   content=<full source>)` saves but does **not** compile — always follow with
   `cache_compile(targets=["web.DHCBL.CT.CTLoc.cls"])`. Pass the whole `targets` list in one
   call; the tool strips `.cls` itself.
5. Compile in dependency order: base class first, then its subclasses. A superclass change is only
   proven by the subclasses compiling.
6. Verify by exercising the code, not by the absence of an error: call a public method you just
   read with `cache_execute_method(className="web.DHCBL.CT.CTLoc", methodName="<method>",
   args=["1"])`, and check the data path with `cache_query` or `cache_global`.
7. Re-read the per-target `OK`/`ERROR` lines. Localised success prose is not evidence.

## SQL rules for this instance

- **Bound everything.** A cross-table `LIKE '%x%'` over `%Dictionary.*` took 39 seconds and once
  returned an HTML error page instead of JSON. Always `TOP n` plus a leading `%STARTSWITH`; never a wildcard.
- **`NOT %STARTSWITH` is a syntax error** here (SQLCODE -14). Write `NOT (x %STARTSWITH y)`.
- **A class name is not a table name.** `%Dictionary.CompiledClass` carries `SqlTableName` and
  `SqlSchemaName`, and 5,842 persistent classes here override the default — `User.CTLoc` is
  `SQLUser.CT_Loc`. Both `cache_table_info` and `cache_resolve_storage` print it; read it, never
  derive it. `FROM User.CTLoc` fails anyway: `USER` is a reserved word (SQLCODE -1).
- **`%Dictionary.StorageDataDefinition` has no `DataLocation` column** (SQLCODE -29), and the
  `DataLocation` object property is multidimensional and unreadable from inside `XECUTE`. That is
  why `cache_resolve_storage` parses the class source. Do not try to rebuild it.
- An HTML body instead of JSON means the query was too heavy. Narrow it; it is not a connection
  failure, and re-running it unchanged will fail again.

## Error text is German

Server messages are localised: `FEHLER #5540`, `Kompilierung ... erfolgreich abgeschlossen`. Match
on the stable `#NNNN` code and on `cache_compile`'s per-target `OK`/`ERROR` prefix — never on
English words like "error" or "successful", and never read an unparsed German sentence as success.

## Text and encoding

Chinese literals round-trip: `cache_execute(code="Write \"病人\",!")` returns `病人`. Do not wrap an
already-UTF-8 string in `$ZCONVERT(x,"O","UTF8")` — that double-encodes it into mojibake.

## Stop conditions

- `cache_docnames` returns nothing for a pattern you expected to hit: the package name is wrong, not the class. Retry a shorter substring before concluding anything.
- A fetched document lacks the method you were told about: check `Extends` and read the superclass first.
- You are about to call a tool once per class in a loop. Stop: each request holds a licence unit until its session ends and roughly 300 exist — batch into one `targets` array instead.
