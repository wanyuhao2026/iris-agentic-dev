---
name: objectscript-mac-routines
description: Work with Caché routine documents (.mac, .int, .inc) instead of classes on DHC-APP (Caché 2016.2.3, Atelier v1, cache-v1 MCP) — find a routine by name, read and edit it, compile it, and exercise a label. Use when a task touches a legacy .mac, a CHUI or ETL routine, an .inc of #define macros, a generated .int, a label or extrinsic function ($$FN^RTN), $ZTRAP, or when a routine edit did not take effect, a compile reported nothing useful, or you are about to search routine bodies on a server that has no code search.
---

# Routine Documents (.mac / .int / .inc)

A routine is addressed by its file name and structured by labels, not by a class body. Every
statement below was measured on DHC-APP (Caché 2016.2.3, Atelier v1) via the `cache-v1` tools.

## 1. Find the document first

    cache_docnames {"category":"RTN","type":"mac","pattern":"BoeDhCom"}
    cache_docnames {"category":"RTN","type":"inc","pattern":"DateFormat"}
    cache_docnames {"category":"RTN","type":"mac","pattern":"^web\\.DHC","regex":true}

`pattern` is a case-insensitive substring unless `regex: true`, and `type` means nothing without
`category: "RTN"`. The call fetches the whole category and filters afterwards, so the pattern
shrinks the output, not the request — one call, never one call per routine. This namespace holds
34,197 class documents, so always pass `pattern`: an unfiltered listing with `limit` raised is how
you get an HTML error page instead of JSON.

The document name is the `ROUTINE <name>` header on line 1 plus the extension, not the file
name, so it can be dotted and can differ from the file: `BalanceDetails.mac` declares
`ROUTINE DHCWL.Account.BalanceDetails`, `2.int` declares `ROUTINE TestXH.IPService.2`, and an
include says `ROUTINE Nur.DateFormat [Type=INC]`. Read line 1; never infer the name from a
path. Dropping it breaks more than the compile — it also changes which document the file syncs
to, since `cache_sync` reads the header and falls back to the file name only when there is none.

## 2. Read it whole

    cache_doc {"action":"get","name":"BoeDhCom.mac"}

`cache_doc` has no fragment, line-range or line-edit mode: a get returns the entire document. There
is no code-body search either — `/action/search` returns 404, `$preprocess` does not exist on
2016.2.3, and the `%Dictionary` `Implementation` columns are empty. You cannot ask the server which
routine contains `^PAADM`. Get candidates by name, read them, search the text yourself, and never
propose a body search as a step.

A `.int` is the compiled intermediate of a `.mac`: read it to map a stack line back, but edit the
`.mac` — the next compile regenerates the `.int`. Compile a `.int` only when it is the only source.

Some `.mac` files here are compiler output: `User.DHCWMRNoTypeExt.T1.mac` opens with the comment
`; Filing Methods for table: ... Do Not Edit`, carries `#classmethod` and `#import` directives plus
`%delete` / `%insert` labels, and regenerates from the persistent class — edit the class instead.

## 3. Edit without breaking the layout

Indentation is syntax. A label starts in column 1, its body is indented at least one space or tab,
and dotted block lines start with `.` at the body's indentation. Reindenting turns statements into
labels and labels into statements, so copy the surrounding style instead of normalising it.

```objectscript
ROUTINE BoeDhCom

GetWRData(sDate,eDate)
    n (sDate,eDate)
    s sDate = $zdh(sDate,3)
    f mDate = sDate:1:eDate d
    . s wrId = "" f  s wrId = $o(^DHCWorkRegReport(0,"ADMDATE",mDate,wrId)) q:wrId=""  d
    .. q:$p($g(^PAADM(wrId)),"^",2)'="E"
    q 1
```

- Keep line 1: without it the compiler reads your first label as the routine name.
- `New` isolates locals, and a routine has no automatic scope: a called label that writes `x`
  changes the caller's `x`. `n (sDate,eDate)` keeps the arguments and protects the rest. Do not
  carry the class-method rule ("no `New` in a method") into a `.mac`.
- Exit a label with `Quit`; `Quit <value>` is the extrinsic return. A value on `Quit` inside a
  `For`/`While` body is a `<COMMAND>` error here — bare `Quit` leaves the loop.
- `#include %occInclude` pulls in macros, the name without an `.inc` extension;
  `#include web.DHCSTM.Macro` reaches the local `web.DHCSTM.Macro.inc`. `#include` is a
  preprocessor directive; `Include %occInclude` without the `#` is class syntax and is wrong.
- Legacy `$ZTRAP` (`s $zt="GetLogError"`, cleared in the handler or it recurses) is common. Keep
  the routine's error style; use `Try`/`Catch` for new labels rather than a second mechanism.
- Chinese round-trips: `Write "病人",!` is correct. Do not wrap an already-UTF-8 string in
  `$ZCONVERT(x,"O","UTF8")` — it double-encodes.
- `$ISLIST` and `$LISTVALID` do not exist on this build; `$LISTLENGTH` does. Do not "modernise"
  working list code with functions the instance lacks.

Write the whole document back, never a patch:

    cache_doc {"action":"put","name":"BoeDhCom.mac","content":"<full source>"}

`put` saves but never compiles, normalises CRLF, and replaces the whole document — `cache_compile` next.

## 4. Compile, and know what success looks like

    cache_compile {"targets":["BoeDhCom.mac"]}
    cache_compile {"targets":["BoeDhCom.mac","Nur.DateFormat.inc","web.abc.mac"]}

Pass the name with its extension. `cache_compile` strips a `.cls` suffix because
`$System.OBJ.Compile` wants the bare class name, but routine extensions are left alone — that is
the name the compiler expects for a routine. Passing `BoeDhCom` without `.mac` compiles nothing.

Batch targets into one call: each HTTP request holds a license unit until its CSP session ends and
about 300 are available. Never loop one compile per routine over a list.

Each result line starts with `OK ` or `ERROR `, and an `ERROR` line is followed by indented log
lines. The prose is this instance's German — `FEHLER #5351: Klasse ... existiert nicht` — so match
on the `OK `/`ERROR ` prefix and the `#NNNN` code, never on English words. An `.inc` is not
validated alone: compile a routine or class that `#include`s it. Then re-read the document — the
put response confirms a line count, not that the server holds the source you meant.

## 5. Exercise a label

`cache_execute_method` calls class methods only, so a routine label is not reachable through it.
Compile first, then call the label:

    cache_execute {"code":"Write $$GetWRData^BoeDhCom(\"2022-11-01\",\"2022-11-01\"),!"}
    cache_execute {"code":"Do GetWRData^BoeDhCom(\"2022-11-01\",\"2022-11-01\") Write \"done\",!"}

`cache_execute` wraps code in `XECUTE`, which takes one line: multi-line code is joined with spaces
and `//` and `;` comments are stripped first, respecting string literals. A `;` in a string survives,
a trailing `;` comment disappears — so test one call per invocation and keep the logic in the routine.

    cache_global {"action":"order","global":"^DHCWorkRegReport(0,\"ADMDATE\")","limit":10}

## 6. SQL inside a routine

`&sql(...)` compiles in a routine as it does in a class. `NOT %STARTSWITH` is a syntax error on this
build (SQLCODE -14); parenthesise it. A class name is not its table name — `User.DHCWMRNoTypeExt`
lives at `SQLUser.DHC_WMR_NoTypeExt`, and `cache_table_info` prints it. Keep probes bounded with
`TOP n` and a narrow `WHERE`: a heavy statement can make the CSP gateway answer with an HTML error
page instead of JSON, and it then reads as a transport fault. Success is `SQLCODE = 0`; read `#NNNN`.

    cache_query {"sql":"SELECT TOP 5 NTE_ParRef FROM SQLUser.DHC_WMR_NoTypeExt WHERE NOT (NTE_ParRef %STARTSWITH ?)","params":["DHC"]}

## 7. Traps that differ from class editing

| Class editing                      | Routine editing                                       |
| ---------------------------------- | ----------------------------------------------------- |
| name comes from the `Class X` line | name comes from the `ROUTINE` line, dots included      |
| `Include %occStatus`               | `#include %occStatus`                                  |
| `New` is illegal in a method       | `New` is required in a label that must not leak locals |
| `..Method()` self-reference        | `Do LABEL` / `$$LABEL(args)` in the same routine       |
| `##class(X).M()`                   | `Do LABEL^RTN` / `$$FN^RTN(args)` across routines      |
| error points at a method           | error reads `label+3^BoeDhCom`; map it back to source  |
| `Return sc`                        | `Quit` / `Quit value`                                  |

Deleting or renaming a routine is not a local change: callers reference it by name and you cannot
search bodies to find them. Search the local `src/` tree, read the likely callers, and say plainly
that callers outside the repository were not checked.

There is no Ensemble here — zero `Ens.*` classes exist — so never propose moving a routine into a
production or reaching for interoperability machinery that this namespace does not have.

Before you finish: line 1 still names the routine, indentation is untouched, the document is
compiled and you quoted the `OK`/`ERROR` lines, and any `.inc` consumer compiles too.
