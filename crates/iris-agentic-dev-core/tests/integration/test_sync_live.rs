//! 120-save-sync live tests — `iris_sync` against a real Caché 2016.2.3 / Atelier v1
//! server (layer 3 of the testing policy).
//!
//! `#[ignore]` like every other live test: run with
//!
//! ```text
//! IRIS_HOST=10.146.8.17 IRIS_WEB_PORT=57772 IRIS_NAMESPACE=DHC-APP \
//! IRIS_USERNAME=_SYSTEM IRIS_PASSWORD=SYS \
//! cargo test --features testing --test integration -- --test-threads=1 --include-ignored sync_live
//! ```
//!
//! Everything here runs on scratch documents (`__iadsyncprobe*` / `IADSync.Probe*`) and
//! deletes them via raw HTTP at the end — the test never touches the project's business
//! code. Cleanup uses its own reqwest calls rather than `iris_doc` mode=delete because
//! that path whole-name-encodes the document name, which the v1 Apache gateway answers
//! with 404 for slash-containing names (the same trap `iris_sync`'s per-segment encoding
//! exists to avoid — see `sync.rs`'s module doc).
//!
//! `--test-threads=1` is mandatory: these tests set OBJECTSCRIPT_WORKSPACE, which every
//! other test in this target can see once they share a process.

use std::path::PathBuf;

use iris_agentic_dev_core::iris::connection::IrisConnection;
use iris_agentic_dev_core::tools::{IrisTools, Toolset};

/// The scratch workspace: a toml with one web-root mapping, and the files under test.
fn scratch_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join(".iris-agentic-dev.toml"),
        "[sync]\nflags = \"cuk\"\n\n[[sync.web_roots]]\nlocal  = \"src/web\"\nserver = \"/dthealth/web\"\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join("src/ABN")).unwrap();
    std::fs::create_dir_all(dir.path().join("src/web/csp")).unwrap();
    let root = dir.path().to_path_buf();
    (dir, root)
}

/// Connection from the IRIS_* env vars the run command above sets.
///
/// No probe needed: `IrisConnection::new` defaults `atelier_version` to V1, which is
/// what this Caché server serves — `versioned_ns_url` builds `/v1/...` either way.
fn live_conn() -> IrisConnection {
    let host = std::env::var("IRIS_HOST").expect("IRIS_HOST");
    let port = std::env::var("IRIS_WEB_PORT").unwrap_or_else(|_| "57772".into());
    let ns = std::env::var("IRIS_NAMESPACE").unwrap_or_else(|_| "DHC-APP".into());
    let user = std::env::var("IRIS_USERNAME").unwrap_or_else(|_| "_SYSTEM".into());
    let pass = std::env::var("IRIS_PASSWORD").unwrap_or_else(|_| "SYS".into());
    IrisConnection::new(
        format!("http://{host}:{port}"),
        &ns,
        &user,
        &pass,
        iris_agentic_dev_core::iris::connection::DiscoverySource::EnvVar,
    )
}

/// DELETE a scratch document by name, per-segment encoded like sync.rs does.
async fn delete_doc(iris: &IrisConnection, name: &str) {
    let encoded = name
        .split('/')
        .map(|s| urlencoding::encode(s).into_owned())
        .collect::<Vec<_>>()
        .join("/");
    let url = iris.versioned_ns_url(&iris.namespace, &format!("/doc/{encoded}"));
    let client = IrisConnection::http_client().unwrap();
    let _ = client
        .delete(&url)
        .basic_auth(&iris.username, Some(&iris.password))
        .send()
        .await;
}

async fn call_sync(tools: &IrisTools, path: &str) -> serde_json::Value {
    let result = tools
        .call_for_test("iris_sync", serde_json::json!({ "path": path }))
        .await
        .unwrap_or_else(|e| panic!("iris_sync({path}) dispatch failed: {e}"));
    for content in &result.content {
        if let Some(text) = content.as_text() {
            return serde_json::from_str(&text.text)
                .unwrap_or_else(|e| panic!("iris_sync({path}) returned non-JSON: {e}"));
        }
    }
    panic!("iris_sync({path}) returned no text content");
}

/// .cls: upload + compile, class name from the Class declaration.
///
/// Method bodies are indented with a tab — UDL treats column-1 lines inside a method
/// as class-level content, and the real hfhis files on disk are indented the same way.
#[tokio::test]
#[ignore = "live Caché 2016.2.3 server — see module doc for the env vars"]
async fn sync_live_cls_uploads_and_compiles() {
    let (_dir, root) = scratch_workspace();
    std::fs::write(
        root.join("src/ABN/IADSyncProbeCls.cls"),
        "Class IADSync.IADSyncProbeCls Extends %RegisteredObject\n{\nMethod Probe() As %String\n{\n\tQuit \"ok\"\n}\n}\n",
    )
    .unwrap();
    std::env::set_var("OBJECTSCRIPT_WORKSPACE", &root);
    let tools = IrisTools::new_with_toolset(Some(live_conn()), Toolset::Merged).unwrap();

    let body = call_sync(&tools, "src/ABN/IADSyncProbeCls.cls").await;
    assert_eq!(body["success"], serde_json::json!(true), "body: {body}");
    // The Class declaration wins over the path-derived name (IADSync.IADSyncProbeCls
    // vs ABN.IADSyncProbeCls).
    assert_eq!(body["document"], "IADSync.IADSyncProbeCls.cls");
    assert_eq!(body["category"], "class");
    assert_eq!(body["action"], "uploaded+compiled");
    assert_eq!(
        body["compile"]["success"],
        serde_json::json!(true),
        "compile: {}",
        body["compile"]
    );

    delete_doc(&live_conn(), "IADSync.IADSyncProbeCls.cls").await;
    std::env::remove_var("OBJECTSCRIPT_WORKSPACE");
}

/// .mac: upload + compile. Local .mac files carry the `ROUTINE <name>` header v1 requires
/// — the probe writes one the same shape the real files on disk have.
#[tokio::test]
#[ignore = "live Caché 2016.2.3 server — see module doc for the env vars"]
async fn sync_live_mac_uploads_and_compiles() {
    let (_dir, root) = scratch_workspace();
    std::fs::write(
        root.join("src/iadsyncprobemac.mac"),
        "ROUTINE iadsyncprobemac\nPROBE\t; sync probe\n\tQuit\n",
    )
    .unwrap();
    std::env::set_var("OBJECTSCRIPT_WORKSPACE", &root);
    let tools = IrisTools::new_with_toolset(Some(live_conn()), Toolset::Merged).unwrap();

    let body = call_sync(&tools, "src/iadsyncprobemac.mac").await;
    assert_eq!(body["success"], serde_json::json!(true), "body: {body}");
    assert_eq!(body["document"], "iadsyncprobemac.mac");
    assert_eq!(body["category"], "routine");
    assert_eq!(body["action"], "uploaded+compiled");

    delete_doc(&live_conn(), "iadsyncprobemac.mac").await;
    std::env::remove_var("OBJECTSCRIPT_WORKSPACE");
}

/// Static web file (.js): upload only, mapped through [[sync.web_roots]] — the path that
/// needs both the mapping and the literal-slash URL form.
#[tokio::test]
#[ignore = "live Caché 2016.2.3 server — see module doc for the env vars"]
async fn sync_live_web_static_uploads_without_compile() {
    let (_dir, root) = scratch_workspace();
    std::fs::write(
        root.join("src/web/csp/iadsyncprobe.js"),
        "// iad sync probe",
    )
    .unwrap();
    std::env::set_var("OBJECTSCRIPT_WORKSPACE", &root);
    let tools = IrisTools::new_with_toolset(Some(live_conn()), Toolset::Merged).unwrap();

    let body = call_sync(&tools, "src/web/csp/iadsyncprobe.js").await;
    assert_eq!(body["success"], serde_json::json!(true), "body: {body}");
    assert_eq!(body["document"], "/dthealth/web/csp/iadsyncprobe.js");
    assert_eq!(body["category"], "web");
    assert_eq!(body["action"], "uploaded");
    // Upload-only: no compile key at all, not a null one.
    assert!(body.get("compile").is_none(), "body: {body}");

    delete_doc(&live_conn(), "/dthealth/web/csp/iadsyncprobe.js").await;
    std::env::remove_var("OBJECTSCRIPT_WORKSPACE");
}

/// .csp through the same mapping: upload + compile — where this server allows it.
///
/// On this particular Caché 2016.2.3 box CSP *compilation* is broken at the server
/// level: the CSP application root points at a directory that does not exist
/// (`D:\dthealth\app\dthis\web\csp`), so even the real `dh.logon.csp` fails with
/// #5012 (file does not exist) + #5351 (class does not exist) — verified live by
/// curl before this test was written. The upload itself works; the compile must be
/// reported as COMPILE_ERROR rather than swallowed. This test pins that honest
/// failure so a regression to "silently pretend it compiled" fails here.
#[tokio::test]
#[ignore = "live Caché 2016.2.3 server — see module doc for the env vars"]
async fn sync_live_csp_uploads_and_compiles() {
    let (_dir, root) = scratch_workspace();
    std::fs::write(
        root.join("src/web/csp/iadsyncprobe.csp"),
        "<html><body>probe</body></html>",
    )
    .unwrap();
    std::env::set_var("OBJECTSCRIPT_WORKSPACE", &root);
    let tools = IrisTools::new_with_toolset(Some(live_conn()), Toolset::Merged).unwrap();

    let body = call_sync(&tools, "src/web/csp/iadsyncprobe.csp").await;
    assert_eq!(body["success"], serde_json::json!(false), "body: {body}");
    assert_eq!(body["document"], "/dthealth/web/csp/iadsyncprobe.csp");
    assert_eq!(body["category"], "csp");
    // The upload happened; the compile failed server-side (#5012/#5351 class of
    // errors — the console array carries them for the caller).
    assert_eq!(body["action"], "uploaded+compile_failed");
    assert_eq!(body["error_code"], "COMPILE_ERROR");
    assert!(
        body["compile"]["console"].is_array(),
        "console must be an array: {body}"
    );

    delete_doc(&live_conn(), "/dthealth/web/csp/iadsyncprobe.csp").await;
    std::env::remove_var("OBJECTSCRIPT_WORKSPACE");
}

/// A web file under no mapping is refused NOT_SYNCABLE before any network call — never
/// a guessed server path.
#[tokio::test]
#[ignore = "live Caché 2016.2.3 server — see module doc for the env vars"]
async fn sync_live_unmapped_web_root_refused_not_syncable() {
    let (_dir, root) = scratch_workspace();
    std::fs::create_dir_all(root.join("src/jquery")).unwrap();
    std::fs::write(root.join("src/jquery/x.js"), "// nope").unwrap();
    std::env::set_var("OBJECTSCRIPT_WORKSPACE", &root);
    let tools = IrisTools::new_with_toolset(Some(live_conn()), Toolset::Merged).unwrap();

    let body = call_sync(&tools, "src/jquery/x.js").await;
    assert_eq!(body["success"], serde_json::json!(false), "body: {body}");
    assert_eq!(body["error_code"], "NOT_SYNCABLE");
    assert!(
        body["error"].as_str().unwrap_or("").contains("web_roots"),
        "error should name the mapping gap: {body}"
    );

    std::env::remove_var("OBJECTSCRIPT_WORKSPACE");
}

/// A broken class compiles to COMPILE_ERROR with the compiler console attached — the
/// response the save-sync loop feeds back to the model.
///
/// The break is semantic (a property of an undefined type), not syntactic: verified live,
/// #5373 comes back with a full console, and it survives any whitespace shape the model
/// might write. The method body is tab-indented per UDL rules.
#[tokio::test]
#[ignore = "live Caché 2016.2.3 server — see module doc for the env vars"]
async fn sync_live_compile_error_reports_console() {
    let (_dir, root) = scratch_workspace();
    std::fs::write(
        root.join("src/ABN/IADSyncProbeBad.cls"),
        "Class IADSync.IADSyncProbeBad Extends %RegisteredObject\n{\nProperty BadProp As IADSync.NoSuchTypeAnywhere;\n\nMethod Broken() As %String\n{\n\tQuit \"ok\"\n}\n}\n",
    )
    .unwrap();
    std::env::set_var("OBJECTSCRIPT_WORKSPACE", &root);
    let tools = IrisTools::new_with_toolset(Some(live_conn()), Toolset::Merged).unwrap();

    let body = call_sync(&tools, "src/ABN/IADSyncProbeBad.cls").await;
    assert_eq!(body["success"], serde_json::json!(false), "body: {body}");
    assert_eq!(body["error_code"], "COMPILE_ERROR");
    assert_eq!(body["action"], "uploaded+compile_failed");
    assert!(
        body["compile"]["console"].is_array(),
        "console must be an array: {body}"
    );

    delete_doc(&live_conn(), "IADSync.IADSyncProbeBad.cls").await;
    std::env::remove_var("OBJECTSCRIPT_WORKSPACE");
}
