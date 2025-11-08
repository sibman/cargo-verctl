use anyhow::{Result, anyhow};
use clap::Parser;
use std::env;

mod cargo_verctl;
use cargo_verctl::{Args, handle_single, handle_workspace_default, is_workspace, list_versions};

fn main() -> Result<()> {
    // --- Workaround for Cargo on Windows injecting the subcommand name ---
    let mut raw_args: Vec<String> = env::args().collect();
    if raw_args.len() > 1 && raw_args[1] == "verctl" {
        #[cfg(debug_assertions)]
        eprintln!("⚙️  Cargo subcommand argument injection detected. Adjusting args.");
        raw_args.remove(1);
    }

    // --- Parse args normally using Clap ---
    let args = Args::parse_from(&raw_args);
    let root_toml = args.file.clone();
    if !root_toml.exists() {
        return Err(anyhow!("Cargo.toml not found at {:?}", root_toml));
    }

    if args.list {
        list_versions(&root_toml)?;
        return Ok(());
    }

    if is_workspace(&root_toml)? {
        handle_workspace_default(&args, &root_toml)?;
    } else {
        handle_single(&args, &root_toml)?;
    }
    Ok(())
}
