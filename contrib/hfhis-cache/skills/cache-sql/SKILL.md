---
name: cache-sql
description: Write, debug, and run SQL against Caché 2016.2.3 (namespace DHC-APP) through the cache-v1 MCP server — embedded &sql() inside ObjectScript, dynamic %SQL.Statement, and the traps that keep it wrong. Use when a task touches a SELECT/INSERT/UPDATE on a DHC-APP table, an &sql() block, SQLCODE, a missing table or column, a query returning nothing, a FROM clause taken from a class name, a SQLCODE -1/-14/-29/-30/-119/-400, or a query so slow the gateway answers with an HTML page instead of JSON.
---

# Caché SQL — embedded and dynamic, on 2016.2.3

Target: **Caché 2016.2.3** (Build 907_11_20753U), namespace **DHC-APP**, Windows, web port 57772, Atelier REST
**v1 only**. Every claim below was measured on this instance. Habits from newer releases, PostgreSQL or SQLite are
wrong here until you have run them here.

## 0. Which tool does what

| Need | Tool |
| --- | --- |
| Confirm the connection, then run a SELECT with `?` placeholders plus `params` | `cache_check_config`, `cache_query` |
| Run ObjectScript — `&sql`, `%SYSTEM.SQL`, `%Dictionary` | `cache_execute` |
| SQL table name, columns and indices of a class | `cache_table_info` |
| The global behind a class, plus its SQL table name | `cache_resolve_storage` |
| A class's source, class names by pattern, then compile | `cache_doc`, `cache_docnames`, `cache_compile` |
| Not SQL: `cache_execute_method`, `cache_info`, `cache_sync`; `cache_global` reads a global | — |

**You cannot search code on this build** — `/action/search` returns 404. Find the class with `cache_docnames`,
fetch it with `cache_doc`, and read the text you got back.

## 1. Never write a FROM clause from a class name

`web.DHCBL.CT.CTLoc` is logic: 0 properties, no storage, nothing to SELECT. The persistent class is `User.CTLoc`,
103 properties — but its SQL table is **`CT_Loc` in schema `SQLUser`**, because the class sets `SqlTableName`. Every
one of these fails:

```sql
SELECT * FROM User.CTLoc                -- SQLCODE -1: USER is a reserved word
SELECT * FROM "User".CTLoc              -- SQLCODE -1: identifier expected
SELECT * FROM User_CTLoc                -- SQLCODE -30: SQLUSER.USER_CTLOC not found
SELECT CTLOC_Code FROM SQLUser.CT_Loc   -- works; columns carry the table's prefix
```

Columns are named from the table, not from the property: `CTLOC_RowID`, `CTLOC_Code`, `CTLOC_Desc`,
`CTLOC_ActiveFlag`. There is no `Code` and no `Label` on this table.

This is the norm, not a curiosity: **5,842 persistent classes here have a `SqlTableName` that differs from the
class's short name.** Call `cache_table_info(className="User.CTLoc")` before writing the FROM clause — it prints
`SQL table : SQLUser.CT_Loc` and flags a name that differs. `cache_resolve_storage` prints it too, beside the globals.

The default derivation still holds — last dot separates schema from table, earlier dots become underscores — but
`SqlTableName` overrides it, and here it overrides it 5,842 times. Confirm with
`SELECT Name, SqlTableName, SqlSchemaName FROM %Dictionary.CompiledClass WHERE Name = 'User.CTLoc'`.

## 2. `NOT %STARTSWITH` is a syntax error here

This build parses `NOT %STARTSWITH` as a missing comparison operator: **SQLCODE -14**, "comparison operator
required". No word order saves it — the negated operator form is what fails.

```sql
-- WRONG on 2016.2.3 — SQLCODE -14
WHERE Name NOT %STARTSWITH 'User.'

-- CORRECT — parenthesise the predicate
WHERE NOT (Name %STARTSWITH 'User.')
-- or invert the question so the predicate stays positive and indexable
WHERE Name %STARTSWITH 'web.DHCBL.'
```

`%STARTSWITH` is literal: `%` and `_` are just characters, while `LIKE 'A_%'` reads `_` as "any character" and
matches `AB1` when you meant `A_1`. Prefer `%STARTSWITH`, and pass input through `?`.

## 3. Read the SQLCODE, never the prose

Error text is **localised German**:

```text
FEHLER #5540: SQLCODE: -29 Meldung: Feld "FOO" wurde unter den anwendbaren Tabellen nicht gefunden
```

`#5540` is the error class, `-29` is the SQLCODE, and the `Meldung` sentence is decoration that changes with locale
and build. Match the number; treat the text as a hint.

| SQLCODE | Means here |
| --- | --- |
| 0, 100 | success; no rows — 100 is not an error |
| -1 | identifier or reserved word rejected — e.g. `FROM User.CTLoc` |
| -14 | comparison operator required — usually `NOT %STARTSWITH` |
| -29 | field not found — wrong column, or `"text"` where you meant `'text'` |
| -30 | table not found — `SqlTableName` ignored, or a class with no storage |
| -37, -51 | scalar/aggregate on a stream field; `CALL` where `SELECT` is expected |
| -114 | lock timeout, **not** a miss |
| -400 | unsupported, or a failure raised at fetch, e.g. `<DIVIDE>` in the select list |

## 4. SQLCODE in ObjectScript: 0 is success and therefore falsy

```objectscript
// WRONG — fires on 100 and on every negative code, so an SQL error is reported as "not found"
If SQLCODE { Return "NOT FOUND" }
// CORRECT — three outcomes, three answers
&sql(SELECT CTLOC_Desc INTO :tDesc FROM SQLUser.CT_Loc WHERE CTLOC_Code = :pCode)
If SQLCODE = 100 { Return "" }
If SQLCODE < 0   { Throw ##class(%Exception.SQL).CreateFromSQLCODE(SQLCODE, %msg) }
Return tDesc
```

Returning `""` on a negative code reports a failure as not-found — throw, or return a `%Status` instead.

## 5. Embedded vs dynamic

Embedded SQL is compiled into the method — fast, static, the default for a fixed statement. Reach for `%SQL.Statement`
only when a table or column name is known at run time.

```objectscript
Set stmt = ##class(%SQL.Statement).%New()
Set sc = stmt.%Prepare("SELECT TOP 50 CTLOC_Code, CTLOC_Desc FROM SQLUser.CT_Loc WHERE CTLOC_Code %STARTSWITH ?")
If $$$ISERR(sc) { Return sc }              // %Prepare returns the error; it does not throw
Set rs = stmt.%Execute("A")
If rs.%SQLCODE < 0 { Return $$$ERROR($$$GeneralError, rs.%Message) }
While rs.%Next(.tSC) { Set out(rs.%Get("CTLOC_Code")) = rs.%Get("CTLOC_Desc") }
If $$$ISERR(tSC) { Return tSC }
```

Check twice. A select-list error (`Amount/0`) does not appear after `%Execute`: `%SQLCODE` is 0 there, `%Next(.tSC)`
stops with -400 in `tSC`, and a check made only after `%Execute` reads the failure as an empty result.

## 6. What a SELECT leaves in its INTO variables

- `SELECT COUNT(*) INTO :n` always returns one row, so `SQLCODE` is 0 and `n` is `0` even when it was never set.
  `COUNT(*)` cannot report "nothing matched" — test `n = 0`.
- A plain `SELECT` that finds no row sets `SQLCODE = 100` **and overwrites each INTO variable with `""`**, so a
  default assigned before the query does not survive a miss. Branch on `SQLCODE`.
- **-114 is a lock timeout, not a miss.** A read of a row another process holds waits, then fails. The INTO variable
  has been seen holding the locked row in some runs and empty in others, so a 100-only check proceeds on either.
  Treat -114 as an error like any other negative.
- **NULL is not the empty string.** `Set obj.Field = ""` stores an empty string, so `IS NULL` misses it. Use
  `(Field IS NULL OR Field = '')` for absent-or-empty, `(Field IS NOT NULL AND Field <> '')` for has-a-value.

## 7. Bound every query — the gateway punishes the rest

- Each HTTP request holds a license unit until its CSP session ends; roughly **300** are available. One query per
  row, or per class in a loop over thousands, takes the instance down for everyone. Batch: one statement, a `TOP`,
  a set of `?` parameters.
- Heavy or unbounded SQL makes the CSP gateway answer with an **HTML error page instead of JSON**. Measured: a
  cross-table `LIKE '%x%'` over `%Dictionary.MethodDefinition` ran 39 seconds and came back as HTML — in a client
  that reads as a transport failure, not as a slow query.

```sql
-- WRONG — full scan of a dictionary table, unanchored, unbounded
SELECT * FROM %Dictionary.MethodDefinition WHERE Name LIKE '%Save%'
-- RIGHT — bounded, anchored, cheap
SELECT TOP 50 Name FROM %Dictionary.MethodDefinition WHERE Name %STARTSWITH 'web.DHCBL.'
```

Put placeholders in the SQL string and values in `cache_query`'s `params` array; never concatenate values into SQL.
After a write, confirm the effect with a follow-up SELECT or `%ROWCOUNT` (§11). `SELECT COUNT(*) FROM CT_Loc` returns
519, but `TOP n` on that table still returns all 103 columns — name the columns you need.

## 8. What this build does not have

- **`$ISLIST`, `$LISTVALID` and `$preprocess` are missing.** `$LISTLENGTH`, `$LISTFROMSTRING`, `$LISTTOSTRING` and
  `$LISTFIND` work. Guard with `$Get`/`$Data` instead of testing list validity.
- **`%Dictionary.CompiledMethod.Implementation` and `%Dictionary.MethodDefinition.Implementation` are both empty** —
  you cannot read a method body through SQL on this instance. Use `cache_doc`.
- **`%Dictionary.StorageDataDefinition` has no `DataLocation` column**; selecting it is SQLCODE -29. To find a
  class's global, read the class source and look at the Storage block's `<DataLocation>` or the SQL map's `<Global>`
  tag. `cache_resolve_storage` does exactly that — use it rather than hand-rolling the query.
- **There is no Ensemble here** — zero `Ens.*` classes, no interoperability tables to join.
- **`cache_query` has no explain mode, and `$SYSTEM.SQL.Explain` does not exist here** (`<METHOD DOES NOT EXIST>`).
  The entry point is `ShowPlan`, which takes the SQL **by reference as a numbered array**; `array=0` writes the
  plan, `array=1` leaves it in `%plan()`.

```objectscript
Set sql=1,sql(1)="SELECT TOP 3 Name FROM %Dictionary.ClassDefinition"
Do ##class(%SYSTEM.SQL).ShowPlan(.sql,0)
```

That prints `<plan> <sql>…</sql> <cost value="20400200"/> Read master map …Master, looping on ID. … </plan>`.
`%SQL.Statement` has no `%GetPlan()`. `ShowPlanAlt` (→ `%AltPlan()`), `FreezePlans`, `SetSQLStats` and the `Export*`
methods are also on `%SYSTEM.SQL`; if a name is missing, ask that class what it exposes rather than guessing.

## 9. Predicates worth knowing

```sql
-- %MATCHES is regex-ish and full-string anchored: no wildcards needed at the ends
WHERE CTLOC_Code %MATCHES '[0-9][0-9][0-9]'      -- LIKE would read [0-9] as literals
-- %List column: LIKE searches the binary encoding and matches nothing
WHERE ? %INLIST Tags                             -- exact element
WHERE FOR SOME %ELEMENT(Tags) (%VALUE LIKE ?)    -- needs an Index X On Tags(ELEMENTS)
```

`%INLIST` is SQL-only; in method code write `$LISTFIND($LISTFROMSTRING(..Tags, ","), tag) > 0`. `"double quotes"`
delimit identifiers (`WHERE Title = "hello"` is -29); ObjectScript `'=` breaks a SQL literal — write `'…'` and `<>`.

## 10. Dates

Dates are `$HOROLOG` integers — days since 31 Dec 1840, time as seconds since midnight. `+$HOROLOG` is today,
`$ZDATE(n,3)` renders `"YYYY-MM-DD"`, `$ZDATEH("2026-01-15",3)` converts back. Pass the integer in SQL
(`WHERE EffectiveDate <= ?`). `$ZDATETIME` is not SQL (SQLCODE -12) — use `CURRENT_TIMESTAMP`, and `%TimeStamp`
literals take a space (`2026-01-15 08:30:00`), never ISO's `T`.

## 11. Writes

```objectscript
&sql(UPDATE SQLUser.CT_Loc SET CTLOC_ActiveFlag = 0 WHERE CTLOC_Code = :pCode)
If SQLCODE < 0 { Return $$$ERROR($$$GeneralError, "update failed: " _ %msg) }
If %ROWCOUNT = 0 { Return $$$ERROR($$$GeneralError, "no such row: " _ pCode) }
Return %ROWCOUNT
```

`%ROWCOUNT` is the affected-row count. **Never return `SQLCODE` as a count** — it is 0 on every success, including
the one that changed nothing.

There is no `MERGE`, no `INSERT OR IGNORE`, no `INSERT IGNORE`, no `ON CONFLICT ... DO NOTHING`. A duplicate primary
key is -119; for upsert, `%OpenId` then `%New`, or DELETE then INSERT. A property with an `InitialExpression` fills
itself when an INSERT omits the column — leave it out.

## 12. Before you run it

- [ ] FROM clause and columns taken from `cache_table_info` — the property name is not the column name
- [ ] No `NOT %STARTSWITH`, no `"double-quoted strings"`, no `$ZDATETIME` in SQL
- [ ] `TOP n` and an anchored WHERE with named columns; no `%`-leading LIKE over a dictionary table
- [ ] Values bound through `?` / `params`, not concatenated
- [ ] Every `SQLCODE` branch separates 100 from a negative; none returns `""` for an error
- [ ] Writes verified with `%ROWCOUNT` or a follow-up SELECT
