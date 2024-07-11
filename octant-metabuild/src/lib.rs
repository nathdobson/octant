#![deny(unused_must_use)]
#![feature(exit_status_error)]

use serde::Deserialize;
use std::{
    fmt::{Display, Formatter},
    fs,
    fs::create_dir_all,
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Deserialize, Debug)]
struct Metadata {
    packages: Vec<Package>,
    workspace_root: String,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    manifest_path: PathBuf,
    metadata: Option<PackageMetadata>,
}

#[derive(Deserialize, Debug)]
struct PackageMetadata {
    #[serde(rename = "octant-metabuild")]
    octant_metabuild: Option<OctantMetabuild>,
}

#[derive(Deserialize, Debug)]
struct OctantMetabuild {
    side: Option<Side>,
    #[serde(rename = "shared-name")]
    shared_name: Option<String>,
    resources: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
enum Side {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "server")]
    Server,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Client => write!(f, "client"),
            Side::Server => write!(f, "server"),
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    let mut entries = fs::read_dir(src)?;
    for entry in entries {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn metabuild() {
    let package =
        std::env::var("CARGO_PKG_NAME").expect("Cannot find CARGO_PKG_NAME environment variable");
    let mut command = Command::new("cargo");
    command
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .stderr(Stdio::inherit());
    let json = command
        .output()
        .unwrap_or_else(|e| panic!("Failed to run cargo metadata for {:?}: {}", command, e));
    json.status
        .exit_ok()
        .unwrap_or_else(|e| panic!("Failed to run cargo metadata for {:?}: {}", command, e));
    let workspace_metadata: Metadata =
        serde_json::from_slice(&json.stdout).expect("Failed to parse metadata as JSON");
    let package = workspace_metadata
        .packages
        .iter()
        .find(|p| p.name == package)
        .expect("Failed to find current package");
    let package_metadata = package
        .metadata
        .as_ref()
        .expect("Current package has no metadata")
        .octant_metabuild
        .as_ref()
        .expect("Current package has no metadata for octant_metabuild");
    if let Some(side) = &package_metadata.side {
        println!("cargo::rustc-check-cfg=cfg(side, values(\"client\", \"server\"))");
        println!("cargo::rustc-cfg=side=\"{}\"", side);
    }
    if let Some(shared_name) = &package_metadata.shared_name {
        println!(
            "cargo::rustc-env=MARSHAL_OBJECT_RENAME_CRATE={}",
            shared_name
        );
    }
    if let Some(resources) = &package_metadata.resources {
        for resource in resources {
            let from = &package
                .manifest_path
                .parent()
                .expect("manifest has no parent directory")
                .join(resource);
            let to = &Path::new(
                &workspace_metadata.workspace_root,
            )
            .join("target")
            .join(resource)
            .join(&package.name);
            create_dir_all(to.parent().unwrap())
                .unwrap_or_else(|e| panic!("Could not create resource directory {:?} {}", to, e));
            fs::remove_file(to).unwrap();
            std::os::unix::fs::symlink(from, to).unwrap_or_else(|e| {
                panic!(
                    "Could not create symlink from {:?} to {:?}: {}",
                    from, to, e
                )
            });
            // copy_dir_all(from, to).expect("Cannot copy resources");
        }
    }
}
