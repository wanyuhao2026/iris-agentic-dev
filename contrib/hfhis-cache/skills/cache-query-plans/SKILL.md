---
name: cache-query-plans
description: Diagnose and fix slow SQL on Caché 2016.2.3 (namespace DHC-APP) through the cache-v1 tools. Use when a query, report or list screen is slow, when a COUNT or search returns fewer rows than the table holds, right after adding an Index to a persistent class, after a bulk load, or when you went looking for a query plan and cache_query answered SQLCODE -51. Covers finding the SQL table behind a class, reading a plan with %SYSTEM.SQL.ShowPlan through cache_execute, stale class-added indices, %BuildIndices, INSERT %NOINDEX loaders, TUNE TABLE and outlier selectivity.
---

# Caché query plans and indices — 2016.2.3

Target: **Caché 2016.2.3** (Build 907_11_20753U), namespace **DHC-APP**, Windows, web port 57772,
Atelier REST **v1 only**. All measured here; 2024+ advice on statistics and adaptive plans does not apply.

## 1. The explain entry point is dead — plan around it

`cache_query` takes `sql` and `params`. That is its whole schema: no `mode`, no explain flag, no
plan field. Prefix a statement with `EXPLAIN` and the answer is **`SQLCODE -51`**, not a plan — the
query action cannot prepare it. Do not spend three calls rephrasing the request.

The plan comes from the engine through `cache_execute`; the index advice below survives untouched.
`cache_check_config` first, `cache_info` for the version, `cache_execute_method` for one method.## 2. Find the SQL table, then the query

`web.DHCBL.CT.CTLoc` has 0 properties; it is logic. The persistent class is `User.CTLoc` (103
properties), and its **SQL table is `SQLUser.CT_Loc`** — 5,842 classes here have a `SqlTableName`
that differs from the class name. `FROM User.CTLoc` is `SQLCODE -1`, "reserved word USER gefunden";
`FROM SQLUser.CT_Loc` returns 519 rows with `CTLOC_RowID`, `CTLOC_Code`, `CTLOC_Desc`.

```
cache_table_info(className="User.CTLoc")
cache_resolve_storage(className="User.CTLoc")
```

Both take the **class** name. `cache_table_info` prints the SQL table name on its own line: read it
and use it in every SQL statement — never derive a table from a class name. The second prints the
data global; read a few nodes of it with `cache_global(action="order", global="^CTLoc", limit=5)`,
substituting the name it printed.

There is no code search here: `/action/search` is 404, `$preprocess` is absent, and
`%Dictionary.*.Implementation` is empty. Find names with `cache_docnames(pattern="CTLoc")` — a
case-insensitive **substring** filter, not a glob, so `"CTLoc*"` matches nothing — then read the
source with `cache_doc(action="get", name="web.DHCBL.CT.CTLoc.cls")`. Always pass `limit`.

## 3. Get a plan: `%SYSTEM.SQL.ShowPlan`

`$SYSTEM.SQL.Explain` is not on this build — `<METHOD DOES NOT EXIST> *Explain,%SYSTEM.SQL` — and
`%SQL.Statement` has no `%GetPlan`. `ShowPlan` is:

```
ShowPlan(sql:%Library.String, array:%Library.Boolean=0, showstats:%Library.Boolean=0, packages:%Library.List="", schemapath:%Library.String="", preparse:%Library.Boolean=0)
```

Pass `sql` **by reference** as a numbered array — `sql=#lines`, `sql(1)` the first line, `sql(2)` the
next. `array=0` writes the plan out; `array=1` leaves it in `%plan()` for `For { Set i=$Order(%plan(i)) Quit:i="" Write %plan(i),! }`;
`showstats=1` also runs the query and appends statistics. `ShowPlanAlt` takes the same arguments and
leaves `%AltPlan()`.

```
Set sql=1,sql(1)="SELECT TOP 3 Name FROM %Dictionary.ClassDefinition" Do ##class(%SYSTEM.SQL).ShowPlan(.sql,0)
```

```
<plan>
<cost value="20400200"/>
Read master map %Dictionary.ClassDefinition.Master, looping on ID.
</plan>
```

Read it by phrase, not case: `Read master map ... looping on ID` is a full scan, `Read index map`
means an index is in use, `Cost` is an estimate you compare plans with. Hunt a master map nested
inside a loop over another master map; a master map read by idkey is fine.

`cache_execute` runs your code through `XECUTE` on one flattened line — no `#include`, no `$$$`
macros, braces on the same line for every block; `//` and `;` comments are stripped first.

## 4. Measure as well — `Cost` is an estimate

```
Set t0 = $ZHOROLOG, n = 0
Set rs = ##class(%SQL.Statement).%ExecDirect(,"SELECT COUNT(*) FROM SQLUser.CT_Loc WHERE CTLOC_Code %STARTSWITH 'A'")
If rs.%Next() { Set n = rs.%GetData(1) }
Write "count=", n, " elapsed=", ($ZHOROLOG - t0), !
```

`$ZHOROLOG` is seconds since the process started, so the difference is a real duration. Re-run with
`%NOINDEX` (step 5): a gap means the index is doing the work. Then re-run with the literal you serve.

## 5. An index added in the class is empty until you build it

Add an `Index CodeIdx On <Property>;` — `<Property>` is the class-side name `cache_table_info` prints
beside `sqlField=CTLOC_Code` — and compile. That defines the index; it does not fill it. Queries the
planner then sends through it return short, with `SQLCODE = 0` and no error. `%NOINDEX` is the only
reliable way to see it:

```
cache_query(sql="SELECT COUNT(*) FROM SQLUser.CT_Loc WHERE CTLOC_Code = ?", params=["A01"])
cache_query(sql="SELECT COUNT(*) FROM SQLUser.CT_Loc WHERE %NOINDEX CTLOC_Code = ?", params=["A01"])
```

Different counts mean the index is stale — the second number is the truth. Build it:

```
Write $System.Status.GetErrorText(##class(User.CTLoc).%BuildIndices($ListBuild("CodeIdx"))), !
```

Note the split: SQL statements take the table `SQLUser.CT_Loc`, while `%BuildIndices` is a class
method and takes `User.CTLoc`. Take the index name from `cache_table_info`. `$ListBuild` makes the
list and `$ListLength` works, but `$ISLIST` and `$LISTVALID` do not exist.

## 6. `INSERT %NOINDEX` leaves the index behind

`INSERT %NOINDEX INTO SQLUser.CT_Loc ...` writes the row and skips index maintenance, so the index
misses it and every count through the index is short. The loader owns the build:

```
For i = 1:1:tCount { Set tRs = tStmt.%Execute(tVal(i)) If tRs.%SQLCODE < 0 { Write "SQLCODE=", tRs.%SQLCODE, " ", tRs.%Message, !  Quit } }
Write ##class(User.CTLoc).%BuildIndices($ListBuild("CodeIdx")), !
```

The build goes at the end of the inserting method, after the last insert. A rebuild by hand lasts
until the next load: if the loader clears the extent and inserts with `%NOINDEX` again, the index is
empty again. Fix the loader — a `%BuildIndices` you ran once in a console is not a fix.

## 7. `TUNE TABLE`, then look at the value the query binds

`TUNE TABLE` measures extent size, per-field selectivity, outlier values and block counts, and
writes them into the class's Storage definition. Run it on data shaped like production, after a
large load, or when the distribution moves. It is not a SELECT:

```
Set rs = ##class(%SQL.Statement).%ExecDirect(,"TUNE TABLE SQLUser.CT_Loc")
Write "SQLCODE=", rs.%SQLCODE, " ", rs.%Message, !
```

Then read the class with `cache_doc` and look in the Storage block for `ExtentSize` and
`Selectivity` — where the numbers landed; default values mean the planner is guessing. This build
has one set of statistics, so there is no fixed-versus-collected split to chase. A cached plan
survives all of it: prepare again with different statement text and re-run `ShowPlan`.

If one value covers most of the rows, the planner refuses the index for that value on purpose —
reading 98% of a table through an index costs more than a scan — and a master-map plan carrying an
outlier-selectivity note is **correct**. Check which value your query binds before you add an index:
a master-map plan for the common value and an index-map plan for a rare one are both right.

## 8. What fails here, and what it looks like

- **Errors are German.** `FEHLER #5540: SQLCODE: -29 Meldung: ...` — read the `#NNNN` and the
  negative SQLCODE; the sentence changes with locale, the numbers do not.
- **Heavy SQL returns HTML, not a timeout.** A cross-table `LIKE` over
  `%Dictionary.MethodDefinition` ran 39 seconds and once returned an HTML page instead of JSON.
  The CSP gateway gave up: narrow it, never retry the text. `cache_query` caps at 500 rows anyway.
- **`NOT %STARTSWITH` is a syntax error** — `SQLCODE -14`. Parenthesise the predicate:
  `WHERE NOT (CTLOC_Code %STARTSWITH 'A')`. `%STARTSWITH` is literal, so it is also the cheap test.
- **One request, one licence unit.** Each HTTP request holds a unit until its CSP session ends and
  the instance has about 300. A tool call per row, or per class over thousands, takes the instance
  down for everyone. Aggregate in one statement and never loop `cache_query`.

## 9. Changing an index from the tools

`cache_doc(action="put")` saves the source and does **not** compile it. The order is fixed: put,
then `cache_compile(targets=["User.CTLoc.cls"])` (each line starts with `OK` or `ERROR` — read it),
then `cache_execute` the `%BuildIndices` call from step 5, then re-run the `%NOINDEX` pair and the
measurement. If the source is a local file, `cache_sync(files=["src/User.CTLoc.cls"])` pushes and
compiles in one call; it still does not build the index.

## Checklist

1. `cache_check_config` first: namespace `DHC-APP`, version, exec helper.
2. Read the SQL table name from `cache_table_info` on the class (`User.CTLoc` → `SQLUser.CT_Loc`); `cache_resolve_storage` for the global.
3. Never ask `cache_query` for a plan — that is `SQLCODE -51`. Get it from `%SYSTEM.SQL.ShowPlan` through `cache_execute`.
4. Counts differ between the plain and the `%NOINDEX` query: the index is stale. `%BuildIndices`.
5. A loader using `INSERT %NOINDEX` calls `%BuildIndices` itself, after its last insert.
6. After a large load, `TUNE TABLE` through `cache_execute`, then re-plan with changed statement text.
