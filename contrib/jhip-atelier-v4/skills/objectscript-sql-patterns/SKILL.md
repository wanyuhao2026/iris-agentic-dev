---
author: tdyar
benchmark_date: "2026-04-02"
benchmark_iris_version: "2025.1"
benchmark_tasks:
 - jira-001
 - jira-002
 - jira-003
 - jira-004
 - jira-005
 - jira-006
 - jira-007
 - jira-008
 - jira-009
 - jira-010
 - jira-011
 - jira-012
 - jira-013
 - jira-014
 - jira-015
 - jira-016
 - jira-017
 - jira-018
 - jira-019
 - jira-020
 - jira-021
 - jira-056
description:
 "ObjectScript embedded SQL, %SQL.Statement, date filtering, NULL handling,
 table naming. Use when writing SQL queries in ObjectScript classes, especially for
 filtering, date ranges, or dynamic queries.

 "
iris_version: ">=2024.1"
name: objectscript-sql-patterns
tier: core
pass_rate: 0.5909090909090909
state: reviewed
tags:
 - objectscript
 - sql
trigger:
 Any ObjectScript code with &sql(), %SQL.Statement, %Prepare, %Execute, SQLCODE,
 or SQL WHERE clauses.
---

# ObjectScript SQL Patterns

## 1. Table Naming: IRIS SQL Schema Convention

The SQL table name depends on package depth:

```text
// Two-level class (Package.ClassName):
Catalog.Item  →  SQL table: Catalog.Item   (schema=Catalog, table=Item)
Healthcare.Patient  →  SQL table: Healthcare.Patient

// Three+ levels (dots before last become underscores):
pkg.isc.genai.Router  →  SQL table: pkg_isc_genai.Router
My.Deep.Pkg.Widget   →  SQL table: My_Deep_Pkg.Widget
```

Rule: **last dot = schema/table separator; all preceding dots → underscores**.

Verified on IRIS 2026.2: `Test127.Ext` answers as `Test127.Ext` and gives SQLCODE -30 as
`Test127_Ext`; `Test127.Sub.Deep` answers as `Test127_Sub.Deep`.

```objectscript
// CORRECT for two-level class Catalog.Item:
Set sc = stmt.%Prepare("SELECT Name FROM Catalog.Item WHERE Category = ?")

// WRONG:
Set sc = stmt.%Prepare("SELECT Name FROM Catalog_Item WHERE Category = ?")
```

## 2. Active Record Filter — NULL OR Future Date

```objectscript
// Filter records where ExpiryDate is null (never expires) or in the future:
Set sc = stmt.%Prepare(
    "SELECT Name FROM Catalog.Item " _
    "WHERE Category = ? " _
    "AND (ActiveUntil IS NULL OR ActiveUntil >= ?)"
)
Set rs = stmt.%Execute(category, +$HOROLOG)

// +$HOROLOG gives today's date as an integer (IRIS $HOROLOG date part)
// ActiveUntil stored as %Integer ($HOROLOG format) or %Date
```

## 3. SQLCODE Semantics — 0 = Success (Falsy), 100 = No Rows

```objectscript
&sql(SELECT Label INTO :result FROM MyTable WHERE Code = :code)

// WRONG — 0 is falsy, so a found row gets through, but the branch
// fires on 100 and on every negative code, so an SQL error comes back as "NOT FOUND":
If SQLCODE { Return "NOT FOUND" }

// CORRECT — no row and an error are different answers:
If SQLCODE = 100 { Return "NOT FOUND" }    // 100 = no rows
If SQLCODE < 0   { Throw ##class(%Exception.SQL).CreateFromSQLCODE(SQLCODE, %msg) }
Return result                               // SQLCODE = 0 = found
```

## 4. %SQL.Statement Full Pattern

```objectscript
Set stmt = ##class(%SQL.Statement).%New()
Set sc = stmt.%Prepare("SELECT Name, Value FROM Config.Setting WHERE Name = ?")
If $$$ISERR(sc) { Return $$$ERROR($$$GeneralError, "Prepare failed: "_$System.Status.GetErrorText(sc)) }

Set rs = stmt.%Execute(name)
If rs.%SQLCODE < 0 { Return $$$ERROR($$$GeneralError, "Execute failed: "_rs.%Message) }

Set tSC = $$$OK
While rs.%Next(.tSC) {
    Set name = rs.%Get("Name")
    Set val  = rs.%Get("Value")
}
If $$$ISERR(tSC) { Return tSC }
If rs.%SQLCODE < 0 { Return $$$ERROR($$$GeneralError, "Fetch failed: "_rs.%Message) }
```

Check twice. `%SQLCODE` after `%Execute` covers errors raised before the first row. With `%ExecDirect`, a missing table is -30 here. An error in the select list happens when the row is fetched. `SELECT Amount/? ...` with 0 has `%SQLCODE = 0` after execute. Then `%Next(.tSC)` returns 0 with SQLCODE -400 ` ` in `tSC`, and `%SQLCODE` is -400 after the loop. Check only after execute and that error reads as an empty result.

## 5. Embedded SQL vs %SQL.Statement

```objectscript
// Embedded SQL — compiled into the method, faster but static:
&sql(SELECT Name INTO :name FROM Config.Setting WHERE Name = :key)
If SQLCODE = 100 { Return "" }    // not found
If SQLCODE < 0   { Throw ##class(%Exception.SQL).CreateFromSQLCODE(SQLCODE, %msg) }

// %SQL.Statement — dynamic, preferred for variable table/field names:
Set stmt = ##class(%SQL.Statement).%New()
Set tSC = stmt.%Prepare("SELECT Name FROM " _ tableName _ " WHERE Key = ?")
If $$$ISERR(tSC) { Return tSC }    // %Prepare returns the error; it does not throw
Set rs = stmt.%Execute(key)
```

Returning `""` on a negative SQLCODE hands the caller the not-found answer for an error. Throw it, or return a `%Status`, so the caller can tell them apart.

**-114 is a lock timeout, not a miss.** Under `READ COMMITTED`, a read of a row another process has locked waits for the lock, then fails with SQLCODE -114. Do not read the `INTO` variable after -114: on IRIS 2026.2 it held the locked row in some runs and came back empty in others, so code that checks only for 100 carries on with whichever it got. Treat -114 as an error like any other negative code.

## 6. IS NULL in IRIS SQL

```objectscript
// Empty string and NULL are different in IRIS SQL:
// Property stored as "" → IS NULL returns FALSE
// Property not set (null) → IS NULL returns TRUE
// For $HOROLOG dates, empty string "" stored as 0 or NULL depending on type

// Safe pattern for "no expiry date" meaning either null or empty:
"AND (ActiveUntil IS NULL OR ActiveUntil = '' OR ActiveUntil >= ?)"
```

## 7. ObjectScript Operators That Break Inside SQL Strings

```objectscript
// ObjectScript '= (not-equal) is INVALID inside SQL string literals.
// The ' character starts a SQL string, so '='' is parsed as a broken string.

// WRONG — causes SQLCODE: -3 "Closing quote missing":
Set sql = "SELECT Name FROM Items WHERE Tags '= ''"

// CORRECT — use SQL standard <> for not-equal in SQL strings:
Set sql = "SELECT Name FROM Items WHERE Tags <> ''"

// WRONG — %INLIST is SQL-only; causes ERROR #1010 in ObjectScript method code:
Return (tag %INLIST $ListFromString(..Tags, ","))

// CORRECT — use $ListFind in ObjectScript:
Return ($ListFind($ListFromString(..Tags, ","), tag) > 0)

// WRONG — $ZDATETIME is ObjectScript, not SQL. SQLCODE -12 "A term expected",
// with the parser's position marker at the $:  ... VALUES ( ? , $
Set sql = "INSERT INTO Demo.Todo (Title, Created) VALUES (?, $ZDATETIME($HOROLOG,3))"

// CORRECT — CURRENT_TIMESTAMP is the SQL spelling:
Set sql = "INSERT INTO Demo.Todo (Title, Created) VALUES (?, CURRENT_TIMESTAMP)"

// WRONG — double quotes delimit identifiers in SQL, so "hello" is read as a field name.
// SQLCODE -29 "Field 'HELLO' not found in the applicable tables":
Set sql = "SELECT Title FROM Demo.Todo WHERE Title = ""hello"""

// CORRECT — a SQL string takes single quotes:
Set sql = "SELECT Title FROM Demo.Todo WHERE Title = 'hello'"
```

A column whose property has an `InitialExpression` can simply be left out of the INSERT; see
`iris-sql`, "Key IRIS INSERT constraints".

## 8. $HOROLOG Date Arithmetic

```objectscript
Set today    = +$HOROLOG           // integer date (days since Dec 31, 1840)
Set tomorrow = today + 1
Set lastYear = today - 365

// Convert to/from display format:
Set display = $ZDATE(today, 3)     // "YYYY-MM-DD"
Set hDate   = $ZDATEH("2026-01-15", 3)  // back to $HOROLOG integer
```

## 9. Embedded SQL INTO Variables — What a SELECT Leaves Behind

```objectscript
// COUNT(*) always returns one row, so SQLCODE is 0 and tCount is set,
// even when nothing matches and tCount was never defined:
Kill tCount
&sql(SELECT COUNT(*) INTO :tCount FROM Bench.Patient WHERE Code = 'none')
If SQLCODE < 0 { Throw ##class(%Exception.SQL).CreateFromSQLCODE(SQLCODE, %msg) }
Write "count="_tCount  // outputs "count=0"

// A plain SELECT that finds no row sets SQLCODE 100 and sets each INTO variable to "":
Set tLabel = "untouched"
&sql(SELECT Label INTO :tLabel FROM Bench.Patient WHERE Code = 'none')
// SQLCODE = 100, tLabel = ""

// %SQL.Statement path:
Set stmt = ##class(%SQL.Statement).%New()
Set sc = stmt.%Prepare("SELECT COUNT(*) AS cnt FROM Bench.Patient")
Set rs = stmt.%Execute()
If rs.%Next() { Set tCount = rs.%Get("cnt") } Else { Set tCount = 0 }
```

**Why this trips up agents**: `SELECT COUNT(*) INTO :var` leaves `var` defined and 0 when no
rows match, whether or not it was set first. On SQLCODE 100 a plain SELECT overwrites its `INTO`
variables with `""`, so a default set before the query does not survive a miss. Check SQLCODE
rather than the variable. The error text is in `%msg`.

## 10. Debugging SQL Table Name Errors — Agent Red Herring Alert

When you see `SQLCODE: -30` ("Table or view not found") or the SQL appears to run
but returns no rows when rows are expected, **check the table name first**:

```objectscript
// The IRIS SQL table name is derived from the CLASS name — NOT the global name.
// Rule: last dot → schema/table separator; all preceding dots → underscores.

// Class: Bench.Patient → SQL table: Bench.Patient  (two-level: fine)
// Class: My.Deep.Patient → SQL table: My_Deep.Patient  (three-level: underscore!)

// WRONG — class is Bench.Patient but developer uses underscore:
&sql(SELECT COUNT(*) INTO :n FROM Bench_Patient)  // table not found or wrong table!

// CORRECT:
&sql(SELECT COUNT(*) INTO :n FROM Bench.Patient)

// Verify the correct SQL table name:
// SELECT SqlTableName FROM %Dictionary.CompiledClass WHERE Name = 'Bench.Patient'
```

**Diagnostic step when SQL returns unexpected results**:

1. Check `SELECT SqlTableName FROM %Dictionary.CompiledClass WHERE Name = ?`
2. Verify the table name in the SQL matches exactly
3. Check `SELECT * FROM Bench.Patient` (works) vs `SELECT * FROM Bench_Patient` (wrong)


