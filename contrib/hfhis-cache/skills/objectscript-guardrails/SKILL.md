---
name: objectscript-guardrails
description: Use when writing, reviewing, or repairing ObjectScript on the DHC-APP namespace of the Caché 2016.2.3 instance through the cache-v1 tools — before cache_compile and before showing code. Hard language rules, a pre-compile diff checklist, and before/after fixes for Quit-in-loop, %OpenId, SQL date filters, postfix syntax and string building.
---

# ObjectScript Guardrails — Caché 2016.2.3, DHC-APP

Target: Caché 2016.2.3 (Build 907_11_20753U), namespace `DHC-APP`, Windows, web port 57772, Atelier REST v1 only. Apply every rule before you compile or present ObjectScript.

## 1. Rules

**R1 — Leave a method with `Return`, not `Quit value`.**
`Quit 1` inside a `For`/`While`/`Try` exits the loop, not the method; execution falls through and the method returns whatever the last statement leaves. Write `Return 1`. Keep bare `Quit` for breaking a loop.

**R2 — No spaces in a postfix condition.**
`Quit:key = ""` is a syntax error (ERROR #1054 Invalid expression): the space ends the postcondition. Write `Quit:key=""`. Never put a postcondition on another statement.

**R3 — There is no operator precedence; evaluation is strictly left to right.**
`3+3*2` is `12`. Parenthesise every compound expression, and write `1.8` rather than `9/5`. Concatenate strings with `_`, never `+`.

**R4 — Check `$IsObject` after every `%OpenId`.**
`%OpenId` returns `""` for a missing id, and `patient.DOB` on `""` raises `<INVALID OREF>`. Write `If '$IsObject(obj) { Return "" }` before touching a property.

**R5 — `SQLCODE = 0` is success, and `0` is falsy.**
`If SQLCODE { ... }` fires on success. Test `If SQLCODE = 0` for success and `If SQLCODE = 100` for no row. A write that matches no row also returns 100.

**R6 — Use the `%Status` macros, and never `Throw` a `%Status`.**
Check with `$$$ISERR(sc)` / `$$$ISOK(sc)`, raise with `$$$ThrowOnError(sc)`, never return `$$$OK` after catching an error. `Throw` wants an exception object:

```objectscript
Try { $$$ThrowOnError(..Work()) } Catch ex { Set sc = ex.AsStatus() }
```

**R7 — Roll back only your own transaction level.**
`Set entry=$TLevel` before `TStart`, then `TROLLBACK:$TLevel>entry 1`; a bare `TROLLBACK` also ends the caller's transaction. Never `Return` out of a `TStart` without a rollback or commit on that path.

**R8 — `New $NAMESPACE` before switching namespace.**
`New $NAMESPACE  Set $NAMESPACE="%SYS"`. Without the `New`, the caller stays in `%SYS` after your method returns.

**R9 — `New varname` is illegal in a method body** (ERROR #1038); procedure blocks already isolate scope, and `New` belongs in routines. Call same-class methods as `..Method()`, read properties as `..Prop` rather than the raw `i%Prop` slot.

**R10 — An empty list is `""`, not `$ListBuild()`.**
`$LISTLENGTH($LISTBUILD())` is `1`, so a "no results" list becomes one empty element. `$LISTLENGTH`, `$LIST`, `$LISTBUILD`, `$LISTFIND`, `$LISTNEXT` and `$LISTTOSTRING` all work on 2016.2.3; `$ISLIST`, `$LISTVALID` and `$preprocess` do not exist — do not use or recommend them.

**R11 — Wrap `%STARTSWITH` in parentheses after `NOT`.**
`WHERE NOT x %STARTSWITH y` is a syntax error here (SQLCODE -14). Write `WHERE NOT (x %STARTSWITH y)`.

**R12 — Inside a SQL string, `'=` is not "not equal".**
The parser reads `'` as the start of a literal. Write `"WHERE Tags <> ''"`, never `"WHERE Tags '= ''"`.

**R13 — `%INLIST` is SQL-only.**
`Return (tag %INLIST list)` is ERROR #1010. In method code write `Return ($ListFind(list, tag) > 0)`.

**R14 — Write Chinese literals directly; never re-encode them.**
`Write "病人"` round-trips correctly because the source is already UTF-8. `$ZCONVERT("病人","O","UTF8")` double-encodes it into mojibake — a live trap in this codebase. Verify with `cache_execute` running `Write "病人",!`.

**R15 — Escape HTML `&` first, then `<`, then `>`.**
Escaping `<` first yields `&lt;`, and the later `&` pass turns it into `&amp;lt;`.

**R16 — `%TimeStamp` literals use a space, not `T`.**
`2026-11-25 10:00:00`, never `2026-11-25T10:00:00`. Today as an integer is `+$HOROLOG`.

**R17 — Iterate backwards when you remove items.**
`For i=items.Count():-1:1` then `Do items.RemoveAt(i)`. Forward iteration skips the element that shifts into the removed slot.

**R18 — Do not concatenate in a loop.**
Each `result _ value` copies everything before it. Accumulate `Set lst = lst _ $LISTBUILD(value)` and convert once with `$LISTTOSTRING(lst, ",")`.

**R19 — Never edit `Storage Default { ... }`.**
The compiler maps added and removed properties itself; an orphan entry is fine. The one exception: renaming a property means renaming its storage entry too. A storage reset needs explicit user confirmation. Read storage with `cache_resolve_storage`; never patch it.

**R20 — Delete callbacks take an `%ObjectIdentity`, not an id.**
`%OnDelete` and `%OnAfterDelete` are `ClassMethod`s whose argument is `$LISTBUILD(id, class)`; use `$$$oidPrimary(oid)` or `$ListGet(oid,1)`. Using it as the id compiles clean and kills the wrong node. There is no `%OnBeforeDelete`, and `..%Id()` does not exist in a class method. Prove a delete with `cache_global` and `$Data`, not with the compiler.

### Tool rules

**T1 — `cache_doc` action=put saves but does not compile.** Always follow it with `cache_compile` on `Pkg.Class.cls`; the tool strips `.cls` for `$System.OBJ.Compile`. A successful put proves nothing.

**T2 — There is no code-body search.** `/action/search` returns 404 here, `$preprocess` is absent, and `%Dictionary.CompiledMethod.Implementation` / `^oddDEF` hold no method bodies. Find code with `cache_docnames` (`pattern="DHC.Pkg.*"`, category `CLS`) and read it with `cache_doc`. Never tell the user to search code bodies.

**T3 — Keep every `cache_query` bounded.** Heavy or cross-table SQL — a `LIKE` over `%Dictionary.*` took 39 seconds — can make the gateway answer with an HTML page instead of JSON. Use `TOP n` and a narrow `WHERE`. There is no explain mode on this build (SQLCODE -51).

**T4 — Tool output is German.** Server and compiler messages arrive localised (`FEHLER #5540`, `Kompilierung ... erfolgreich abgeschlossen`). Match on the stable `#NNNN` code and on `erfolgreich abgeschlossen`; never match English words.

**T5 — Every HTTP request holds a license unit until its CSP session ends**, and the instance has about 300. Never loop one tool call per class over thousands of classes. `cache_docnames` with category `CLS` returns all 34,197 classes in ~13 seconds — one request, but always pass `pattern`.

**T6 — When a call fails with no `#NNNN` and no obvious cause, run `cache_check_config`** to read the resolved connection and namespace before you conclude the code is wrong.

## 2. Review checklist — run before `cache_compile`

- [ ] Every `Return` value matches the declared return type, and no `Quit value` sits inside `For`/`While`/`Try`.
- [ ] Every `%OpenId` result is tested with `$IsObject` before a property is read.
- [ ] Every `..Method()` resolves: confirm the class with `cache_docnames pattern="Pkg.*"`, then `cache_doc`. A misspelled method name is a runtime error, not a compile error.
- [ ] Every `TStart` has a `TCommit` or `TROLLBACK` on every exit path, and `entry=$TLevel` is recorded before it.
- [ ] Every `$NAMESPACE` assignment is preceded by `New $NAMESPACE`.
- [ ] Every `FROM` clause uses the SQL table printed by `cache_table_info(className=...)`, never a name derived from the class. Business class `web.DHCBL.CT.CTLoc` (0 properties, logic) → persistent class `User.CTLoc` (103 properties) → table `SQLUser.CT_Loc`. `FROM User.CTLoc` fails with SQLCODE -1 (`USER` is a reserved word), and so do `FROM "User".CTLoc` and `FROM User_CTLoc`; 5,842 persistent classes here differ the same way.
- [ ] Every other SQL string: `<>` not `'=`, `NOT (x %STARTSWITH y)`, and `SQLCODE = 0` / `= 100` tests.
- [ ] Every query is bounded with `TOP n` or a selective `WHERE` before it reaches `cache_query`.
- [ ] Date filters read `(field IS NULL OR field >= ?)` with `+$HOROLOG`.
- [ ] No `$ISLIST`, `$LISTVALID`, `$preprocess`, `%INLIST` in method code, and no `$ZCONVERT(...,"O","UTF8")` on Chinese text.
- [ ] Added or removed properties left `Storage Default` untouched; a renamed property renamed its storage entry. Check with `cache_resolve_storage`.
- [ ] New persistent class: an index for each lookup the code performs, and `%TimeStamp` with a space.
- [ ] Loops that remove items iterate `Count():-1:1`; string building accumulates `$LISTBUILD` and converts once.
- [ ] Delete callbacks are `ClassMethod`s taking `%ObjectIdentity`, named `%OnDelete` or `%OnAfterDelete`.
- [ ] After the put you ran `cache_compile` and read the German output for `#NNNN`. Saved is not compiled, and compiled is not tested.

## 3. Fix catalogue

Name the pattern and state the cause in one sentence before you write the fix.

**1. `Quit` in a loop returns the wrong value**

```objectscript
// WRONG — exits the loop, then falls through to Quit 0
For i=1:1:names.Count() { If names.GetAt(i)=name { Quit 1 } }
Quit 0
// RIGHT — Return leaves the method
For i=1:1:names.Count() { If names.GetAt(i)=name { Return 0 } }
Return 1
```

**2. HTML escaping order double-encodes**

```objectscript
// WRONG — "&lt;" becomes "&amp;lt;"
Set safe = $REPLACE(msg, "<", "&lt;")
Set safe = $REPLACE(safe, "&", "&amp;")
// RIGHT — & first, then < and >
Set safe = $REPLACE(msg, "&", "&amp;")
Set safe = $REPLACE(safe, "<", "&lt;")
```

**3. Query returns expired rows**

```objectscript
// FROM takes the SQL table from cache_table_info(className=...), not the class name
// WRONG — ActiveUntil never checked
Set sc = stmt.%Prepare("SELECT Name FROM SQLUser.AppItem WHERE Category = ?")
Set rs = stmt.%Execute(category)
// RIGHT — NULL means never expires, and the row count is bounded
Set sc = stmt.%Prepare("SELECT TOP 100 Name FROM SQLUser.AppItem WHERE Category = ? AND (ActiveUntil IS NULL OR ActiveUntil >= ?)")
Set rs = stmt.%Execute(category, +$HOROLOG)
```

**4. Forward iteration with removal skips items**

```objectscript
// WRONG — after RemoveAt(2) the old item 3 shifts into slot 2 and is skipped
For i=1:1:items.Count() { If items.GetAt(i)["DELETE" { Do items.RemoveAt(i) } }
// RIGHT
For i=items.Count():-1:1 { If items.GetAt(i)["DELETE" { Do items.RemoveAt(i) } }
```

**5. Missing `$IsObject` after `%OpenId`**

```objectscript
// WRONG — <INVALID OREF> when the id does not exist
Set patient = ##class(Patient).%OpenId(id)
Set birthYear = $PIECE(patient.DOB, "-", 1)
// RIGHT
Set patient = ##class(Patient).%OpenId(id)
If '$IsObject(patient) { Return -1 }
Set birthYear = $PIECE(patient.DOB, "-", 1)
```

**6. Postfix condition with spaces**

```objectscript
// WRONG — ERROR #1054 Invalid expression
Quit:key = ""
// RIGHT
Quit:key=""
```

**7. CSV built by O(n²) concatenation**

```objectscript
// WRONG — each _ copies the whole string so far
Set result = ""
For i=1:1:values.Count() { Set result = result _ "," _ values.GetAt(i) }
// RIGHT — accumulate a $LIST, convert once
Set lst = ""
For i=1:1:values.Count() { Set lst = lst _ $LISTBUILD(values.GetAt(i)) }
Return $LISTTOSTRING(lst, ",")
```
