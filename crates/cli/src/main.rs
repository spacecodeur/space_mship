use clap::Parser;
use colored::*;
use dotenv::dotenv;
use include_dir::{include_dir, Dir};
use std::env;
use std::path::Path;
use std::process::{Command, exit};
use walkdir::WalkDir;

mod service_maker;
use service_maker::ServiceMaker;

// Embed the skeleton templates at compile time
static SKELETON_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../commands/make/service/_maker_system/skeletons");

#[derive(Parser, Debug)]
#[command(name = "space_mship")]
#[command(about = "Space Microservices Platform CLI", long_about = None)]
struct Args {
    /// The command to execute (e.g., commands/services/web/container-management/build.sh)
    #[arg(value_name = "COMMAND")]
    command: Option<String>,
    
    /// Additional arguments to pass to the command
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();
    
    // Load .env file if it exists
    if dotenv().is_err() {
        eprintln!("{}", "Error: the .env file is missing.".red());
        exit(1);
    }
    
    // Get APP_NAME from environment
    let app_name = env::var("APP_NAME").unwrap_or_else(|_| {
        eprintln!("{}", "Error: APP_NAME not set in .env file".red());
        exit(1);
    });
    
    match args.command {
        None => show_help(),
        Some(command) => execute_command(&command, &args.args, &app_name),
    }
}

fn show_help() {
    println!("Usage: space_mship <command/subcommand/...>.sh [args...]");
    println!("\nAvailable commands:");
    
    let commands_dir = Path::new("./commands");
    if !commands_dir.exists() {
        println!("  No commands directory found. Run from project root.");
        return;
    }
    
    let mut commands: Vec<String> = Vec::new();
    
    for entry in WalkDir::new(commands_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sh") {
            let path_str = path.display().to_string();
            if !is_system_command(&path_str) {
                commands.push(path_str);
            }
        }
    }
    
    // Add built-in command
    commands.push("commands/make/service/new.sh (built-in)".to_string());
    
    commands.sort();
    for cmd in commands {
        println!("  {}", cmd);
    }
}

fn is_system_command(path: &str) -> bool {
    path.contains("/_")
}

fn is_running_in_docker() -> bool {
    Path::new("/.dockerenv").exists()
}

fn execute_command(command: &str, args: &[String], app_name: &str) {
    // Special handling for make service command
    if command == "commands/make/service/new.sh" {
        let maker = ServiceMaker::new(SKELETON_DIR.clone());
        if let Err(e) = maker.create_new_service() {
            eprintln!("{} {}", "Error:".red(), e);
            exit(1);
        }
        return;
    }
    
    // Check if command file exists
    if !Path::new(command).exists() {
        eprintln!("{} Unknown command: {}", "Error:".red(), command);
        show_help();
        exit(1);
    }
    
    // Check if it's a system command
    if is_system_command(command) {
        eprintln!("{} Unknown command: {}", "Error:".red(), command);
        show_help();
        exit(1);
    }
    
    // Parse command path
    let parts: Vec<&str> = command.split('/').collect();
    
    // Check if it's an in-container command that needs to run in Docker
    if parts.len() > 3 
        && parts[1] == "services" 
        && parts[3] == "in-container" 
        && !is_running_in_docker() 
    {
        let service = parts[2];
        let container_name = format!("{}-{}-container", app_name, service);
        
        let mut docker_cmd = Command::new("docker");
        docker_cmd
            .arg("exec")
            .arg("--tty")
            .arg("--interactive")
            .arg("--workdir")
            .arg("/app")
            .arg(&container_name)
            .arg("./cli.sh")  // Note: this assumes cli.sh exists in container
            .arg(command);
        
        for arg in args {
            docker_cmd.arg(arg);
        }
        
        let status = docker_cmd.status().unwrap_or_else(|e| {
            eprintln!("{} Failed to execute docker command: {}", "Error:".red(), e);
            exit(1);
        });
        
        exit(status.code().unwrap_or(1));
    } else {
        // Execute command directly
        let mut bash_cmd = Command::new("bash");
        bash_cmd.arg(command);
        
        for arg in args {
            bash_cmd.arg(arg);
        }
        
        let status = bash_cmd.status().unwrap_or_else(|e| {
            eprintln!("{} Failed to execute command: {}", "Error:".red(), e);
            exit(1);
        });
        
        exit(status.code().unwrap_or(1));
    }
}