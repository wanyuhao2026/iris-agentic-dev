//! 120-save-sync unit tests — the pure mapping layer of `iris_sync`.
//!
//! No IRIS connection anywhere: what these cover is the deterministic half of the
//! save-sync — extension classification, the local-path → document-name mapping, the
//! per-segment URL encoding the v1 Apache gateway requires, the `[sync]` TOML
//! round-trip (layer 1 of the testing policy: parse the config *string*, not a struct
//! literal), and the hook-payload extraction. The upload/compile half lives in
//! `tests/integration/test_sync_live.rs` against the real server.
//!
//! The v1 URL rule deserves the direct assertions below: a whole-name
//! `urlencoding::encode` turns `/` into `%2F`, which the Caché 2016.2.3 Apache gateway
//! answers with 404 — verified live before this module was written. A regression back
//! to whole-name encoding would still compile and still pass every class-name test
//! (class names contain no slashes), so only these web-path assertions would catch it.

use iris_agentic_dev_core::iris::workspace_config::{SyncConfig, WorkspaceConfig};
use iris_agentic_dev_core::tools::sync::{
    classify_extension, doc_name_for_rel_path, encode_doc_path, file_path_from_hook_json, SyncKind,
};

/// Parse the `[sync]` section the way the real loader does — as part of a whole
/// `WorkspaceConfig`. Parsing `[[sync.web_roots]]` directly as `SyncConfig` silently
/// drops everything (the keys sit under the `sync.` prefix, which `SyncConfig` has no
/// field for) — the exact silent-drop shape this suite exists to catch, caught here in
/// the helper's first draft.
fn sync_cfg() -> SyncConfig {
    let cfg: WorkspaceConfig = toml::from_str(
        r#"
[sync]

[[sync.web_roots]]
local  = "src/dthealth/web"
server = "/dthealth/web"
"#,
    )
    .unwrap();
    cfg.sync.expect("[sync] section must parse")
}

#[test]
fn classify_extensions_matches_the_documented_matrix() {
    assert_eq!(classify_extension("a/b/C.cls"), Some(SyncKind::Class));
    assert_eq!(classify_extension("addloc.mac"), Some(SyncKind::Routine));
    assert_eq!(
        classify_extension("Nur/DateFormat.inc"),
        Some(SyncKind::Routine)
    );
    assert_eq!(classify_extension("x.int"), Some(SyncKind::Routine));
    assert_eq!(classify_extension("csp/dh.logon.csp"), Some(SyncKind::Csp));
    assert_eq!(classify_extension("BOE/x.js"), Some(SyncKind::WebStatic));
    assert_eq!(classify_extension("x.css"), Some(SyncKind::WebStatic));
    assert_eq!(classify_extension("x.htm"), Some(SyncKind::WebStatic));
    assert_eq!(classify_extension("x.svg"), Some(SyncKind::WebStatic));
    // Case-insensitive extension.
    assert_eq!(classify_extension("x.CLS"), Some(SyncKind::Class));
    assert_eq!(classify_extension("X.JS"), Some(SyncKind::WebStatic));
    // Not syncable: binaries and non-code text.
    assert_eq!(classify_extension("x.png"), None);
    assert_eq!(classify_extension("noext"), None);
    assert_eq!(classify_extension("README.md"), None);
    assert_eq!(classify_extension("Cargo.toml"), None);
}

#[test]
fn encode_doc_path_keeps_slashes_literal_for_the_v1_gateway() {
    // The trap this function exists for: %2F gets an Apache 404, literal / works.
    assert_eq!(
        encode_doc_path("/dthealth/web/csp/dh.logon.csp"),
        "/dthealth/web/csp/dh.logon.csp"
    );
    // Class names pass through unchanged (no separators).
    assert_eq!(
        encode_doc_path("ABN.DHCNurBadResponse.cls"),
        "ABN.DHCNurBadResponse.cls"
    );
    assert_eq!(encode_doc_path("addloc.mac"), "addloc.mac");
    // A leading slash survives as the empty first segment — the /doc//name form.
    assert!(encode_doc_path("/a.js").starts_with("/"));
    // Percent still encodes per segment (% → %25), spaces → %20.
    assert_eq!(encode_doc_path("%zTest.mac"), "%25zTest.mac");
    assert_eq!(encode_doc_path("/a b/c d.js"), "/a%20b/c%20d.js");
}

#[test]
fn code_paths_map_to_dotted_document_names() {
    let (name, kind) = doc_name_for_rel_path("src/ABN/DHCNurBadResponse.cls", None).unwrap();
    assert_eq!(name, "ABN.DHCNurBadResponse.cls");
    assert_eq!(kind, SyncKind::Class);

    let (name, kind) = doc_name_for_rel_path("src/addloc.mac", None).unwrap();
    assert_eq!(name, "addloc.mac");
    assert_eq!(kind, SyncKind::Routine);

    let (name, _) = doc_name_for_rel_path("src/Nur/DateFormat.inc", None).unwrap();
    assert_eq!(name, "Nur.DateFormat.inc");

    // Windows-shaped separators normalize before mapping.
    let (name, _) = doc_name_for_rel_path("src\\Nur\\DateFormat.inc", None).unwrap();
    assert_eq!(name, "Nur.DateFormat.inc");

    // No src/ prefix: mapped as-is, for layouts whose root IS the source root.
    let (name, _) = doc_name_for_rel_path("ABN/X.cls", None).unwrap();
    assert_eq!(name, "ABN.X.cls");
}

#[test]
fn web_paths_need_a_web_roots_mapping_and_say_which() {
    let cfg = sync_cfg();
    let (name, kind) =
        doc_name_for_rel_path("src/dthealth/web/csp/dh.logon.csp", Some(&cfg)).unwrap();
    assert_eq!(name, "/dthealth/web/csp/dh.logon.csp");
    assert_eq!(kind, SyncKind::Csp);

    // Nested web subdirectories keep their shape.
    let (name, _) =
        doc_name_for_rel_path("src/dthealth/web/BOE/HISUI/BOE.AdrShare.js", Some(&cfg)).unwrap();
    assert_eq!(name, "/dthealth/web/BOE/HISUI/BOE.AdrShare.js");

    // Unmapped local root: refused naming the configured mappings, not a guess.
    let err = doc_name_for_rel_path("src/jquery/x.js", Some(&cfg)).unwrap_err();
    assert!(err.contains("not under any [[sync.web_roots]] mapping"));
    assert!(err.contains("src/dthealth/web -> /dthealth/web"));

    // No [sync] section at all: that is the stated reason.
    let err = doc_name_for_rel_path("src/anything/x.js", None).unwrap_err();
    assert!(err.contains("no [[sync.web_roots]] mappings are configured"));
}

/// Layer 1 of the testing policy: the `[sync]` section must survive a real TOML parse.
/// This is the #110 pattern — a key serde silently drops reads as configured from the
/// docs while doing nothing. Parse the string, never build the struct by hand.
#[test]
fn sync_toml_round_trips_flags_and_web_roots() {
    // Bare SyncConfig shape: no [sync] prefix, so the keys land on the struct's own
    // fields. (The prefixed whole-file shape is the next test — a direct
    // from_str::<SyncConfig> on the prefixed form silently drops every key, the #110
    // pattern, which is exactly why both shapes get asserted.)
    let toml = r#"
flags = "cuk"

[[web_roots]]
local  = "src/dthealth/web"
server = "/dthealth/web"

[[web_roots]]
local  = "src/other"
server = "/other"
"#;
    let cfg: SyncConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.flags.as_deref(), Some("cuk"));
    assert_eq!(cfg.web_roots.len(), 2);
    assert_eq!(cfg.web_roots[0].local, "src/dthealth/web");
    assert_eq!(cfg.web_roots[0].server, "/dthealth/web");
    assert_eq!(cfg.web_roots[1].server, "/other");
}

/// The `[sync]` key must also parse as part of a whole `WorkspaceConfig` — a section
/// that only deserializes standalone is one whose placement in the real file is
/// untested.
#[test]
fn sync_section_parses_inside_the_workspace_config() {
    let toml = r#"
host = "10.146.8.17"
web_port = 57772
namespace = "DHC-APP"

[sync]
flags = "cuk"

[[sync.web_roots]]
local  = "src/dthealth/web"
server = "/dthealth/web"
"#;
    let cfg: WorkspaceConfig = toml::from_str(toml).unwrap();
    let sync = cfg.sync.as_ref().expect("[sync] section must parse");
    assert_eq!(sync.flags.as_deref(), Some("cuk"));
    assert_eq!(sync.web_roots.len(), 1);
    assert_eq!(sync.web_roots[0].server, "/dthealth/web");
}

#[test]
fn hook_payload_file_path_extraction() {
    // PostToolUse shape: tool_input.file_path, from Write/Edit/MultiEdit alike.
    let edit = r#"{"tool_name":"Edit","tool_input":{"file_path":"src/Nur/DateFormat.inc","old_string":"a","new_string":"b"}}"#;
    assert_eq!(
        file_path_from_hook_json(edit),
        Some("src/Nur/DateFormat.inc".to_string())
    );
    let write = r#"{"tool_name":"Write","tool_input":{"file_path":"x.csp","content":"..."}}"#;
    assert_eq!(file_path_from_hook_json(write), Some("x.csp".to_string()));

    // Tools that touch no file carry no file_path — the hook stays silent, exit 0.
    assert_eq!(
        file_path_from_hook_json(r#"{"tool_name":"Bash","tool_input":{"command":"ls"}}"#),
        None
    );
    assert_eq!(file_path_from_hook_json("not json"), None);
    assert_eq!(file_path_from_hook_json("{}"), None);
}
