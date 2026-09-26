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
