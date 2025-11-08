use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use verctl::cargo_verctl::{
    Args, BumpKind, handle_single, handle_workspace, handle_workspace_default, is_workspace,
    list_versions, workspace_members,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn ws_root(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
    }

    #[test]
    fn test_list_versions_workspace() -> Result<()> {
        let root = ws_root("tests/workspaces/simple/Cargo.toml");
        assert!(root.exists(), "Missing test workspace file at {:?}", root);
        list_versions(&root)?;
        Ok(())
    }

    #[test]
    fn test_list_versions_single() -> Result<()> {
        let root = ws_root("tests/crates/simple/Cargo.toml");
        assert!(root.exists(), "Missing test crate file at {:?}", root);
        list_versions(&root)?;
        Ok(())
    }

    #[test]
    fn test_is_workspace_workspace() -> Result<()> {
        let root = ws_root("tests/workspaces/simple/Cargo.toml");
        assert!(root.exists(), "Missing test workspace file at {:?}", root);
        assert_eq!(is_workspace(&root).unwrap(), true);
        Ok(())
    }

    #[test]
    fn test_is_workspace_single() -> Result<()> {
        let root = ws_root("tests/crates/simple/Cargo.toml");
        assert!(root.exists(), "Missing test crate file at {:?}", root);
        assert_eq!(is_workspace(&root).unwrap(), false);
        Ok(())
    }

    #[test]
    fn test_handle_workspace_all_members() -> Result<()> {
        // Absolute path to the workspace Cargo.toml
        let root = ws_root("tests/workspaces/simple/Cargo.toml");

        // Make sure the test file exists
        assert!(root.exists(), "Missing test workspace at {:?}", root);

        // Prepare dummy args (no filtering)
        let mut args = Args::default();
        args.file = root.clone();
        args.bump = Some(BumpKind::None);
        args.set = Some(String::new());

        // Run the function — should process all members
        handle_workspace_default(&args, &root)?;
        Ok(())
    }

    #[test]
    fn test_handle_workspace_filtered_member() -> Result<()> {
        let root = ws_root("tests/workspaces/simple/Cargo.toml");
        assert!(root.exists(), "Missing test workspace at {:?}", root);

        // Only process one member
        let mut args = Args::default();
        args.file = root.clone();
        args.only = Some("a".to_string());
        args.bump = Some(BumpKind::None);
        args.set = Some(String::new());

        println!("✔ handle_workspace filtered to {:?}", args.only);
        handle_workspace_default(&args, &root)?;
        println!("✔ handle_workspace completed for {:?}", args.only);

        Ok(())
    }

    #[test]
    fn test_handle_workspace_with_mock() -> Result<()> {
        let root = ws_root("tests/workspaces/simple/Cargo.toml");
        assert!(root.exists());

        // args: no filter -> should handle both members
        let mut args = Args::default();
        args.file = root.clone();
        args.bump = Some(BumpKind::None);
        args.set = Some(String::new());

        // shared sink
        let handled: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = handled.clone();

        // mock handler records the folder name of each member (a / b)
        let mut mock = move |_a: &Args, p: &Path| -> Result<()> {
            let name = p
                .parent()
                .and_then(|q| q.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            sink.lock().unwrap().push(name);
            Ok(())
        };

        handle_workspace(&args, &root, &mut mock)?;

        let got = handled.lock().unwrap().clone(); // read from the original Arc
        assert_eq!(got.len(), 2, "Expected two members to be handled");
        assert!(got.contains(&"a".to_string()));
        assert!(got.contains(&"b".to_string()));
        Ok(())
    }

    #[test]
    fn test_handle_workspace_with_filter_mock() -> Result<()> {
        let root = ws_root("tests/workspaces/simple/Cargo.toml");
        assert!(root.exists());

        // filter to only "a"
        let mut args = Args::default();
        args.file = root.clone();
        args.only = Some("a".to_string());
        args.bump = Some(BumpKind::None);
        args.set = Some(String::new());

        let handled: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = handled.clone();

        let mut mock = move |_a: &Args, p: &Path| -> Result<()> {
            let name = p
                .parent()
                .and_then(|q| q.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            sink.lock().unwrap().push(name);
            Ok(())
        };

        println!("✔ handle_workspace filtered to {:?}", args.only);
        handle_workspace(&args, &root, &mut mock)?;
        println!("✔ handle_workspace completed for {:?}", args.only);

        let got = handled.lock().unwrap().clone();
        assert_eq!(got.len(), 1, "Expected only one filtered member");
        assert_eq!(got[0], "a");
        Ok(())
    }

    #[test]
    fn test_workspace_members_missing_file() {
        // Use a non-existent path
        let fake_root = PathBuf::from("tests/workspaces/does_not_exist/Cargo.toml");

        let result = workspace_members(&fake_root);

        assert!(
            result.is_err(),
            "Expected error for missing workspace file, got: {:?}",
            result
        );
    }

    #[test]
    fn test_workspace_members_invalid_toml() -> Result<()> {
        // Create a temporary invalid Cargo.toml for testing
        let tmp_dir = PathBuf::from("tests/tmp_invalid_ws");
        let tmp_file = tmp_dir.join("Cargo.toml");
        fs::create_dir_all(&tmp_dir)?;
        fs::write(&tmp_file, "[workspace]\nthis_is_invalid_toml = [")?;

        let result = workspace_members(&tmp_file);

        assert!(
            result.is_err(),
            "Expected error for invalid TOML format, got: {:?}",
            result
        );

        // Cleanup
        let _ = fs::remove_dir_all(&tmp_dir);
        Ok(())
    }

    #[test]
    fn test_handle_workspace_with_missing_members() -> Result<()> {
        // Create a fake workspace Cargo.toml with missing member path
        let tmp_dir = PathBuf::from("tests/tmp_ws_missing");
        let tmp_file = tmp_dir.join("Cargo.toml");
        fs::create_dir_all(&tmp_dir)?;
        fs::write(
            &tmp_file,
            r#"
        [workspace]
        members = ["does_not_exist"]
        "#,
        )?;

        let args = Args {
            bump: Some(BumpKind::None),
            set: Some(String::new()),
            file: tmp_file.clone(),
            only: None,
            auto: false,
            list: false,
        };

        // Should NOT panic; should handle missing member gracefully
        let result = handle_workspace_default(&args, &tmp_file);
        assert!(
            result.is_ok(),
            "Expected graceful handling of missing members, got: {:?}",
            result
        );

        // Cleanup
        let _ = fs::remove_dir_all(&tmp_dir);
        Ok(())
    }

    #[test]
    fn test_handle_single() -> Result<()> {
        let root = ws_root("tests/crates/simple/Cargo.toml");
        assert!(root.exists(), "Missing test crate file at {:?}", root);
        //assert_eq!(handle_single(&root).unwrap(), false);
        Ok(())
    }
}
