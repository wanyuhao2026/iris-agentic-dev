//! Caché-family compatibility layer (Caché / Ensemble / HealthShare, 2016.x–2018.x).
//!
//! These products serve Atelier API v1 only, and two v1 gaps break core tools
//! (all verified against a live Caché 2016.2.3):
//!
//! • `/action/search` always returns `{"result": {}}` — a content grep that
//!   reports nothing regardless of matches. `iris_search` falls back to the
//!   SQL data dictionary (`%Dictionary.ClassDefinition`, `MethodDefinition`,
//!   `PropertyDefinition`), matching the query against class, method, and
//!   property names.
//! • `/docnames/CLS` times out (HTTP 504) on large namespaces — a 51k-class
//!   namespace never answers. The class list falls back to
//!   `SELECT Name FROM %Dictionary.ClassDefinition WHERE Name LIKE …`, which
//!   answers the same question in under half a second.
//! • `%UnitTest.Manager` differs from IRIS's enough that `iris_test`'s
//!   IRIS-shaped `RunTest("Pkg","/verbose=1/nodelete/noload")` call never runs
//!   a single test on Caché: `/verbose` does not exist (错误 #5001 kills the
//!   run), and a bare class name in testspec names a *directory* under
//!   `^UnitTestRoot`, not a class. The fix — colon-syntax testspec, one
//!   `RunTest` per class, `/displaylog` — lives in
//!   [`build_cache_test_run_code`]; `iris_test` calls it after resolving the
//!   pattern through the SQL dictionary.
//!
//! v1 also uses different routine categories: RTN covers .mac/.int/.inc, and
//! MAC/INT/INC are not v1 categories at all. Worse, both `/docnames/RTN` and
//! `/docnames/CLS` time out on large namespaces, so `docnames_entries` answers
//! both from SQL instead: classes from `%Dictionary.ClassDefinition`, routines
//! from `%Library.RoutineIndex` filtered by its `Type` column.
//!
//! Everything here is gated on `IrisConnection::product.is_cache_family()`;
//! IRIS instances never touch this module.

use crate::iris::connection::IrisConnection;

/// POST one SQL statement to Atelier `/action/query` and return `result.content`
/// rows.
///
/// Deliberately a bare `{"query": …}` body rather than going through
/// [`IrisConnection::query`]: every fallback query here was validated with
/// exactly that shape against Caché 2016.2.3's v1 endpoint, and this layer has
/// no use for bind parameters.
async fn run_sql(
    iris: &IrisConnection,
    client: &reqwest::Client,
    namespace: &str,
    sql: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let url = iris.versioned_ns_url(namespace, "/action/query");
    let resp = client
        .post(&url)
        .basic_auth(&iris.username, Some(&iris.password))
        .json(&serde_json::json!({ "query": sql }))
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {} for {}", resp.status(), url));
    }
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("response decode failed: {e}"))?;
    // Atelier reports SQL errors as 200 OK with status.errors populated.
    if let Some(errs) = body["status"]["errors"].as_array() {
        if let Some(first) = errs.first() {
            let msg = first["error"].as_str().unwrap_or("Atelier query error");
            return Err(msg.to_string());
        }
    }
    Ok(body["result"]["content"]
        .as_array()
        .cloned()
        .unwrap_or_default())
}

/// Escape a single-quoted SQL string literal.
fn sql_quote(s: &str) -> String {
    s.replace('\'', "''")
}

/// Convert a document glob (`DHC.LIS*`, `M.*.cls`) to a SQL `LIKE` pattern:
/// `*` → `%`, `?` → `_`.
///
/// Not exact: `_` in the input is literal to the glob but a wildcard to LIKE,
/// and LIKE's default SQLUPPER collation is case-insensitive. Callers keep
/// applying their own regex afterwards — the LIKE is a volume pre-filter that
/// keeps `SELECT Name` from shipping every class in the namespace, never the
/// final word on what matches.
pub fn glob_to_sql_like(glob: &str) -> String {
    let mut out = String::new();
    for ch in glob.chars() {
        match ch {
            '*' => out.push('%'),
            '?' => out.push('_'),
            c => out.push(c),
        }
    }
    out
}

/// Upper bound on classes one `iris_test` call chains on a Caché-family server.
///
/// Caché needs one `RunTest` per class (see [`build_cache_test_run_code`]), all
/// inside a single executor snippet — a glob matching half the namespace would
/// turn one tool call into a very long-running one. A pattern that broad is
/// almost certainly a mistake, so it is an error rather than a truncation.
pub const CACHE_TEST_CLASS_CAP: usize = 50;

/// Build the `%UnitTest.Manager` invocation for a Caché-family server: one
/// `RunTest` per class, chained in a single snippet.
///
/// Caché's Manager differs from IRIS's in three ways that each break the
/// IRIS-shaped bare-name call (all verified against a live Caché 2016.2.3):
///
/// • `/verbose` does not exist — `RunTest("X","/verbose=1/…")` dies with
///   `错误 #5001: qualifier '/verbose' does not exist` before any test runs,
///   leaving no stdout to parse. `/displaylog` is the Caché switch that gates
///   every `PrintLine` (the IRIS `/verbose` equivalent).
/// • A bare class name in testspec names a DIRECTORY under `^UnitTestRoot`
///   (`$tr(testsuite,"/.","\")` then `AddSubDirectoryNames`). No XML there →
///   zero classes run → the run still ends "All PASSED". The colon form
///   `":Package.Class"` puts the class straight into `classLoaded`, no
///   filesystem involved.
/// • Each colon entry resolves to the `^UnitTestRoot` directory itself, and
///   `RunTestSuites` dedupes on that directory (`haverun(dir)`), so a second
///   colon entry inside one call is silently skipped. Hence one call per class.
///
/// The flags are the caller's business, but they must include `/norecursive`:
/// with recursion on, `GetSubDirectories` only registers a directory that holds
/// XML files directly, and `^UnitTestRoot` holds none — so even a colon entry
/// never reaches `RunOneTestSuite`. `/norecursive` registers `^UnitTestRoot`
/// unconditionally, which is where the injected class then runs. This is the
/// fourth way the IRIS-shaped flags (`/verbose=1/nodelete/noload`) fail:
/// `/verbose` errors out, and the `/noload` without `/norecursive` runs nothing.
///
/// `classes` comes from the SQL dictionary ([`class_names_like`]); the Manager
/// itself skips anything that does not extend `%UnitTest.TestCase`, so a glob
/// matching non-test classes costs time but never correctness.
pub fn build_cache_test_run_code(classes: &[String], flags: &str, token: &str) -> String {
    classes
        .iter()
        .map(|c| {
            format!(
                r#"do ##class(%UnitTest.Manager).RunTest(":{c}","{flags}","{token}")"#,
                c = c.replace('"', "\\\""),
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A class-scope glob (`MyPkg.*.cls`, `DHC.LIS*`) to a LIKE over
/// `%Dictionary.ClassDefinition.Name` — which stores bare class names with no
/// `.cls` suffix, so a trailing extension is stripped first.
pub fn class_glob_to_like(glob: &str) -> String {
    let g = glob.strip_suffix(".cls").unwrap_or(glob);
    glob_to_sql_like(g)
}

/// Leading-literal-prefix hint for [`class_glob_to_like`], from a caller's
/// filter pattern. `Region.**.*.cls` → `Region.*`, `M.*` → `M.*`,
/// `M.Test.cls` → `M.Test.cls`, `*Foo*` → `*`.
///
/// The prefix is only a volume pre-filter — the caller's own regex still makes
/// the final match — so a prefix that over-matches (the `*` fallback when the
/// pattern starts with a wildcard) is correct, just slower. A pattern with no
/// wildcard is passed through unchanged so an exact lookup stays exact.
pub fn glob_prefix_hint(pattern: &str) -> String {
    let prefix: String = pattern
        .chars()
        .take_while(|&c| c != '*' && c != '?')
        .collect();
    if prefix.len() == pattern.len() {
        // No wildcard anywhere — an exact name.
        return pattern.to_string();
    }
    format!("{prefix}*")
}

/// Class names from the SQL data dictionary, pre-filtered by a LIKE pattern.
///
/// The replacement for `/docnames/CLS` on Caché-family instances, where that
/// endpoint times out (504) on large namespaces. Measured: full count over
/// 51k classes answers in ~0.4s, a `LIKE 'DHC%'` filter in ~0.06s.
pub async fn class_names_like(
    iris: &IrisConnection,
    client: &reqwest::Client,
    namespace: &str,
    like: &str,
) -> Result<Vec<String>, String> {
    let sql = format!(
        "SELECT Name FROM %Dictionary.ClassDefinition WHERE Name LIKE '{}'",
        sql_quote(like)
    );
    let rows = run_sql(iris, client, namespace, &sql).await?;
    Ok(rows
        .iter()
        .filter_map(|r| r["Name"].as_str().map(str::to_string))
        .collect())
}

/// `/docnames/RTN` — Caché v1's routine category, covering .mac/.int/.inc —
/// replaced by a `%Library.RoutineIndex` query, filtered by `Type`.
///
/// The RTN endpoint itself also times out (504) on large namespaces — the
/// same failure as `/docnames/CLS` (DHC-APP's 65k MAC routines never answer
/// within the Web Gateway's 60s cutoff, while the SQL COUNT returns
/// instantly). `RoutineIndex` stores bare names (`AddIVItem`, no `.mac`), so
/// the caller's extension is appended back on; `Modified` fills the `ts`
/// field the docnames shape carries.
async fn rtn_docnames_filtered(
    iris: &IrisConnection,
    client: &reqwest::Client,
    namespace: &str,
    ext: &str,
    like: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let sql = format!(
        "SELECT Name, Modified FROM %Library.RoutineIndex \
         WHERE Type='{}' AND Name LIKE '{}'",
        ext,
        sql_quote(like)
    );
    let rows = run_sql(iris, client, namespace, &sql).await?;
    let ext_lower = ext.to_ascii_lowercase();
    Ok(rows
        .iter()
        .filter_map(|r| {
            let name = r["Name"].as_str()?;
            Some(serde_json::json!({
                "name": format!("{name}.{ext_lower}"),
                "cat": ext,
                "ts": r["Modified"].clone(),
            }))
        })
        .collect())
}

/// Strip a trailing suffix, ASCII case-insensitively.
///
/// A caller's exact-name pattern arrives with its extension (`"M.Test.mac"`)
/// while `RoutineIndex` stores the bare name — the extension must not leak
/// into the LIKE or the pattern matches nothing.
fn strip_suffix_ci<'a>(s: &'a str, suffix: &str) -> &'a str {
    if s.len() >= suffix.len() && s[s.len() - suffix.len()..].eq_ignore_ascii_case(suffix) {
        &s[..s.len() - suffix.len()]
    } else {
        s
    }
}

/// A routine-scope glob (`M.*.mac`, `AddIVItem.mac`) to a LIKE over
/// `%Library.RoutineIndex.Name` — which stores bare names with the category in
/// `Type`, so a trailing extension is stripped first (case-insensitively;
/// category spellings arrive uppercase from callers but globs may not be).
pub fn routine_glob_to_like(glob: &str) -> String {
    let bare = [".mac", ".int", ".inc", ".bas"]
        .iter()
        .fold(glob, |g, e| strip_suffix_ci(g, e));
    glob_to_sql_like(bare)
}

/// One SQL-dictionary match for the `iris_search` Caché fallback.
#[derive(Debug)]
pub struct DictMatch {
    /// Document name with extension, e.g. "DHC.LIS.Utils.cls".
    pub document: String,
    /// Member name; empty for a class-name match.
    pub member: String,
    /// "class", "method", or "property".
    pub kind: &'static str,
}

/// SQL data-dictionary search — the Caché fallback for `/action/search`, which
/// returns an empty result on Atelier v1 regardless of matches.
///
/// Matches the query against class names, method names, and property names
/// (`Name [ 'query'`; the `[` contains operator under SQLUPPER collation is
/// case-insensitive), each restricted to the caller's document scope via
/// `Name LIKE` / `Parent LIKE`. `case_sensitive` is re-checked client-side
/// with an exact-case substring because the server-side contains is always
/// case-insensitive. A `regex` query is treated as a plain substring — the
/// dictionary has no regex support, and the response says so.
pub async fn dictionary_search(
    iris: &IrisConnection,
    client: &reqwest::Client,
    namespace: &str,
    query: &str,
    scope_like: &str,
    case_sensitive: bool,
    limit: usize,
) -> Result<Vec<DictMatch>, String> {
    let mut out: Vec<DictMatch> = Vec::new();
    let q = sql_quote(query);
    let scope = sql_quote(scope_like);

    // Class-name matches.
    let sql = format!(
        "SELECT TOP {limit} Name FROM %Dictionary.ClassDefinition \
         WHERE Name LIKE '{scope}' AND Name [ '{q}'"
    );
    for row in run_sql(iris, client, namespace, &sql).await? {
        let Some(name) = row["Name"].as_str() else {
            continue;
        };
        if case_sensitive && !name.contains(query) {
            continue;
        }
        out.push(DictMatch {
            document: format!("{name}.cls"),
            member: String::new(),
            kind: "class",
        });
    }

    // Member matches: methods and properties, attributed to their parent class.
    for (table, kind) in [
        ("%Dictionary.MethodDefinition", "method"),
        ("%Dictionary.PropertyDefinition", "property"),
    ] {
        let sql = format!(
            "SELECT TOP {limit} Parent, Name FROM {table} \
             WHERE Parent LIKE '{scope}' AND Name [ '{q}'"
        );
        for row in run_sql(iris, client, namespace, &sql).await? {
            // Caché returns the Parent column as "parent" in the result JSON;
            // accept both spellings.
            let parent = row["Parent"].as_str().or_else(|| row["parent"].as_str());
            let (Some(parent), Some(name)) = (parent, row["Name"].as_str()) else {
                continue;
            };
            if case_sensitive && !name.contains(query) {
                continue;
            }
            out.push(DictMatch {
                document: format!("{parent}.cls"),
                member: name.to_string(),
                kind,
            });
        }
        if out.len() >= limit {
            break;
        }
    }
    out.truncate(limit);
    Ok(out)
}

/// Caché-family `/docnames` replacement: CLS from the SQL dictionary, and
/// MAC/INT/INC — not v1 categories — from `%Library.RoutineIndex` filtered by
/// its `Type` column. Returns entries in the docnames shape (`{name, cat, ts}`)
/// so callers parse them exactly as they parse the real endpoint's output.
///
/// `glob_hint` narrows both queries (e.g. `"MyPkg.*"`); pass `"*"` for the
/// unfiltered list. `ts` is null in CLS entries — the dictionary's
/// `TimeChanged` is a raw $HOROLOG pair and fetching it doubles the row size
/// for a field no current caller reads — but routine entries carry
/// `RoutineIndex.Modified` verbatim, matching the real endpoint's `ts`.
pub async fn docnames_entries(
    iris: &IrisConnection,
    client: &reqwest::Client,
    namespace: &str,
    cat: &str,
    glob_hint: &str,
) -> Result<Vec<serde_json::Value>, String> {
    match cat {
        "CLS" => {
            let like = class_glob_to_like(glob_hint);
            let names = class_names_like(iris, client, namespace, &like).await?;
            Ok(names
                .into_iter()
                .map(|n| {
                    serde_json::json!({
                        "name": format!("{n}.cls"),
                        "cat": "CLS",
                        "ts": serde_json::Value::Null,
                    })
                })
                .collect())
        }
        "MAC" | "INT" | "INC" => {
            // RoutineIndex stores bare names with a Type column, so the
            // category IS the Type filter — and the entries already come back
            // stamped with the caller's category, no rewriting needed.
            let like = routine_glob_to_like(glob_hint);
            rtn_docnames_filtered(iris, client, namespace, cat, &like).await
        }
        other => Err(format!(
            "category '{other}' is not available on Caché (Atelier v1 serves CLS and RTN only)"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── glob_to_sql_like ───────────────────────────────────────────────────────
    #[test]
    fn glob_star_becomes_percent() {
        assert_eq!(glob_to_sql_like("M.*.cls"), "M.%.cls");
        assert_eq!(glob_to_sql_like("DHC.LIS*"), "DHC.LIS%");
    }

    #[test]
    fn glob_question_mark_becomes_underscore() {
        assert_eq!(glob_to_sql_like("Foo?.mac"), "Foo_.mac");
    }

    #[test]
    fn glob_double_star_is_redundant_but_harmless() {
        // Atelier scopes like "Region.**.*.cls" — each '*' maps independently.
        assert_eq!(glob_to_sql_like("Region.**.*.cls"), "Region.%%.%.cls");
    }

    #[test]
    fn glob_plain_string_passes_through() {
        assert_eq!(glob_to_sql_like("M.Test.cls"), "M.Test.cls");
    }

    // ── class_glob_to_like ─────────────────────────────────────────────────────
    #[test]
    fn class_glob_strips_cls_suffix() {
        // The dictionary stores bare class names — "MyPkg.Foo", not
        // "MyPkg.Foo.cls" — so the extension must not leak into the LIKE.
        assert_eq!(class_glob_to_like("MyPkg.*.cls"), "MyPkg.%");
    }

    #[test]
    fn class_glob_without_suffix_unchanged() {
        assert_eq!(class_glob_to_like("DHC.LIS*"), "DHC.LIS%");
    }

    #[test]
    fn class_glob_bare_wildcard_matches_everything() {
        assert_eq!(class_glob_to_like("*"), "%");
    }

    #[test]
    fn class_glob_exact_name_is_exact_like() {
        // A wildcard-free pattern still arrives here from callers that don't
        // pre-filter; it must become a literal LIKE, not "%".
        assert_eq!(class_glob_to_like("M.Test.cls"), "M.Test");
    }

    // ── glob_prefix_hint ───────────────────────────────────────────────────────
    #[test]
    fn prefix_hint_takes_leading_literal() {
        assert_eq!(glob_prefix_hint("Region.**.*.cls"), "Region.*");
        assert_eq!(glob_prefix_hint("M.*.cls"), "M.*");
    }

    #[test]
    fn prefix_hint_no_wildcard_is_exact() {
        // An exact-name lookup must stay exact — passing it through unchanged
        // keeps the LIKE from broadening to a suffix match.
        assert_eq!(glob_prefix_hint("M.Test.cls"), "M.Test.cls");
    }

    #[test]
    fn prefix_hint_leading_wildcard_falls_back_to_all() {
        assert_eq!(glob_prefix_hint("*Foo*"), "*");
    }

    #[test]
    fn prefix_hint_question_mark_also_cuts_the_prefix() {
        // '?' is a wildcard just like '*'; the prefix stops before it.
        assert_eq!(glob_prefix_hint("DHC.LI?*"), "DHC.LI*");
    }

    // ── routine_glob_to_like ───────────────────────────────────────────────────
    #[test]
    fn routine_glob_strips_extension_case_insensitively() {
        assert_eq!(routine_glob_to_like("M.*.mac"), "M.%");
        assert_eq!(routine_glob_to_like("M.*.MAC"), "M.%");
    }

    #[test]
    fn routine_glob_strips_int_inc_and_bas_too() {
        assert_eq!(routine_glob_to_like("AddIVItem.int"), "AddIVItem");
        assert_eq!(routine_glob_to_like("M.*.inc"), "M.%");
        assert_eq!(routine_glob_to_like("weird.bas"), "weird");
    }

    #[test]
    fn routine_glob_bare_name_stays_bare() {
        // A bare name (no extension) must pass through — RoutineIndex stores
        // bare names, so "AddIVItem" is already in the right shape.
        assert_eq!(routine_glob_to_like("AddIVItem"), "AddIVItem");
    }

    // ── strip_suffix_ci ────────────────────────────────────────────────────────
    #[test]
    fn strip_suffix_ci_exact_and_case_variants() {
        assert_eq!(strip_suffix_ci("M.Test.mac", ".mac"), "M.Test");
        assert_eq!(strip_suffix_ci("M.Test.MAC", ".mac"), "M.Test");
        assert_eq!(strip_suffix_ci("M.Test", ".mac"), "M.Test");
        // The suffix alone must not panic — len check holds, result is empty.
        assert_eq!(strip_suffix_ci(".mac", ".mac"), "");
    }

    // ── build_cache_test_run_code ───────────────────────────────────────────────
    #[test]
    fn cache_run_code_single_class_uses_colon_syntax() {
        // The colon form is the whole point: ":Package.Class" injects the class
        // straight into classLoaded instead of naming a ^UnitTestRoot directory.
        let code = build_cache_test_run_code(
            &["Test.McpDemoTest".to_string()],
            "/noload/nodelete/norecursive/displaylog",
            "tok1",
        );
        assert_eq!(
            code,
            r#"do ##class(%UnitTest.Manager).RunTest(":Test.McpDemoTest","/noload/nodelete/norecursive/displaylog","tok1")"#
        );
    }

    #[test]
    fn cache_run_code_chains_one_call_per_class() {
        // haverun(dir) dedupes colon entries to the same ^UnitTestRoot directory,
        // so two classes in one RunTest would silently skip the second.
        let code = build_cache_test_run_code(
            &["A.One".to_string(), "A.Two".to_string()],
            "/noload/nodelete/norecursive/displaylog",
            "tok2",
        );
        assert_eq!(code.matches("RunTest").count(), 2, "code: {code}");
        assert!(code.contains(r#"RunTest(":A.One""#), "code: {code}");
        assert!(code.contains(r#"RunTest(":A.Two""#), "code: {code}");
    }

    #[test]
    fn cache_run_code_empty_class_list_is_empty_snippet() {
        // The caller checks for empty before invoking; an empty slice here must
        // not produce a dangling RunTest with an empty testspec.
        assert_eq!(build_cache_test_run_code(&[], "/noload", "t"), "");
    }

    #[test]
    fn cache_run_code_escapes_embedded_quotes() {
        let code = build_cache_test_run_code(&[r#"We"ird"#.to_string()], "/noload", "t");
        assert!(
            code.contains(r#"RunTest(":We\"ird""#),
            "quote must be escaped: {code}"
        );
    }

    // ── sql_quote ──────────────────────────────────────────────────────────────
    #[test]
    fn sql_quote_doubles_single_quotes() {
        assert_eq!(sql_quote("a'b"), "a''b");
    }

    #[test]
    fn sql_quote_plain_string_unchanged() {
        assert_eq!(sql_quote("DHC.LIS"), "DHC.LIS");
    }

    #[test]
    fn sql_quote_empty_stays_empty() {
        assert_eq!(sql_quote(""), "");
    }
}
