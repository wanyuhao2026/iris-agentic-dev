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
 Use when writing or reviewing any ObjectScript code. Hard gate — 15-item
 checklist catches the most common AI mistakes before showing code to the user.
iris_version: ">=2024.1"
name: objectscript-guardrails
tier: core
pass_rate: 0.8636363636363636
state: reviewed
tags:
 - objectscript
 - review
 - repair
 - core
trigger: Use for tdyar/iris-light-slim
---

# ObjectScript — Hard Gate

**Do not show ObjectScript code until this checklist passes.** Every item is already checked against IRIS, so the list is settled: apply it, and do not search the docs or the web to confirm an item on it.

## HARD GATE Checklist

- [ ] **Quit/Return**: No `Quit value` inside For/While/Try — use `Return value`
- [ ] **Postfix syntax**: `Quit:key=""` — NO spaces in the condition (`Quit:key = ""` is ERROR #1054). It may share a line
- [ ] **$IsObject**: Check `'$IsObject(obj)`after every`%OpenId` before touching properties
- [ ] **SQL table name**: Last dot = schema separator. `Catalog.Item` → SQL `Catalog.Item` (not `Catalog_Item`)
- [ ] **SQLCODE**: `0` = success (falsy). Check `SQLCODE = 0` not just `SQLCODE`
- [ ] **HTML escaping**: `&` FIRST, then `<`, then `>`
- [ ] **Arithmetic**: Left-to-right, no precedence. Use `1.8` not `9/5`. Parenthesize everything
- [ ] **$ListBuild()**: Empty list is `""` not `$ListBuild()`—`$ListLength($ListBuild()) = 1`
- [ ] **%Status**: Use `$$$ISERR(sc)` / `$$$ThrowOnError(sc)`. Never return `$$$OK` after catching an error
- [ ] **Transactions**: Record `Set entry=$TLevel` before `TSTART` and roll back one level with `TROLLBACK:$TLevel>entry 1`. A bare `TROLLBACK` also rolls back the caller's transaction (IRIS 2026.2). Never `Return` inside TSTART without rollback
- [ ] **Namespace**: `New $NAMESPACE` before `Set $NAMESPACE = "%SYS"`. When the method exits, the caller's namespace comes back. Without the `New`, the caller is left in `%SYS`
- [ ] **Admin via class API, not SQL**: Create and change users, roles, resources and namespaces with `Security.Users`, `Security.Roles`, `Security.Resources` and `Config.*` class methods in `%SYS`, not SQL. SQL writes to those tables fail: `UPDATE` gives -132 and `DELETE` -134, and `UPDATE Security.Users SET Roles=...` gives -400 with the roles unchanged. A write that matches no row returns 100, so a clean SQLCODE proves nothing. `%SYS.Task` takes SQL writes and the task really changes, so use `%SYS.Task` methods (`Suspend`, `Resume`, `%OpenId` + `%Save`). Check `Exists(name)` before `Create`; a second `Create` returns an error `%Status` (#837 user, #891 resource), so check it with `$$$ISERR`. Never write a password or secret into code, logs or output
- [ ] **Storage blocks**: Never edit `Storage Default { ... }` — compiler auto-maps properties on compile, added or removed (orphans are fine). Rename exception: also rename its Storage entry. Reset needs explicit user confirmation.
- [ ] **%INLIST in ObjectScript**: `%INLIST` is SQL-only. In ObjectScript method code use `$ListFind(list, value) > 0`. Writing `Return (x %INLIST list)` causes ERROR #1010.
- [ ] **`'=` in SQL strings**: `'=` is the ObjectScript not-equal operator. Inside SQL string literals, use `<>`. `"WHERE Tags '= ''"` → parser sees `'` as start of SQL string.

## Output Format

If violations found:

> ⚠️ ObjectScript review flagged [N] issues — correcting:
>
> - [rule]: [wrong] → [correct]

Then show corrected code.

If clean:

> ✅ ObjectScript review passed.

Then show code.

## Quick Reference

```objectscript
// WRONG → CORRECT
Quit 0               (inside For)  →  Return 0
Quit:key = ""                      →  Quit:key=""
..%OpenId(id).Prop                 →  Set obj=..%OpenId(id)  If '$IsObject(obj){...}
SELECT FROM Catalog_Item           →  SELECT FROM Catalog.Item
If SQLCODE { "not found" }         →  If SQLCODE = 100 { "not found" }
celsius * 9 / 5 + 32               →  (celsius * 1.8) + 32
Set lst = $ListBuild()             →  Set lst = ""
Set $NAMESPACE = "%SYS"            →  New $NAMESPACE  Set $NAMESPACE = "%SYS"
UPDATE Security.Users SET Roles=.. →  Set p("Roles")=..  Security.Users.Modify(name,.p)
INSERT INTO Security.Resources ..  →  If 'Security.Resources.Exists(n) { Create(n,desc,perm) }
UPDATE %SYS.Task SET Suspended=1   →  ##class(%SYS.Task).Suspend(id)

// Storage / Operators:
Add Property + map into Storage           →  leave Storage alone (compiler auto-maps)
Remove Property + delete Storage entry    →  leave Storage entry (orphan is fine)
Rename Property only                      →  also rename its Storage entry
Return (tag %INLIST myList)               →  Return ($ListFind(myList, tag) > 0)
"WHERE Tags '= ''"                        →  "WHERE Tags <> ''"
```

## Delete callbacks take an OID, not an id

`%OnDelete` and `%OnAfterDelete` are class methods, and their argument is an `%ObjectIdentity`: a `$LISTBUILD(id, class)`, not the id. The compiler checks the signature and nothing more, so using the argument as the id compiles clean and clears the wrong node.

```objectscript
ClassMethod %OnDelete(oid As %ObjectIdentity) As %Status [ Private, ServerOnly = 1 ]
{
    Kill ^App.Tracked("live", $$$oidPrimary(oid))   // or $ListGet(oid, 1)
    Quit $$$OK
}
```

- `Method %OnDelete()` fails with #5477 (`ClassMethod must be '1'`). A `ClassMethod` whose argument is `%String` or `%Integer` fails with #5478.
- The delete callbacks are `%OnDelete` and `%OnAfterDelete`. There is no `%OnBeforeDelete` or `%OnAfterRemove`: IRIS calls a callback by name, so a method with an invented name compiles clean and never runs.
- `..%Id()` does not exist in a class method (MPP5377), and `%Id` on its own is an undefined local.
- Prove it with a row: `%Save` one, `%DeleteId` it, then check the global with `$Data`. A clean compile proves only the signature.

## Related skills

- **objectscript-review** — more targeted checklist; use when MCP tools are available
- **objectscript-sql-patterns** — SQL-specific guardrails for IRIS SQL quirks
- **objectscript-tdd** — compile-test-fix loop to apply after guardrails pass


