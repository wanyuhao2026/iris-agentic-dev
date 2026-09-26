//! Live integration tests for the Caché-family compatibility layer
//! (`tools::cache_compat`) — run against a real Caché 2016.2.3 instance.
//!
//! All tests are `#[ignore]`d and gated on `IAD_CACHE_TEST_HOST`, so they run
//! only when an operator points them at a Caché-family server. Run with:
//!
//! ```text
//! IAD_CACHE_TEST_HOST=10.146.8.17 \
//!   cargo test --features testing --test integration -- --include-ignored --test-threads=1 cache_compat
//! ```
//!
//! Defaults match the instance this layer was built against: port 57772,
//! `_SYSTEM`/`SYS`, namespace `DHC-APP` (51k classes, 4.8k `User.*` classes —
//! big enough that the `/docnames/CLS` 504 is reproducible, which is the whole
//! reason the SQL fallback exists). Override via `IAD_CACHE_TEST_PORT`,
//! `IAD_CACHE_TEST_USER`, `IAD_CACHE_TEST_PASSWORD`, `IAD_CACHE_TEST_NS`.
//!
//! No IRIS container is involved: these tests must never touch
//! `iris-dev-iris`, whose product is IRIS and whose paths never reach
//! `cache_compat` at all.

use iris_agentic_dev_core::iris::connection::{DiscoverySource, IrisConnection};
use iris_agentic_dev_core::tools::cache_compat;
use iris_agentic_dev_core::tools::log_store::LogStore;
use iris_agentic_dev_core::tools::search::{handle_iris_search, SearchParams};
use iris_agentic_dev_core::tools::IrisTools;
use std::sync::{Arc, Mutex};

fn make_cache_conn() -> Option<IrisConnection> {
    let host = std::env::var("IAD_CACHE_TEST_HOST").unwrap_or_default();
    if host.is_empty() {
        return None;
    }
    let port = std::env::var("IAD_CACHE_TEST_PORT").unwrap_or_else(|_| "57772".to_string());
    let user = std::env::var("IAD_CACHE_TEST_USER").unwrap_or_else(|_| "_SYSTEM".to_string());
    let password = std::env::var("IAD_CACHE_TEST_PASSWORD").unwrap_or_else(|_| "SYS".to_string());
    let base_url = format!("http://{host}:{port}");
    let ns = std::env::var("IAD_CACHE_TEST_NS").unwrap_or_else(|_| "DHC-APP".to_string());
    Some(IrisConnection::new(
        base_url,
        ns,
        user,
        password,
        DiscoverySource::EnvVar,
    ))
}

/// Connection after the Atelier root probe — `product` and `atelier_version`
/// are only correct once `probe()` has run, exactly as in the server startup.
async fn probed_conn() -> Option<IrisConnection> {
    let mut iris = make_cache_conn()?;
    iris.probe().await;
    Some(iris)
}

const NS: &str = "DHC-APP";

// ── probe / product identification ──────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn probe_identifies_cache_product_and_atelier_v1() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    assert_eq!(iris.product.as_str(), "cache", "product after probe");
    assert_eq!(
        iris.atelier_version,
        iris_agentic_dev_core::iris::connection::AtelierVersion::V1,
        "Caché 2016.2.3 serves Atelier v1"
    );
    assert!(iris.product.is_cache_family(), "Caché must be cache-family");
    assert!(
        iris.version
            .as_deref()
            .unwrap_or_default()
            .starts_with("Cache"),
        "version string: {:?}",
        iris.version
    );
}

// ── class_names_like (the /docnames/CLS 504 fallback) ───────────────────────

#[tokio::test]
#[ignore]
async fn class_names_like_filters_by_prefix() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    let names = cache_compat::class_names_like(&iris, &client, NS, "User%")
        .await
        .expect("dictionary query should succeed");
    assert!(
        names.len() > 1000,
        "DHC-APP has ~4.8k User.* classes, got {}",
        names.len()
    );
    assert!(
        names.iter().all(|n| n.starts_with("User")),
        "LIKE pre-filter must hold"
    );
    assert!(
        names.iter().all(|n| !n.ends_with(".cls")),
        "dictionary names are bare — no .cls suffix"
    );
}

// ── dictionary_search (the /action/search empty-result fallback) ────────────

#[tokio::test]
#[ignore]
async fn dictionary_search_finds_property_and_method_names() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    let matches =
        cache_compat::dictionary_search(&iris, &client, NS, "ACCTSubj", "User%", false, 50)
            .await
            .expect("dictionary search should succeed");
    assert!(
        !matches.is_empty(),
        "User.ACCTSubj has properties containing 'ACCTSubj'"
    );
    assert!(
        matches.len() <= 50,
        "limit must be respected, got {}",
        matches.len()
    );
    for m in &matches {
        assert!(m.document.ends_with(".cls"), "doc: {}", m.document);
        assert!(
            matches!(m.kind, "class" | "method" | "property"),
            "kind: {}",
            m.kind
        );
        if m.kind == "property" {
            assert!(
                m.member.contains("ACCTSubj") || m.document.contains("ACCTSubj"),
                "member {} in {}",
                m.member,
                m.document
            );
        }
    }
    // At least one property match attributed to the parent class — the
    // lowercase-`parent` JSON spelling this layer tolerates.
    assert!(
        matches
            .iter()
            .any(|m| m.kind == "property" && m.document.starts_with("User.")),
        "expected property matches, got: {matches:?}"
    );
}

#[tokio::test]
#[ignore]
async fn dictionary_search_case_sensitive_filters_client_side() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    // "acctsubj" (all lowercase) matches server-side under SQLUPPER; with
    // case_sensitive=true only members/documents containing the exact-cased
    // substring survive the client-side re-check.
    let matches =
        cache_compat::dictionary_search(&iris, &client, NS, "acctsubj", "User%", true, 50)
            .await
            .expect("dictionary search should succeed");
    for m in &matches {
        let hit_exact_case = m.member.contains("acctsubj") || m.document.contains("acctsubj");
        assert!(
            hit_exact_case,
            "case-sensitive match must contain the exact query: {m:?}"
        );
    }
}

// ── docnames_entries (category mapping) ─────────────────────────────────────

#[tokio::test]
#[ignore]
async fn docnames_cls_from_dictionary_has_docnames_shape() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    let entries = cache_compat::docnames_entries(&iris, &client, NS, "CLS", "User.ACCT*")
        .await
        .expect("CLS fallback should succeed");
    assert!(!entries.is_empty(), "User.ACCT* classes exist in DHC-APP");
    for e in &entries {
        let name = e["name"].as_str().unwrap_or_default();
        assert!(name.starts_with("User.ACCT"), "name: {name}");
        assert!(name.ends_with(".cls"), "name: {name}");
        assert_eq!(e["cat"].as_str(), Some("CLS"));
        // ts is null by design — TimeChanged is a raw $HOROLOG pair nobody reads.
        assert!(e["ts"].is_null(), "ts should be null: {e}");
    }
}

#[tokio::test]
#[ignore]
async fn docnames_inc_maps_rtn_category_by_extension() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    let entries = cache_compat::docnames_entries(&iris, &client, NS, "INC", "*")
        .await
        .expect("INC fallback should succeed");
    assert!(!entries.is_empty(), "DHC-APP has .inc files");
    for e in &entries {
        let name = e["name"].as_str().unwrap_or_default();
        assert!(name.to_ascii_lowercase().ends_with(".inc"), "name: {name}");
        // The v1 bucket (RTN) must be rewritten to the category the caller asked for.
        assert_eq!(e["cat"].as_str(), Some("INC"), "entry: {e}");
    }
}

#[tokio::test]
#[ignore]
async fn docnames_rejects_category_absent_on_v1() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    // BAS is an IRIS-only category; on v1 the fallback must say so rather than
    // return an empty list that reads as "no documents".
    let err = cache_compat::docnames_entries(&iris, &client, NS, "BAS", "*")
        .await
        .expect_err("BAS must be rejected");
    assert!(err.contains("BAS"), "error: {err}");
    assert!(err.contains("v1") || err.contains("Caché"), "error: {err}");
}

// ── iris_search end-to-end through the fallback ─────────────────────────────

#[tokio::test]
#[ignore]
async fn iris_search_returns_dictionary_hits_on_cache() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    let store = Arc::new(Mutex::new(LogStore::new(200, 60)));
    let p: SearchParams = serde_json::from_value(serde_json::json!({
        "query": "ACCTSubj",
        "documents": ["User.*.cls"],
        "namespace": NS,
        "inline": true,
    }))
    .unwrap();
    let result = handle_iris_search(&iris, &client, p, store)
        .await
        .expect("search should not error");
    let text = result.content[0].as_text().unwrap().text.clone();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["success"].as_bool(), Some(true), "body: {text}");
    let total = v["total_found"].as_u64().unwrap_or(0);
    assert!(total > 0, "User.* classes define ACCTSubj members: {text}");
    let results = v["results"].as_array().unwrap();
    for r in results {
        let doc = r["document"].as_str().unwrap_or_default();
        assert!(doc.starts_with("User."), "doc: {doc}");
        assert!(doc.ends_with(".cls"), "doc: {doc}");
    }
}

#[tokio::test]
#[ignore]
async fn iris_search_routine_category_says_what_it_cannot_do() {
    let Some(iris) = probed_conn().await else {
        return;
    };
    let client = reqwest::Client::new();
    let store = Arc::new(Mutex::new(LogStore::new(200, 60)));
    let p: SearchParams = serde_json::from_value(serde_json::json!({
        "query": "anything",
        "documents": ["M.*.mac"],
        "category": "MAC",
        "namespace": NS,
        "inline": true,
    }))
    .unwrap();
    let result = handle_iris_search(&iris, &client, p, store)
        .await
        .expect("search should not error");
    let text = result.content[0].as_text().unwrap().text.clone();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["success"].as_bool(), Some(true), "body: {text}");
    assert_eq!(v["total_found"].as_u64(), Some(0));
    // An empty result with no note reads as "no hits" — the note is the
    // difference between a limitation and a lie.
    let note = v["note"].as_str().unwrap_or_default();
    assert!(note.contains("MAC"), "note: {note}");
}

// ── iris_test on Caché: colon-syntax RunTest, end to end ─────────────────────
//
// The IRIS-shaped `RunTest("Pkg","/verbose=1/nodelete/noload")` call never runs a
// single test on Caché (no /verbose qualifier; a bare name in testspec names a
// ^UnitTestRoot directory). These tests hold the Caché branch to its contract:
// fixture classes are PUT + compiled through Atelier directly, then `iris_test`
// runs them through the full handler — dictionary resolution, colon-syntax
// RunTest, /displaylog stdout, and the parser.

const FIXTURE_CLASS: &str = "IadCacheTest.FixtureTest";

/// IrisTools around a probed Caché connection — `product` is only resolved
/// after `probe()`, and the iris_test Caché branch gates on it.
async fn cache_tools() -> Option<IrisTools> {
    let iris = probed_conn().await?;
    Some(IrisTools::new(Some(iris)).expect("IrisTools::new"))
}

/// The connection the tools instance holds, cloned out of its Arc — fixture
/// setup/deardown talks Atelier directly rather than through the handler.
fn tools_test_connection(tools: &IrisTools) -> IrisConnection {
    let state = tools.connection.lock().unwrap();
    state
        .iris
        .as_ref()
        .expect("cache_tools always sets a connection")
        .as_ref()
        .clone()
}

/// PUT an arbitrary class document through Atelier and compile it. Returns false on any
/// failure so callers can skip (and report) rather than panic on a connection problem.
async fn put_class(
    iris: &IrisConnection,
    client: &reqwest::Client,
    class_name: &str,
    content: &[&str],
) -> bool {
    let doc_name = format!("{class_name}.cls");
    let lines: Vec<String> = content.iter().map(|s| s.to_string()).collect();
    let url = iris.versioned_ns_url(NS, &format!("/doc/{doc_name}"));
    let Ok(resp) = client
        .put(&url)
        .basic_auth(&iris.username, Some(&iris.password))
        .json(&serde_json::json!({ "enc": false, "content": lines }))
        .send()
        .await
    else {
        return false;
    };
    if !resp.status().is_success() {
        return false;
    }
    matches!(
        iris.compile_document(&doc_name, NS, "cuk", client).await,
        Ok(r) if r.success()
    )
}

/// PUT a class document through Atelier and compile it. Returns false on any
/// failure so callers can skip (and report) rather than panic on a connection
/// problem.
async fn compile_fixture(
    iris: &IrisConnection,
    client: &reqwest::Client,
    class_name: &str,
) -> bool {
    put_class(
        iris,
        client,
        class_name,
        &[
            &format!("Class {class_name} Extends %UnitTest.TestCase"),
            "{",
            "",
            "Method TestOne()",
            "{",
            r#"    do $$$AssertEquals(1, 1, "one equals one")"#,
            "}",
            "",
            "Method TestTwo()",
            "{",
            r#"    do $$$AssertEquals(2, 2, "two equals two")"#,
            "}",
            "}",
        ],
    )
    .await
}

async fn delete_fixture(iris: &IrisConnection, client: &reqwest::Client, class_name: &str) {
    // delete_doc is private, and the cleanup is best-effort anyway — a plain
    // DELETE against the doc URL is exactly what it does.
    let url = iris.versioned_ns_url(NS, &format!("/doc/{class_name}.cls"));
    let _ = client
        .delete(&url)
        .basic_auth(&iris.username, Some(&iris.password))
        .send()
        .await;
}

/// Call a tool through the real handler dispatch and parse the JSON body.
async fn call_tool(tools: &IrisTools, tool: &str, params: serde_json::Value) -> serde_json::Value {
    let r = tools.call_for_test(tool, params).await.expect("dispatch");
    let text = r.content[0].as_text().expect("text content").text.clone();
    serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }))
}

/// Call iris_test through the real handler dispatch and parse the JSON body.
async fn call_test(tools: &IrisTools, params: serde_json::Value) -> serde_json::Value {
    call_tool(tools, "iris_test", params).await
}

#[tokio::test]
#[ignore]
async fn iris_test_runs_single_compiled_class_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    let iris = tools_test_connection(&tools);
    let client = reqwest::Client::new();
    assert!(
        compile_fixture(&iris, &client, FIXTURE_CLASS).await,
        "fixture must compile"
    );
    let v = call_test(&tools, serde_json::json!({ "pattern": FIXTURE_CLASS })).await;
    delete_fixture(&iris, &client, FIXTURE_CLASS).await;
    assert_eq!(v["success"].as_bool(), Some(true), "body: {v}");
    assert_eq!(v["total"].as_u64(), Some(2), "both fixture methods: {v}");
    assert_eq!(v["passed"].as_u64(), Some(2), "body: {v}");
    assert_eq!(v["failed"].as_u64(), Some(0), "body: {v}");
    let suites = v["test_suites"].as_array().expect("test_suites");
    assert_eq!(suites.len(), 1, "one class ran: {v}");
    assert_eq!(
        suites[0]["name"].as_str(),
        Some(FIXTURE_CLASS),
        "suite is the class: {v}"
    );
}

#[tokio::test]
#[ignore]
async fn iris_test_glob_expands_through_the_dictionary_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    let iris = tools_test_connection(&tools);
    let client = reqwest::Client::new();
    assert!(
        compile_fixture(&iris, &client, FIXTURE_CLASS).await,
        "fixture must compile"
    );
    let v = call_test(&tools, serde_json::json!({ "pattern": "IadCacheTest.*" })).await;
    delete_fixture(&iris, &client, FIXTURE_CLASS).await;
    assert_eq!(v["success"].as_bool(), Some(true), "body: {v}");
    assert_eq!(
        v["passed"].as_u64(),
        Some(2),
        "glob matched the fixture: {v}"
    );
    let suites = v["test_suites"].as_array().expect("test_suites");
    assert!(
        suites
            .iter()
            .any(|s| s["name"].as_str() == Some(FIXTURE_CLASS)),
        "suite names: {v}"
    );
}

#[tokio::test]
#[ignore]
async fn iris_test_directory_pattern_is_rejected_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    // The IRIS directory path assumes a Unix /tmp scaffold this Windows Caché
    // server does not have — refusing beats silently running nothing.
    let v = call_test(&tools, serde_json::json!({ "pattern": "MyApp/Tests" })).await;
    assert_eq!(v["success"].as_bool(), Some(false), "body: {v}");
    assert_eq!(
        v["error_code"].as_str(),
        Some("UNSUPPORTED_ON_CACHE"),
        "body: {v}"
    );
}

#[tokio::test]
#[ignore]
async fn iris_test_testproduction_type_is_rejected_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    let v = call_test(
        &tools,
        serde_json::json!({ "pattern": FIXTURE_CLASS, "test_type": "testproduction" }),
    )
    .await;
    assert_eq!(v["success"].as_bool(), Some(false), "body: {v}");
    assert_eq!(
        v["error_code"].as_str(),
        Some("UNSUPPORTED_ON_CACHE"),
        "body: {v}"
    );
}

#[tokio::test]
#[ignore]
async fn iris_test_missing_class_reports_no_tests_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    let v = call_test(
        &tools,
        serde_json::json!({ "pattern": "IadCacheTest.DoesNotExist" }),
    )
    .await;
    assert_eq!(v["success"].as_bool(), Some(false), "body: {v}");
    assert_eq!(
        v["error_code"].as_str(),
        Some("NO_TESTS_FOUND"),
        "body: {v}"
    );
    // A single class goes straight to RunTest (the Manager itself reports the
    // missing testcase), so the empty result surfaces through the stdout-parse
    // branch — only the glob branch answers from the dictionary.
    assert_eq!(v["source"].as_str(), Some("stdout_parse"), "body: {v}");
}

#[tokio::test]
#[ignore]
async fn iris_test_overbroad_glob_is_capped_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    // DHC-APP carries ~4.8k User.* classes — far past the 50-class cap. Each
    // class would be a separate RunTest inside one snippet, so this must be an
    // error, never a truncated or runaway run.
    let v = call_test(&tools, serde_json::json!({ "pattern": "User.*" })).await;
    assert_eq!(v["success"].as_bool(), Some(false), "body: {v}");
    assert_eq!(
        v["error_code"].as_str(),
        Some("TOO_MANY_TEST_CLASSES"),
        "body: {v}"
    );
}

// ── iris_table_info ───────────────────────────────────────────────────────────

const SQL_STORE_CLASS: &str = "IadCacheTest.TableInfoSqlStore";
// Short on purpose: Caché caps generated global names at 31 characters, and a
// longer class name would come back hash-truncated (`…PlD974D`), which would
// make the expected value server-dependent.
const PLAIN_STORE_CLASS: &str = "IadCacheTest.PlainStore";

/// A %CacheSQLStorage class — the legacy SQL mapping. %Dictionary.CompiledStorage
/// reports no DataLocation/IndexLocation for it, so the globals only exist in the
/// source Storage block's <SQLMap> entries. Glob layout copied from User.PAPatMas.
async fn compile_sql_store_fixture(iris: &IrisConnection, client: &reqwest::Client) -> bool {
    put_class(
        iris,
        client,
        SQL_STORE_CLASS,
        &[
            &format!(
                "Class {SQL_STORE_CLASS} Extends %Persistent [ StorageStrategy = SQLStorage ]"
            ),
            "{",
            "Property Name As %String;",
            "Index IdxOnName On Name [ IdKey ];",
            "Storage SQLStorage",
            "{",
            "<SQLMap name=\"DataMasterMap\">",
            "<Data name=\"Name\">",
            "<Delimiter>\"^\"</Delimiter>",
            "<Node>\"N\"</Node>",
            "<Piece>1</Piece>",
            "</Data>",
            "<Global>^IadCacheTest.SQLStoreT</Global>",
            "<RowIdSpec name=\"1\">",
            "<Expression>{L1}</Expression>",
            "<Field>Name</Field>",
            "</RowIdSpec>",
            "<Structure>delimited</Structure>",
            "<Subscript name=\"1\">",
            "<AccessType>sub</AccessType>",
            "<Expression>{Name}</Expression>",
            "<StartValue>1</StartValue>",
            "</Subscript>",
            "<Type>data</Type>",
            "</SQLMap>",
            "<SQLMap name=\"NameIdxMap\">",
            "<Global>^IadCacheTest.SQLStoreTi</Global>",
            "<RowIdSpec name=\"1\">",
            "<Expression>{L1}</Expression>",
            "<Field>Name</Field>",
            "</RowIdSpec>",
            "<Structure>delimited</Structure>",
            "<Subscript name=\"1\">",
            "<Expression>{Name}</Expression>",
            "</Subscript>",
            "<Type>index</Type>",
            "</SQLMap>",
            "<Type>%CacheSQLStorage</Type>",
            "}",
            "}",
        ],
    )
    .await
}

#[tokio::test]
#[ignore]
async fn iris_table_info_reads_globals_from_sql_storage_source_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    let iris = tools_test_connection(&tools);
    let client = reqwest::Client::new();
    assert!(
        compile_sql_store_fixture(&iris, &client).await,
        "fixture must compile"
    );
    // The class lives in the IadCacheTest package, so its SQL schema is the
    // package name — not SQLUser (which only covers the User package).
    let v = call_tool(
        &tools,
        "iris_table_info",
        serde_json::json!({ "table": SQL_STORE_CLASS }),
    )
    .await;
    delete_fixture(&iris, &client, SQL_STORE_CLASS).await;
    assert_eq!(v["success"].as_bool(), Some(true), "body: {v}");
    let r = &v["result"];
    assert_eq!(r["class"].as_str(), Some(SQL_STORE_CLASS), "body: {v}");
    assert_eq!(
        r["data_global"].as_str(),
        Some("^IadCacheTest.SQLStoreT"),
        "data global from the <SQLMap Type=\"data\"> block: {v}"
    );
    assert_eq!(
        r["index_global"].as_str(),
        Some("^IadCacheTest.SQLStoreTi"),
        "index global from the <SQLMap Type=\"index\"> block: {v}"
    );
}

#[tokio::test]
#[ignore]
async fn iris_table_info_dictionary_path_unchanged_for_default_storage_on_cache() {
    let Some(tools) = cache_tools().await else {
        return;
    };
    let iris = tools_test_connection(&tools);
    let client = reqwest::Client::new();
    assert!(
        put_class(
            &iris,
            &client,
            PLAIN_STORE_CLASS,
            &[
                &format!("Class {PLAIN_STORE_CLASS} Extends %Persistent"),
                "{",
                "Property Name As %String;",
                "Index NameIdx On Name;",
                "}",
            ],
        )
        .await,
        "fixture must compile"
    );
    let v = call_tool(
        &tools,
        "iris_table_info",
        serde_json::json!({ "table": PLAIN_STORE_CLASS }),
    )
    .await;
    delete_fixture(&iris, &client, PLAIN_STORE_CLASS).await;
    assert_eq!(v["success"].as_bool(), Some(true), "body: {v}");
    let r = &v["result"];
    // Default storage answers from %Dictionary.CompiledStorage — the source
    // fallback must not disturb this path.
    let expected_data = format!("^{PLAIN_STORE_CLASS}D");
    let expected_index = format!("^{PLAIN_STORE_CLASS}I");
    assert_eq!(
        r["data_global"].as_str(),
        Some(expected_data.as_str()),
        "body: {v}"
    );
    assert_eq!(
        r["index_global"].as_str(),
        Some(expected_index.as_str()),
        "body: {v}"
    );
}
