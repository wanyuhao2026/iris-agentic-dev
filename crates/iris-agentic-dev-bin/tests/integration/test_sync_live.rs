//! 120-save-sync CLI tests, layer 2+3: spawn `iris-agentic-dev sync` as a subprocess
//! against a live Atelier server.
//!
//! What the unit parse tests cannot answer is the exit-code contract — the whole reason
//! the hook exists:
//!
//! - manual `sync <file>`: exit 0 + `OK:` on success, exit 1 + `ERROR:` on failure
//! - `sync --hook`: exit 0 for success, silence for non-file tools, a quiet
//!   `sync skipped` for NOT_SYNCABLE, and **exit 2** with the compiler console on
//!   stderr for a real failure — exit 2 being the PostToolUse "feed stderr back to
//!   the model" channel.
//!
//! `#[ignore]` like every other live test here: run with
//!
//! ```text
//! IRIS_HOST=10.146.8.17 IRIS_WEB_PORT=57772 IRIS_NAMESPACE=DHC-APP \
//! IRIS_USERNAME=_SYSTEM IRIS_PASSWORD=SYS \
//! cargo test --features testing --test bin_integration -- --test-threads=1 --include-ignored sync
//! ```
//!
//! Everything runs on scratch documents (`IADSync.SyncProbeBin*`) and deletes them via
//! raw HTTP at the end, per-segment-encoded the way sync.rs does — see that module's
//! doc for why whole-name `%2F` encoding 404s on the v1 gateway (no probe doc name here
//! contains a slash, but the shape stays consistent with the core live tests).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// A fresh `sync` invocation pointed at the live server. One builder per call —
/// `Command` is not cloneable, and each test needs its own.
fn sync_cmd() -> Command {
    let bin = env!("CARGO_BIN_EXE_iris-agentic-dev");
    let mut cmd = Command::new(bin);
    cmd.env("IRIS_HOST", env_or("IRIS_HOST", "localhost"))
        .env("IRIS_WEB_PORT", env_or("IRIS_WEB_PORT", "52780"))
        .env("IRIS_NAMESPACE", env_or("IRIS_NAMESPACE", "USER"))
        .env("IRIS_USERNAME", env_or("IRIS_USERNAME", "_SYSTEM"))
        .env("IRIS_PASSWORD", env_or("IRIS_PASSWORD", "SYS"));
    cmd
}

/// The scratch workspace root, exported as OBJECTSCRIPT_WORKSPACE for the child so
/// `src/...` paths resolve against it (the same root .iris-agentic-dev.toml would be
/// read from — none is needed for class syncs).
fn scratch_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src/ABN")).unwrap();
    let root = dir.path().to_path_buf();
    (dir, root)
}

fn probe_class_name(prefix: &str) -> String {
    format!("IADSync.{prefix}Bin{}", std::process::id())
}

/// A valid probe class: tab-indented method body — UDL treats column-1 lines inside a
/// method as class-level content, and this exact shape was verified to compile clean
/// on the live Caché 2016.2.3 server before this test was written.
fn write_good_cls(root: &Path, name: &str) {
    std::fs::write(
        root.join(format!("src/ABN/{name}.cls")),
        format!(
            "Class {name} Extends %RegisteredObject\n{{\nMethod Probe() As %String\n{{\n\tQuit \"ok\"\n}}\n}}\n",
        ),
    )
    .unwrap();
}

/// A class with a semantic break (property of an undefined type) — #5373 with a full
/// console, stable under any whitespace shape, unlike a syntax error.
fn write_bad_cls(root: &Path, name: &str) {
    std::fs::write(
        root.join(format!("src/ABN/{name}.cls")),
        format!(
            "Class {name} Extends %RegisteredObject\n{{\nProperty BadProp As IADSync.NoSuchTypeAnywhere;\n\nMethod Broken() As %String\n{{\n\tQuit \"ok\"\n}}\n}}\n",
        ),
    )
    .unwrap();
}

/// Build a fresh sync command, set the scratch workspace, wire stdin/stdout/stderr,
/// feed `payload` to stdin, and run to completion. Hook mode reads exactly one line.
fn run_sync(args: &[&str], workspace: &Path, payload: &str) -> Output {
    let mut cmd = sync_cmd();
    cmd.args(args)
        .env("OBJECTSCRIPT_WORKSPACE", workspace)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("failed to spawn iris-agentic-dev");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    child
        .wait_with_output()
        .expect("child did not run to completion")
}

/// Like [`run_sync`] but with every `IRIS_*` connection variable stripped from the
/// child. The workspace toml then becomes the child's *only* source of connection
/// facts — which is exactly the state the hook runs in inside a real project.
fn run_sync_no_iris_env(args: &[&str], workspace: &Path, payload: &str) -> Output {
    let mut cmd = sync_cmd();
    cmd.args(args)
        .env("OBJECTSCRIPT_WORKSPACE", workspace)
        .env_remove("IRIS_HOST")
        .env_remove("IRIS_WEB_PORT")
        .env_remove("IRIS_NAMESPACE")
        .env_remove("IRIS_USERNAME")
        .env_remove("IRIS_PASSWORD")
        .env_remove("IRIS_CONTAINER")
        .env_remove("IRIS_WEB_PREFIX")
        .env_remove("IRIS_SCHEME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("failed to spawn iris-agentic-dev");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    child
        .wait_with_output()
        .expect("child did not run to completion")
}

fn hook_payload(file: &str) -> String {
    format!(r#"{{"tool_name":"Edit","tool_input":{{"file_path":"{file}"}}}}"#)
}

/// DELETE a scratch document by name via raw HTTP (the same cleanup shape the core
/// sync_live tests use).
async fn delete_doc(name: &str) {
    let host = env_or("IRIS_HOST", "localhost");
    let port = env_or("IRIS_WEB_PORT", "52780");
    let ns = env_or("IRIS_NAMESPACE", "USER");
    let user = env_or("IRIS_USERNAME", "_SYSTEM");
    let pass = env_or("IRIS_PASSWORD", "SYS");
    let encoded = name
        .split('/')
        .map(|s| urlencoding::encode(s).into_owned())
        .collect::<Vec<_>>()
        .join("/");
    let url = format!("http://{host}:{port}/api/atelier/v1/{ns}/doc/{encoded}");
    let _ = reqwest::Client::new()
        .delete(&url)
        .basic_auth(&user, Some(&pass))
        .send()
        .await;
}

/// HTTP status of a GET for `name` in `namespace` — the ground truth for "which
/// namespace did the sync actually land in", independent of what the CLI printed.
async fn doc_status(namespace: &str, name: &str) -> u16 {
    let host = env_or("IRIS_HOST", "localhost");
    let port = env_or("IRIS_WEB_PORT", "52780");
    let user = env_or("IRIS_USERNAME", "_SYSTEM");
    let pass = env_or("IRIS_PASSWORD", "SYS");
    let encoded = name
        .split('/')
        .map(|s| urlencoding::encode(s).into_owned())
        .collect::<Vec<_>>()
        .join("/");
    let url = format!("http://{host}:{port}/api/atelier/v1/{namespace}/doc/{encoded}");
    reqwest::Client::new()
        .get(&url)
        .basic_auth(&user, Some(&pass))
        .send()
        .await
        .map(|r| r.status().as_u16())
        .unwrap_or(0)
}

/// Manual mode: one good class, exit 0, `OK:` line naming the mapped document.
#[tokio::test]
#[ignore]
async fn sync_manual_cls_uploads_compiles_and_exits_zero() {
    let (_dir, root) = scratch_workspace();
    let name = probe_class_name("SyncProbe");
    write_good_cls(&root, &name);
    let out = run_sync(&["sync", &format!("src/ABN/{name}.cls")], &root, "");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "expected exit 0\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        stdout.contains("OK:") && stdout.contains(&format!("{name}.cls")),
        "expected 'OK:' naming the document, got: {stdout}"
    );

    delete_doc(&format!("{name}.cls")).await;
}

/// Hook mode with a real edit: exit 0 and one `synced:` line.
#[tokio::test]
#[ignore]
async fn sync_hook_syncs_the_edited_file_and_exits_zero() {
    let (_dir, root) = scratch_workspace();
    let name = probe_class_name("SyncProbeHook");
    write_good_cls(&root, &name);
    let out = run_sync(
        &["sync", "--hook"],
        &root,
        &hook_payload(&format!("src/ABN/{name}.cls")),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "expected exit 0\nstdout: {stdout}\nstderr: {stderr}"
    );
    assert!(
        stdout.contains("synced:") && stdout.contains(&format!("{name}.cls")),
        "expected a 'synced:' line naming the document, got: {stdout}"
    );

    delete_doc(&format!("{name}.cls")).await;
}

/// The exit-code contract the hook exists for: a compile failure exits 2 so stderr
/// reaches the model, with the compiler ERROR line included.
#[tokio::test]
#[ignore]
async fn sync_hook_compile_error_exits_2_with_stderr() {
    let (_dir, root) = scratch_workspace();
    let name = probe_class_name("SyncProbeBad");
    write_bad_cls(&root, &name);
    let out = run_sync(
        &["sync", "--hook"],
        &root,
        &hook_payload(&format!("src/ABN/{name}.cls")),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(2),
        "expected exit 2 (PostToolUse feedback channel)\nstdout: {}\nstderr: {stderr}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        stderr.contains("sync failed") && stderr.contains("ERROR"),
        "stderr must carry the failure and a compiler ERROR line, got: {stderr}"
    );

    delete_doc(&format!("{name}.cls")).await;
}

/// A payload that names no file (Bash, Grep, …): exit 0, silent — the hook must not
/// speak for every unrelated tool a session runs.
#[tokio::test]
#[ignore]
async fn sync_hook_non_file_payload_exits_0_silent() {
    let (_dir, root) = scratch_workspace();
    let out = run_sync(
        &["sync", "--hook"],
        &root,
        r#"{"tool_name":"Bash","tool_input":{"command":"ls"}}"#,
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "expected exit 0 for a non-file payload\nstdout: {stdout}\nstderr: {stderr}"
    );
    assert!(
        stdout.trim().is_empty() && stderr.trim().is_empty(),
        "the hook must stay silent when there is nothing to sync, got stdout: {stdout} stderr: {stderr}"
    );
}

/// A file that is syncable in principle but not here (a .md under src/) is a skip, not
/// a failure: exit 0 with a one-line `sync skipped` on stderr.
#[tokio::test]
#[ignore]
async fn sync_hook_not_syncable_file_skips_with_exit_0() {
    let (_dir, root) = scratch_workspace();
    std::fs::write(root.join("src/notes.md"), "notes\n").unwrap();
    let out = run_sync(&["sync", "--hook"], &root, &hook_payload("src/notes.md"));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "NOT_SYNCABLE is a skip, not a failure\nstderr: {stderr}"
    );
    assert!(
        stderr.contains("sync skipped"),
        "expected a one-line skip notice, got: {stderr}"
    );
}

/// The namespace regression: with no `IRIS_*` env in the child, the workspace toml is
/// the only connection source, and the sync must land in the toml's namespace — not
/// clap's "USER" default passed through as an explicit tool parameter.
///
/// Found live while deploying the hook into a DHC-APP workspace: the hook reported
/// `uploaded+compiled` but the document landed in USER, because `SyncCommand::run_hook`
/// took `self.conn.namespace` (clap's default) before `resolve()` and passed it to
/// `iris_sync`, where `resolve_namespace` gives an explicit param precedence over the
/// connection. Every other live test here was masked by `sync_cmd()` setting
/// `IRIS_NAMESPACE`, which clap reads as an env default — this test strips the env so
/// the bug shape can actually appear.
#[tokio::test]
#[ignore]
async fn sync_hook_uses_the_workspace_toml_namespace_not_the_clap_default() {
    let (dir, root) = scratch_workspace();
    // A toml pointing at the live server with a namespace that differs from clap's
    // "USER" default — whatever the run's IRIS_NAMESPACE says.
    let ns = env_or("IRIS_NAMESPACE", "USER");
    if ns == "USER" {
        // Without a differing namespace this test cannot distinguish the two paths;
        // the run command in the module doc sets DHC-APP.
        eprintln!("skipped: IRIS_NAMESPACE is USER, the default this test pins against");
        return;
    }
    std::fs::write(
        root.join(".iris-agentic-dev.toml"),
        format!(
            "host = \"{}\"\nweb_port = {}\nnamespace = \"{}\"\nusername = \"{}\"\npassword = \"{}\"\n",
            env_or("IRIS_HOST", "localhost"),
            env_or("IRIS_WEB_PORT", "52780"),
            ns,
            env_or("IRIS_USERNAME", "_SYSTEM"),
            env_or("IRIS_PASSWORD", "SYS"),
        ),
    )
    .unwrap();
    let name = probe_class_name("SyncProbeNs");
    write_good_cls(&root, &name);
    let doc = format!("{name}.cls");

    let out = run_sync_no_iris_env(
        &["sync", "--hook"],
        &root,
        &hook_payload(&format!("src/ABN/{name}.cls")),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "expected exit 0\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Ground truth from the server, not the CLI's own report: the document must exist
    // in the toml's namespace and not in USER.
    assert_eq!(
        doc_status(&ns, &doc).await,
        200,
        "the document must land in the workspace toml's namespace {ns}"
    );
    assert_eq!(
        doc_status("USER", &doc).await,
        404,
        "the document must NOT land in the clap default namespace USER"
    );

    let _ = &dir; // keep the tempdir alive until the assertions are done
    delete_doc_in(&ns, &doc).await;
}

/// DELETE in an explicit namespace (the cleanup helper assumes the run's namespace).
async fn delete_doc_in(namespace: &str, name: &str) {
    let host = env_or("IRIS_HOST", "localhost");
    let port = env_or("IRIS_WEB_PORT", "52780");
    let user = env_or("IRIS_USERNAME", "_SYSTEM");
    let pass = env_or("IRIS_PASSWORD", "SYS");
    let encoded = name
        .split('/')
        .map(|s| urlencoding::encode(s).into_owned())
        .collect::<Vec<_>>()
        .join("/");
    let url = format!("http://{host}:{port}/api/atelier/v1/{namespace}/doc/{encoded}");
    let _ = reqwest::Client::new()
        .delete(&url)
        .basic_auth(&user, Some(&pass))
        .send()
        .await;
}
