#![feature(exit_status_error)]

use std::{io, io::ErrorKind, path::Path};

use clap::{Parser, Subcommand, ValueEnum};
use futures::future::{BoxFuture, FutureExt};
use tokio::{fs, fs::create_dir_all, process::Command};

use octant_error::OctantResult;

#[derive(Clone, ValueEnum, Debug)]
enum Profile {
    Release,
    Dev,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    profile: Option<Profile>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test,
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_impl().await {
        eprintln!("Error: {:?}", e);
    }
}

async fn main_impl() -> OctantResult<()> {
    let cli = Cli::parse();
    let profile = cli.profile.unwrap_or(Profile::Dev);
    let profile_name = match profile {
        Profile::Release => "release",
        Profile::Dev => "dev",
    };
    let profile_dir_name = match profile {
        Profile::Release => "release",
        Profile::Dev => "debug",
    };
    create_dir_all("../target/www/wasm-pack").await?;
    tokio::process::Command::new("wasm-pack")
        .args(&["--log-level", "warn"])
        .arg("build")
        .args(&["--target", "web"])
        .args(&["--out-dir", "../target/www/wasm-pack"])
        .arg("octant-client")
        .arg(&format!("--{profile_name}"))
        .status()
        .await?
        .exit_ok()
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    Command::new("cargo")
        .arg("build")
        .args(&["-p", "octant-scoreboard"])
        .args(&["--profile", profile_name])
        .status()
        .await?
        .exit_ok()
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    create_dir_all("target/db").await?;
    // copy_dir_all("octant-client/www".as_ref(), "target/www".as_ref()).await?;
    Command::new(&format!("target/{profile_dir_name}/octant-scoreboard"))
        .args(&["--bind-http", "0.0.0.0:8080"])
        .args(&["--bind-https", "0.0.0.0:8081"])
        .args(&["--cert-path", "octant-server/cert/certificate.pem"])
        .args(&["--key-path", "octant-server/cert/key.pem"])
        .args(&["--db-path", "target/db"])
        .env("RUST_BACKTRACE", "1")
        .env("RUST_LOG", "info")
        .status()
        .await?
        .exit_ok()
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    Ok(())
}
