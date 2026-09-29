//! 120-save-sync CLI tests, layer 2: argument parsing for the `sync` subcommand.
//!
//! What "it parses" proves here is narrow on purpose: the flags exist and land on the
//! struct (`--hook`, `--flags`, `--server`, `--json`, positional FILEs). The exit-code
//! contract — success/skip = 0, failure = 2 in hook mode — is a live-connection
//! behavior, covered against the real server in `tests/integration/test_sync_live.rs`,
//! not mockable here.

use clap::{Parser, Subcommand};
use iris_agentic_dev::cmd::sync::SyncCommand;

#[derive(Parser)]
struct TestCli {
    #[command(subcommand)]
    cmd: TestCmd,
}

#[derive(Subcommand)]
enum TestCmd {
    Sync(SyncCommand),
}

fn parse(args: &[&str]) -> SyncCommand {
    let mut argv = vec!["test-cli", "sync"];
    argv.extend_from_slice(args);
    match TestCli::parse_from(argv).cmd {
        TestCmd::Sync(s) => s,
    }
}

#[test]
fn positional_files_collect() {
    let cmd = parse(&["src/ABN/X.cls", "src/addloc.mac"]);
    assert_eq!(cmd.files.len(), 2);
    assert_eq!(cmd.files[0], "src/ABN/X.cls");
    assert!(!cmd.hook);
}

#[test]
fn hook_flag_parses_without_files() {
    let cmd = parse(&["--hook"]);
    assert!(cmd.hook);
    assert!(cmd.files.is_empty());
}

#[test]
fn optional_flags_parse() {
    let cmd = parse(&["x.cls", "--flags", "cukd", "--server", "prod", "--json"]);
    assert_eq!(cmd.flags.as_deref(), Some("cukd"));
    assert_eq!(cmd.server.as_deref(), Some("prod"));
    assert!(cmd.json);
}

#[test]
fn no_files_and_no_hook_is_rejected_by_usage_not_a_silent_ok() {
    // clap itself accepts an empty positional list; the run() path exits 1 with a
    // message. What this pins is that the empty parse does not panic here.
    let cmd = parse(&[]);
    assert!(cmd.files.is_empty());
    assert!(!cmd.hook);
}
