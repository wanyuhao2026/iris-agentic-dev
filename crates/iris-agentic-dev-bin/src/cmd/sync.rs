//! `iris-agentic-dev sync` — the save-sync CLI: the PostToolUse hook's entry point.
//!
//! Two modes:
//!
//! - **Manual**: `sync <FILES...>` — sync the named files now. Exit 1 on failure,
//!   printing each file's error, same as `compile`.
//! - **Hook**: `sync --hook` — read one Claude Code PostToolUse payload from stdin
//!   (`{"tool_name":"Edit","tool_input":{"file_path":"..."}}`), sync that one file if
//!   it is syncable, and **exit 2** on failure. Exit 2 is the PostToolUse contract for
//!   "feed stderr back to the model" — a compile error lands in the conversation, the
//!   model fixes the local file, the next save re-syncs. Exit 0 covers both success and
//!   the not-syncable skip (a `.md` edit, a file outside `src/`): silence there is
//!   correct, not a swallowed error.
//!
//! Connection resolution is `ConnectionArgs` like every other subcommand, so the hook
//! runs in the project root and picks up the project's `.iris-agentic-dev.toml`.

use anyhow::Result;
use clap::Args;
use iris_agentic_dev_core::tools::sync as sync_core;

use super::connection_args::ConnectionArgs;
use super::dispatch::dispatch_tool;

#[derive(Args)]
pub struct SyncCommand {
    /// File(s) to sync: absolute, or relative to the workspace root.
    /// With --hook, no files are read — the path comes from stdin.
    #[arg(value_name = "FILE")]
    pub files: Vec<String>,

    /// Hook mode: read a Claude Code PostToolUse JSON payload from stdin and sync the
    /// one file it names. Exit 2 on failure so stderr reaches the model.
    #[arg(long)]
    pub hook: bool,

    /// Compile flags (class/routine/csp documents). Default: [sync].flags or "cuk".
    #[arg(long)]
    pub flags: Option<String>,

    /// Route this call to a named registered IRIS instance instead of the default connection.
    #[arg(long)]
    pub server: Option<String>,

    /// Print each result as JSON instead of the one-line summary.
    #[arg(long)]
    pub json: bool,

    #[command(flatten)]
    pub conn: ConnectionArgs,
}

impl SyncCommand {
    pub async fn run(self) -> Result<()> {
        if self.hook {
            return self.run_hook().await;
        }
        if self.files.is_empty() {
            eprintln!("error: pass one or more files, or --hook to read a PostToolUse payload");
            std::process::exit(1);
        }
        // Namespace comes from the RESOLVED connection, not self.conn: clap's default
        // ("USER") passed as an explicit tool parameter would override the namespace
        // the workspace toml configured into the connection (resolve_namespace gives
        // the param precedence). Found live: a hook in a DHC-APP workspace synced to USER.
        let iris = self.conn.resolve().await?;
        let namespace = iris.namespace.clone();

        let mut any_error = false;
        for path in &self.files {
            let mut args = serde_json::json!({ "path": path, "namespace": namespace });
            if let Some(flags) = &self.flags {
                args["flags"] = serde_json::Value::String(flags.clone());
            }
            if let Some(server) = &self.server {
                args["server"] = serde_json::Value::String(server.clone());
            }
            match dispatch_tool(iris.clone(), "iris_sync", args).await {
                Ok(body) => {
                    let success = body["success"].as_bool().unwrap_or(false);
                    if self.json {
                        println!("{}", body);
                    } else if success {
                        println!(
                            "OK: {} -> {} ({})",
                            body["file"].as_str().unwrap_or(path),
                            body["document"].as_str().unwrap_or(""),
                            body["action"].as_str().unwrap_or("")
                        );
                    } else {
                        let code = body["error_code"].as_str().unwrap_or("SYNC_FAILED");
                        let msg = body["error"].as_str().unwrap_or("sync failed");
                        eprintln!("ERROR: {}: [{}] {}", path, code, msg);
                        for line in body["compile"]["console"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter_map(|l| l.as_str())
                        {
                            if line.trim().starts_with("ERROR") {
                                eprintln!("  {}", line.trim());
                            }
                        }
                    }
                    if !success {
                        any_error = true;
                    }
                }
                Err(e) => {
                    eprintln!("error: sync failed for {}: {}", path, e);
                    any_error = true;
                }
            }
        }
        if any_error {
            std::process::exit(1);
        }
        Ok(())
    }

    /// PostToolUse hook mode. See the module doc for the exit-code contract.
    async fn run_hook(self) -> Result<()> {
        let mut payload = String::new();
        std::io::stdin()
            .read_line(&mut payload)
            .map_err(|e| anyhow::anyhow!("reading hook payload from stdin: {e}"))?;
        let Some(file) = sync_core::file_path_from_hook_json(payload.trim()) else {
            // No file named (Bash, Grep, …) — nothing to sync, nothing to say.
            std::process::exit(0);
        };

        // Same as run(): namespace from the resolved connection, or clap's "USER"
        // default overrides the workspace toml's namespace (found live on a
        // DHC-APP workspace whose hook synced into USER).
        let iris = self.conn.resolve().await?;
        let namespace = iris.namespace.clone();
        let mut args = serde_json::json!({ "path": file, "namespace": namespace });
        if let Some(flags) = &self.flags {
            args["flags"] = serde_json::Value::String(flags.clone());
        }
        if let Some(server) = &self.server {
            args["server"] = serde_json::Value::String(server.clone());
        }

        let body = dispatch_tool(iris, "iris_sync", args).await?;
        match body["success"].as_bool() {
            Some(true) => {
                // One line, so a hook transcript stays readable; the details live in
                // the server and the local file, which already agree.
                println!(
                    "synced: {} -> {} ({})",
                    body["file"].as_str().unwrap_or(""),
                    body["document"].as_str().unwrap_or(""),
                    body["action"].as_str().unwrap_or("")
                );
                Ok(())
            }
            // NOT_SYNCABLE covers the routine cases (wrong extension, unmapped web
            // root, outside the workspace): the hook stays quiet rather than reporting
            // a "failure" for every unrelated file a session touches. Everything else
            // (upload/compile/read errors) is a real failure the model must see.
            _ if body["error_code"].as_str() == Some("NOT_SYNCABLE") => {
                eprintln!(
                    "sync skipped: {} ({})",
                    body["error"].as_str().unwrap_or("not syncable"),
                    file
                );
                Ok(())
            }
            _ => {
                let code = body["error_code"].as_str().unwrap_or("SYNC_FAILED");
                let msg = body["error"].as_str().unwrap_or("sync failed");
                eprintln!("sync failed: {file}: [{code}] {msg}");
                for line in body["compile"]["console"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|l| l.as_str())
                {
                    if line.trim().starts_with("ERROR") {
                        eprintln!("  {}", line.trim());
                    }
                }
                std::process::exit(2);
            }
        }
    }
}
