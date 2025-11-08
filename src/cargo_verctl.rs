use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::{
    borrow::BorrowMut,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use toml_edit::{DocumentMut, value};

#[derive(Clone, Debug, ValueEnum)]
pub enum BumpKind {
    Major,
    Minor,
    Patch,
    None,
}

#[derive(Parser)]
#[command(
    name = "cargo-verctl",
    about = "Manage Cargo.toml version (auto bump, set, or workspace-wide)",
    version = "0.2.0"
)]
pub struct Args {
    #[arg(long, value_enum)]
    pub bump: Option<BumpKind>,
    #[arg(long)]
    pub auto: bool,
    #[arg(long)]
    pub set: Option<String>,
    #[arg(long, default_value = "Cargo.toml")]
    pub file: PathBuf,
    #[arg(long)]
    pub only: Option<String>,
    #[arg(long)]
    pub list: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            bump: None,
            set: None,
            file: PathBuf::new(),
            only: None,
            auto: false,
            list: false,
        }
    }
}

pub fn is_workspace(path: &Path) -> Result<bool> {
    let text = fs::read_to_string(path)?;
    let doc: DocumentMut = text.parse()?;
    Ok(doc.get("workspace").is_some())
}

pub fn workspace_members(root: &Path) -> Result<Vec<PathBuf>> {
    let text = fs::read_to_string(root)?;
    let doc: DocumentMut = text.parse()?;
    let mut members = vec![];
    if let Some(arr) = doc["workspace"]["members"].as_array() {
        for item in arr.iter() {
            let rel = item.as_str().unwrap_or("");
            let mut path = root
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            path.push(rel);
            path.push("Cargo.toml");
            if path.exists() {
                members.push(path);
            }
        }
    }
    Ok(members)
}

pub fn handle_workspace_default(args: &Args, root: &Path) -> Result<()> {
    handle_workspace(args, root, handle_single)
}

pub fn handle_workspace<F>(args: &Args, root: &Path, mut handler: F) -> Result<()>
where
    F: FnMut(&Args, &Path) -> Result<()>,
{
    println!("📦 Workspace detected: {:?}", root);
    let members = workspace_members(root)?;

    for member in &members {
        if let Some(ref only) = args.only {
            let folder_name = member
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("");

            if folder_name != only {
                continue;
            }
        }

        handler(args, member)?;
    }

    Ok(())
}

pub fn handle_single(args: &Args, path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let mut doc: DocumentMut = text.parse::<DocumentMut>()?;

    if !doc.as_table().contains_key("package") {
        return Err(anyhow::anyhow!("Missing [package] section in {:?}.", path));
    }

    let version_opt = doc["package"]["version"].as_str().map(|s| s.to_string());
    let version = match version_opt {
        Some(v) => v,
        None => {
            println!("🆕 No version found in {:?}, setting default 0.1.0", path);
            doc.borrow_mut()["package"]["version"] = value("0.1.0");
            "0.1.0".to_string()
        }
    };

    if let Some(vset) = &args.set {
        doc["package"]["version"] = value(vset.clone());
        fs::write(path, doc.to_string())?;
        println!("🔧 Set {:?} version to {}", path.display(), vset);
        return Ok(());
    }

    let bump_kind = if let Some(b) = args.bump.clone() {
        b
    } else if args.auto {
        BumpKind::Patch
    } else {
        print!(
            "Increment version for {:?}? (major/minor/patch/none) [patch]: ",
            path.display()
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "major" => BumpKind::Major,
            "minor" => BumpKind::Minor,
            "none" => BumpKind::None,
            _ => BumpKind::Patch,
        }
    };

    if matches!(bump_kind, BumpKind::None) {
        println!("➡️  Keeping version {} for {:?}", version, path);
        return Ok(());
    }

    let new_version = bump_version(&version, &bump_kind)?;
    doc["package"]["version"] = value(new_version.clone());
    fs::write(path, doc.to_string())?;
    println!("🔼 Updated {:?} → {}", path.display(), new_version);
    Ok(())
}

fn bump_version(current: &str, bump: &BumpKind) -> Result<String> {
    let mut parts: Vec<u32> = current.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    while parts.len() < 3 {
        parts.push(0);
    }
    match bump {
        BumpKind::Major => {
            parts[0] += 1;
            parts[1] = 0;
            parts[2] = 0;
        }
        BumpKind::Minor => {
            parts[1] += 1;
            parts[2] = 0;
        }
        BumpKind::Patch => {
            parts[2] += 1;
        }
        BumpKind::None => (),
    }
    Ok(format!("{}.{}.{}", parts[0], parts[1], parts[2]))
}

pub fn list_versions(root: &Path) -> Result<()> {
    if is_workspace(root)? {
        let members = workspace_members(root)?;
        println!("📦 Workspace members:");
        for member in members {
            let txt = fs::read_to_string(&member)?;
            let doc: DocumentMut = txt.parse()?;
            let name = doc["package"]["name"].as_str().unwrap_or("unknown");
            let ver = doc["package"]["version"].as_str().unwrap_or("missing");
            println!("  • {} → {}", name, ver);
        }
    } else {
        let txt = fs::read_to_string(root)?;
        let doc: DocumentMut = txt.parse()?;
        let name = doc["package"]["name"].as_str().unwrap_or("unknown");
        let ver = doc["package"]["version"].as_str().unwrap_or("missing");
        println!("📦 {} → {}", name, ver);
    }
    Ok(())
}
