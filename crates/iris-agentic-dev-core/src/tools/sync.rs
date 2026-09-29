//! iris_sync — upload a local file to the server and compile it when the type needs
//! compiling, mirroring the VS Code ObjectScript plugin's save behavior: `.cls`/`.mac`/
//! `.inc`/`.csp` upload+compile, static web files (`.js`/`.css`/…) upload only.
//!
//! ## The URL rule this module exists for
//!
//! On Atelier v1 (Caché 2016.x, Apache gateway), a document name containing slashes —
//! every CSP-path document, e.g. `/dthealth/web/csp/x.js` — must be PUT with the slashes
//! **literal**: `/doc//dthealth/web/csp/x.js`. Encoding them as `%2F` (what a whole-name
//! `urlencoding::encode` produces, and what the slash-free class-name paths elsewhere in
//! this crate use) gets an Apache 404 that reads as "document not found" and pushes the
//! caller into a retry loop. [`encode_doc_path`] encodes per segment and keeps the
//! separators, which is the only form verified to work on both shapes — verified live
//! against Caché 2016.2.3: literal-slash PUT returns `upd:true` and GET reads the content
//! back; `%2F` PUT returns 404.
//!
//! ## PUT errors hide in two places
//!
//! A failed PUT on v1 returns HTTP 200 with the failure either in `status.errors` (the
//! array every other path reads) or as a string in `result.status` — `#16006` (invalid
//! document name) and `#5001` (illegal routine header) both surface only there. The
//! handler checks both.
//!
//! ## Document-name mapping (hfhis layout, recorded in AGENTS.md there)
//!
//! - `src/ABN/DHCNurBadResponse.cls` → `ABN.DHCNurBadResponse.cls` (class name taken from
//!   the `Class` declaration when present, path as fallback)
//! - `src/addloc.mac`, `src/Nur/DateFormat.inc` → path separators become dots:
//!   `addloc.mac`, `Nur.DateFormat.inc`. Local .mac/.inc files already carry the
//!   `ROUTINE <name>` header line v1 requires — content is uploaded verbatim.
//! - web files under a `[[sync.web_roots]]` mapping → server path, e.g.
//!   `src/dthealth/web/csp/x.csp` → `/dthealth/web/csp/x.csp`. Unmapped roots are refused
//!   with `NOT_SYNCABLE` rather than guessing a server path.

use schemars::JsonSchema;
use serde::Deserialize;

use crate::iris::connection::IrisConnection;
use crate::iris::workspace_config::SyncConfig;

/// Parameters for the `iris_sync` tool.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncParams {
    /// Local file to sync: absolute, or relative to the workspace root (the directory
    /// `.iris-agentic-dev.toml` is read from).
    pub path: String,
    /// Compile flags for class/routine/csp documents. Defaults to `[sync].flags` or "cuk".
    #[serde(default)]
    pub flags: Option<String>,
    /// IRIS namespace. Defaults to the connection namespace (IRIS_NAMESPACE).
    #[serde(default)]
    pub namespace: Option<String>,
    /// Route this call to a named registered IRIS instance. If omitted, uses the default
    /// connection.
    #[serde(default)]
    pub server: Option<String>,
}

/// What kind of sync a file needs.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyncKind {
    /// `.cls` — upload + compile.
    Class,
    /// `.mac` / `.int` / `.inc` — upload + compile.
    Routine,
    /// `.csp` — upload + compile (v1 compiles it into a `csp.*` class).
    Csp,
    /// Static web file (`.js`, `.css`, …) — upload only, like the VS Code plugin.
    WebStatic,
}

impl SyncKind {
    /// The `category` field of the response.
    pub fn as_str(self) -> &'static str {
        match self {
            SyncKind::Class => "class",
            SyncKind::Routine => "routine",
            SyncKind::Csp => "csp",
            SyncKind::WebStatic => "web",
        }
    }
}

/// Classify a file by extension. `None` = not syncable (binary types included — they are
/// refused as `NOT_SYNCABLE` rather than attempted and failed as a UTF-8 read).
pub fn classify_extension(path: &str) -> Option<SyncKind> {
    let ext = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "cls" => Some(SyncKind::Class),
        "mac" | "int" | "inc" => Some(SyncKind::Routine),
        "csp" => Some(SyncKind::Csp),
        // Text web assets only. Binaries (png/jpg/woff/…) have no text PUT path.
        "js" | "css" | "html" | "htm" | "svg" | "json" => Some(SyncKind::WebStatic),
        _ => None,
    }
}

/// Encode a document name for a URL path segment sequence: each `/`-separated segment is
/// percent-encoded, the separators stay literal. A leading `/` (CSP-path documents)
/// survives as the empty first segment, producing the `/doc//dthealth/...` form v1's
/// Apache gateway requires — see the module doc.
pub fn encode_doc_path(name: &str) -> String {
    name.split('/')
        .map(|seg| urlencoding::encode(seg).into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// Workspace-relative path (forward slashes) → server document name, via the
/// `[[sync.web_roots]]` mapping for web files and `src/`-strip + dots for code files.
///
/// `Err` carries the human-readable reason for the `NOT_SYNCABLE` result.
pub fn doc_name_for_rel_path(
    rel: &str,
    sync: Option<&SyncConfig>,
) -> Result<(String, SyncKind), String> {
    let rel_norm = rel.replace('\\', "/");
    let kind =
        classify_extension(&rel_norm).ok_or_else(|| format!("not a syncable file type: {rel}"))?;
    match kind {
        SyncKind::Class | SyncKind::Routine => {
            let p = rel_norm.strip_prefix("src/").unwrap_or(&rel_norm);
            Ok((p.replace('/', "."), kind))
        }
        SyncKind::Csp | SyncKind::WebStatic => {
            let mappings = sync.map(|s| s.web_roots.as_slice()).unwrap_or(&[]);
            for m in mappings {
                let local = m.local.replace('\\', "/");
                if let Some(rest) =
                    rel_norm.strip_prefix(&format!("{}/", local.trim_end_matches('/')))
                {
                    let server = m.server.trim_end_matches('/');
                    return Ok((format!("{}/{}", server, rest), kind));
                }
            }
            let configured = if mappings.is_empty() {
                "no [[sync.web_roots]] mappings are configured".to_string()
            } else {
                format!(
                    "configured mappings: {}",
                    mappings
                        .iter()
                        .map(|m| format!("{} -> {}", m.local, m.server))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            Err(format!(
                "web file {rel} is not under any [[sync.web_roots]] mapping ({configured})"
            ))
        }
    }
}

/// Extract the written file's path from a Claude Code PostToolUse hook payload
/// (`{"tool_name":"Edit","tool_input":{"file_path":"..."}}`). `None` when the payload
/// names no file — the hook stays silent (exit 0) rather than erroring.
pub fn file_path_from_hook_json(payload: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(payload).ok()?;
    let fp = v.get("tool_input")?.get("file_path")?.as_str()?;
    let fp = fp.trim();
    (!fp.is_empty()).then(|| fp.to_string())
}

fn ok_json(v: serde_json::Value) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    Ok(rmcp::model::CallToolResult::structured(v))
}
fn err_json(code: &str, msg: &str) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    crate::tools::err_result(serde_json::json!({
        "success": false,
        "error_code": code,
        "error": msg,
    }))
}

/// Handle one `iris_sync` call: resolve, read, map, gate, PUT, compile.
pub async fn handle_iris_sync(
    iris: &IrisConnection,
    client: &reqwest::Client,
    p: SyncParams,
    namespace: &str,
) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    // Resolve the file against the workspace root (the same root .iris-agentic-dev.toml
    // is found from), then express it relative to that root for the mapping step.
    let root = crate::iris::workspace_config::workspace_root(None);
    let given = std::path::Path::new(&p.path);
    let abs = if given.is_absolute() {
        given.to_path_buf()
    } else {
        root.join(given)
    };
    if !abs.is_file() {
        return err_json(
            "FILE_NOT_FOUND",
            &format!("Local file not found: {}", abs.display()),
        );
    }
    // Absolute path outside the workspace root: no rel-path mapping can apply.
    let rel = match abs.strip_prefix(&root) {
        Ok(r) => r.display().to_string(),
        Err(_) => {
            return err_json(
                "NOT_SYNCABLE",
                &format!(
                    "{} is outside the workspace root {} — sync needs a path inside it",
                    abs.display(),
                    root.display()
                ),
            )
        }
    };
    let rel = match rel {
        r if !r.is_empty() => r,
        _ => return err_json("NOT_SYNCABLE", "cannot sync the workspace root itself"),
    };

    let content = match std::fs::read_to_string(&abs) {
        Ok(c) => c,
        Err(e) => {
            return err_json(
                "READ_ERROR",
                &format!("Could not read {}: {}", abs.display(), e),
            )
        }
    };

    let sync_cfg = crate::iris::workspace_config::load_workspace_config(None).and_then(|c| c.sync);
    let (doc_name, kind) = match doc_name_for_rel_path(&rel, sync_cfg.as_ref()) {
        Ok(v) => v,
        Err(reason) => return err_json("NOT_SYNCABLE", &reason),
    };

    // Class name follows the Class declaration, not the file name — the same derivation
    // iris_compile's local-path branch uses. Path-derived name is the fallback.
    let doc_name = if kind == SyncKind::Class {
        content
            .lines()
            .find(|l| l.trim_start().to_lowercase().starts_with("class "))
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|cls| format!("{}.cls", cls))
            .unwrap_or(doc_name)
    } else {
        doc_name
    };

    // Compile-time code execution gate — same check every other compile path runs.
    if let Some(err) =
        crate::policy::code_edit_gate::check_compile_time_code_mode(&content, &doc_name)
    {
        return ok_json(err);
    }

    // PUT, per-segment encoded. `ignoreConflict=1`: this is a save-sync — the local file
    // is the source of truth, same choice the VS Code plugin makes on save.
    let put_url = iris.versioned_ns_url(
        namespace,
        &format!("/doc/{}?ignoreConflict=1", encode_doc_path(&doc_name)),
    );
    let lines: Vec<&str> = content.lines().collect();
    let put_resp = client
        .put(&put_url)
        .basic_auth(&iris.username, Some(&iris.password))
        .json(&serde_json::json!({"enc": false, "content": lines}))
        .send()
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("Upload failed: {e}"), None))?;
    if !put_resp.status().is_success() {
        return err_json(
            "UPLOAD_FAILED",
            &format!("PUT {} returned HTTP {}", doc_name, put_resp.status()),
        );
    }
    let put_body: serde_json::Value = put_resp.json().await.unwrap_or_default();
    // status.errors is where every other Atelier path reports failures…
    if let Some(errs) = put_body["status"]["errors"].as_array() {
        if !errs.is_empty() {
            let msg = errs[0]["error"].as_str().unwrap_or("Upload failed");
            return err_json("UPLOAD_FAILED", msg);
        }
    }
    // …and result.status (a string) is where v1 reports invalid-name / bad-header cases
    // like #16006 and #5001 with HTTP 200. Both were observed live on Caché 2016.2.3.
    if let Some(s) = put_body["result"]["status"].as_str() {
        if !s.trim().is_empty() {
            return err_json("UPLOAD_FAILED", s.trim());
        }
    }

    // Compile for the code kinds; static web files are upload-only, matching the plugin.
    let flags = p
        .flags
        .clone()
        .or_else(|| sync_cfg.as_ref().and_then(|s| s.flags.clone()))
        .unwrap_or_else(|| "cuk".to_string());
    let (action, compile_json) = match kind {
        SyncKind::WebStatic => ("uploaded", None),
        SyncKind::Class | SyncKind::Routine | SyncKind::Csp => {
            let cr = iris
                .compile_document(&doc_name, namespace, &flags, client)
                .await
                .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
            let compile = serde_json::json!({
                "success": cr.success(),
                "errors": cr.errors,
                "console": cr.console,
            });
            let action = if cr.success() {
                "uploaded+compiled"
            } else {
                "uploaded+compile_failed"
            };
            (action, Some(compile))
        }
    };
    let success = match &compile_json {
        Some(c) => c["success"].as_bool().unwrap_or(false),
        None => true,
    };

    if success {
        // Upload-only results carry no "compile" key at all — a null one reads as
        // "compiled to nothing" in the response consumers scan.
        let mut body = serde_json::json!({
            "success": true,
            "file": rel.replace('\\', "/"),
            "document": doc_name,
            "category": kind.as_str(),
            "action": action,
            "namespace": namespace,
        });
        if let Some(c) = compile_json {
            body["compile"] = c;
        }
        ok_json(body)
    } else {
        // Compile errors are reported with the full console so the caller can fix the
        // local file — the save-sync loop's whole point.
        let errors = compile_json
            .as_ref()
            .and_then(|c| c["errors"].as_array())
            .cloned()
            .unwrap_or_default();
        let first = errors
            .first()
            .and_then(|e| e.as_str())
            .unwrap_or("compile failed");
        crate::tools::err_result(serde_json::json!({
            "success": false,
            "error_code": "COMPILE_ERROR",
            "error": first,
            "file": rel.replace('\\', "/"),
            "document": doc_name,
            "category": kind.as_str(),
            "action": action,
            "compile": compile_json,
            "namespace": namespace,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iris::workspace_config::WebRootMapping;

    fn sync_cfg() -> SyncConfig {
        SyncConfig {
            flags: None,
            web_roots: vec![WebRootMapping {
                local: "src/dthealth/web".to_string(),
                server: "/dthealth/web".to_string(),
            }],
        }
    }

    #[test]
    fn classify_extensions() {
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
        assert_eq!(classify_extension("x.CLS"), Some(SyncKind::Class));
        assert_eq!(classify_extension("x.png"), None);
        assert_eq!(classify_extension("noext"), None);
        assert_eq!(classify_extension("CMakeLists.txt"), None);
    }

    #[test]
    fn encode_doc_path_keeps_separators_literal() {
        // The v1 Apache-gateway trap: %2F gets a 404, literal / works.
        assert_eq!(encode_doc_path("MyApp.MyClass.cls"), "MyApp.MyClass.cls");
        assert_eq!(
            encode_doc_path("/dthealth/web/csp/dh.logon.csp"),
            "/dthealth/web/csp/dh.logon.csp"
        );
        // % in a segment (e.g. %SYS-ish names) must still encode.
        assert_eq!(encode_doc_path("%zTest.mac"), "%25zTest.mac");
        // Spaces encode per segment, slashes stay.
        assert_eq!(encode_doc_path("/a b/c d.js"), "/a%20b/c%20d.js");
    }

    #[test]
    fn doc_names_for_code_paths() {
        let (name, kind) = doc_name_for_rel_path("src/ABN/DHCNurBadResponse.cls", None).unwrap();
        assert_eq!(name, "ABN.DHCNurBadResponse.cls");
        assert_eq!(kind, SyncKind::Class);

        let (name, kind) = doc_name_for_rel_path("src/addloc.mac", None).unwrap();
        assert_eq!(name, "addloc.mac");
        assert_eq!(kind, SyncKind::Routine);

        let (name, _) = doc_name_for_rel_path("src/Nur/DateFormat.inc", None).unwrap();
        assert_eq!(name, "Nur.DateFormat.inc");

        // Windows-shaped input normalizes.
        let (name, _) = doc_name_for_rel_path("src\\Nur\\DateFormat.inc", None).unwrap();
        assert_eq!(name, "Nur.DateFormat.inc");

        // No src/ prefix: mapped as-is (the root IS the src root in some layouts).
        let (name, _) = doc_name_for_rel_path("ABN/X.cls", None).unwrap();
        assert_eq!(name, "ABN.X.cls");
    }

    #[test]
    fn doc_names_for_web_paths_need_a_mapping() {
        let (name, kind) =
            doc_name_for_rel_path("src/dthealth/web/csp/dh.logon.csp", Some(&sync_cfg())).unwrap();
        assert_eq!(name, "/dthealth/web/csp/dh.logon.csp");
        assert_eq!(kind, SyncKind::Csp);

        let (name, _) = doc_name_for_rel_path(
            "src/dthealth/web/BOE/HISUI/BOE.AdrShare.js",
            Some(&sync_cfg()),
        )
        .unwrap();
        assert_eq!(name, "/dthealth/web/BOE/HISUI/BOE.AdrShare.js");

        // Unmapped root: refused with the configured mappings named, not guessed.
        let err = doc_name_for_rel_path("src/jquery/x.js", Some(&sync_cfg())).unwrap_err();
        assert!(err.contains("not under any [[sync.web_roots]] mapping"));
        assert!(err.contains("src/dthealth/web -> /dthealth/web"));

        // No config at all: names that as the reason.
        let err = doc_name_for_rel_path("src/anything/x.js", None).unwrap_err();
        assert!(err.contains("no [[sync.web_roots]] mappings are configured"));
    }

    #[test]
    fn hook_json_file_path_extraction() {
        let payload = r#"{"tool_name":"Edit","tool_input":{"file_path":"src/Nur/DateFormat.inc","old_string":"a","new_string":"b"}}"#;
        assert_eq!(
            file_path_from_hook_json(payload),
            Some("src/Nur/DateFormat.inc".to_string())
        );
        // Write uses the same tool_input.file_path shape.
        let payload = r#"{"tool_name":"Write","tool_input":{"file_path":"x.csp","content":"..."}}"#;
        assert_eq!(file_path_from_hook_json(payload), Some("x.csp".to_string()));
        // Non-file tools (Bash etc.) carry no file_path: stay silent.
        assert_eq!(
            file_path_from_hook_json(r#"{"tool_name":"Bash","tool_input":{"command":"ls"}}"#),
            None
        );
        assert_eq!(file_path_from_hook_json("not json"), None);
    }
}
