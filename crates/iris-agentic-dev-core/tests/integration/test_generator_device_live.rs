//! Live integration tests for the generator's output-device contract.
//!
//! `execute_via_generator` captures output by opening a temp file and selecting it as the
//! current device. Code that selects a different device and does not put it back sends every
//! later `Write` somewhere the tool never reads. Before `ERROR($DEVICE)` existed, that surfaced
//! as an empty string, and 27 call sites reported empty output as a successful result — the
//! 1.3.0 `iris_system_performance` bug.
//!
//! Since the device-recovery change, a drift to a *readable file* is recovered: the wrapper
//! closes the hijacked device, reads its contents back, and appends them to the captured
//! output. That is what makes server-side source-control hooks (iMedical on Caché, which
//! redirects the device even for `Write "hello"`) usable through the generator. The error
//! survives only for a drift to a non-file device (terminal, TCP) whose output cannot be
//! read back at all.
//!
//! Run with:
//!   IRIS_HOST=localhost IRIS_WEB_PORT=52780 \
//!   cargo test --features testing --test test_generator_device_live -- --ignored --nocapture

use iris_agentic_dev_core::iris::connection::{
    is_generator_error, DiscoverySource, IrisConnection,
};

fn make_conn() -> Option<(IrisConnection, reqwest::Client)> {
    let iris_host = std::env::var("IRIS_HOST").unwrap_or_default();
    if iris_host.is_empty() {
        eprintln!("IRIS_HOST not set — skipping");
        return None;
    }
    let web_port = std::env::var("IRIS_WEB_PORT").unwrap_or_else(|_| "52780".to_string());
    let username = std::env::var("IRIS_USERNAME").unwrap_or_else(|_| "_SYSTEM".to_string());
    let password = std::env::var("IRIS_PASSWORD").unwrap_or_else(|_| "SYS".to_string());
    let conn = IrisConnection::new(
        format!("http://{}:{}", iris_host, web_port),
        "USER",
        username,
        password,
        DiscoverySource::EnvVar,
    );
    Some((conn, reqwest::Client::new()))
}

/// A drift to another *file* is recovered: the wrapper closes the hijacked device and
/// reads its contents back, so output written after the drift comes home.
#[tokio::test]
#[ignore]
async fn stolen_file_device_output_is_read_back() {
    let Some((conn, client)) = make_conn() else {
        return;
    };

    let code = r#" Set f="/tmp/iad_device_drift_test.txt"
 Do ##class(%Library.File).Delete(f)
 Open f:("WNS"):5
 Use f
 Write "this output goes to the wrong device"
"#;
    let out = conn
        .execute_via_generator(code, "USER", &client)
        .await
        .expect("the call itself must succeed — the failure is in the output");

    assert!(
        !is_generator_error(&out),
        "a stolen *file* device is recoverable, got {out:?}"
    );
    assert!(
        out.contains("this output goes to the wrong device"),
        "the hijacked file's contents must be read back, got {out:?}"
    );
}

/// A drift to a non-file device (the principal device) cannot be read back — that
/// must still fail loudly, never return an empty success.
#[tokio::test]
#[ignore]
async fn stolen_non_file_device_is_still_an_error() {
    let Some((conn, client)) = make_conn() else {
        return;
    };

    let code = r#" Use 0
 Write "this goes to the terminal and is lost"
"#;
    let out = conn
        .execute_via_generator(code, "USER", &client)
        .await
        .expect("the call itself must succeed — the failure is in the output");

    assert!(
        is_generator_error(&out),
        "a stolen non-file device must be a generator error, got {out:?}"
    );
    assert!(
        out.contains("ERROR($DEVICE)") && out.contains("could not be read back"),
        "the error must name the shape and the recovery failure, got {out:?}"
    );
}

/// Snapshot-and-restore is the documented remedy, so it must produce ordinary output.
#[tokio::test]
#[ignore]
async fn restored_device_returns_output_normally() {
    let Some((conn, client)) = make_conn() else {
        return;
    };

    let code = r#" Set io=$IO
 Set f="/tmp/iad_device_restore_test.txt"
 Open f:("WNS"):5
 Use f
 Write "written to the side file"
 Use io
 Write "DEVICE-RESTORED"
"#;
    let out = conn
        .execute_via_generator(code, "USER", &client)
        .await
        .expect("must execute");

    assert!(
        !is_generator_error(&out),
        "restoring the device must not trip the guard, got {out:?}"
    );
    assert!(
        out.contains("DEVICE-RESTORED"),
        "output written after Use io must come back, got {out:?}"
    );
}

/// The guard must not fire on ordinary code, which is the regression that would break every tool.
#[tokio::test]
#[ignore]
async fn ordinary_output_is_untouched() {
    let Some((conn, client)) = make_conn() else {
        return;
    };

    let out = conn
        .execute_via_generator(" Write \"plain\",!\n Write 1+1", "USER", &client)
        .await
        .expect("must execute");

    assert!(!is_generator_error(&out), "unexpected error: {out:?}");
    assert!(out.contains("plain") && out.contains('2'), "got {out:?}");
}
