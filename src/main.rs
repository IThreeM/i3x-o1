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

    match &cli.command {
        Commands::Run => run_docs(),
        Commands::Deploy => deploy_to_icp(),
    }
}

fn run_docs() {
    println!("Building and serving the developer docs...");

    // Run mdbook build command
    let build_status = Command::new("mdbook")
        .arg("build")
        .arg("src/Developer-Docs")
        .status()
        .expect("Failed to build the mdbook project.");

    if !build_status.success() {
        eprintln!("Failed to build the developer docs.");
        std::process::exit(1);
    }

    // Run mdbook serve command
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

    // Run dfx start --clean --background
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

    // Run dfx deploy command
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
