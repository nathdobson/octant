#![deny(unused_must_use)]
#![feature(exit_status_error)]

use std::{
    fmt::{Display, Formatter},
    process::{Command, Stdio},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    metadata: Option<PackageMetadata>,
}

#[derive(Deserialize, Debug)]
struct PackageMetadata {
    #[serde(rename = "octant-sys-build")]
    octant_sys_build: Option<OctantSysBuildMetadata>,
}

#[derive(Deserialize, Debug)]
struct OctantSysBuildMetadata {
    side: Side,
    #[serde(rename = "shared-name")]
    shared_name: String,
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

pub fn metabuild() {
    let package = std::env::var("CARGO_PKG_NAME").unwrap();
    let json = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    json.status.exit_ok().unwrap();
    let metadata: Metadata = serde_json::from_slice(&json.stdout).unwrap();
    let package = metadata
        .packages
        .iter()
        .find(|p| p.name == package)
        .unwrap();
    let metadata = package
        .metadata
        .as_ref()
        .unwrap()
        .octant_sys_build
        .as_ref()
        .unwrap();
    println!("cargo::rustc-check-cfg=cfg(side, values(\"client\", \"server\"))");
    println!("cargo::rustc-cfg=side=\"{}\"", metadata.side);
    println!(
        "cargo::rustc-env=MARSHAL_OBJECT_RENAME_CRATE={}",
        metadata.shared_name
    );
}
