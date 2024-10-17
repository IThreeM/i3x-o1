use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "i3x-o1")]
#[command(about = "An internal CLI tool to build, serve, and deploy the I3M Developer Docs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build and serve the developer docs locally
    Run,
    /// Deploy the developer docs to the Internet Computer Protocol (ICP)
    Deploy,
}

fn main() {
    let cli = Cli::parse();

    // Install mdbook before proceeding
    install_mdbook();

    match &cli.command {
        Commands::Run => run_docs(),
        Commands::Deploy => deploy_to_icp(),
    }
}

fn install_mdbook() {
    println!("Installing mdbook...");

    let install_status = Command::new("cargo")
        .arg("install")
        .arg("mdbook")
        .status()
        .expect("Failed to install mdbook.");

    if !install_status.success() {
        eprintln!("Failed to install mdbook.");
        std::process::exit(1);
    } else {
        println!("mdbook installed successfully.");
    }
}

fn run_docs() {
    println!("Building and serving the developer docs...");

    let build_status = Command::new("mdbook")
        .arg("build")
        .arg("src/Developer-Docs")
        .status()
        .expect("Failed to build the mdbook project.");

    if !build_status.success() {
        eprintln!("Failed to build the developer docs.");
        std::process::exit(1);
    }

    let serve_status = Command::new("mdbook")
        .arg("serve")
        .arg("src/Developer-Docs")
        .status()
        .expect("Failed to serve the mdbook project.");

    if !serve_status.success() {
        eprintln!("Failed to serve the developer docs.");
        std::process::exit(1);
    }
}

fn deploy_to_icp() {
    println!("Starting local DFX replica...");

    let start_status = Command::new("dfx")
        .arg("start")
        .arg("--clean")
        .arg("--background")
        .status()
        .expect("Failed to start DFX replica.");

    if !start_status.success() {
        eprintln!("Failed to start DFX replica.");
        std::process::exit(1);
    }

    println!("Deploying to ICP...");

    let deploy_status = Command::new("dfx")
        .arg("deploy")
        .status()
        .expect("Failed to deploy the canisters.");

    if !deploy_status.success() {
        eprintln!("Deployment to ICP failed.");
        std::process::exit(1);
    } else {
        println!("Deployment to ICP was successful.");
    }
}
