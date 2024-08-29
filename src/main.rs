mod cli;
mod config;
mod file_operations;
mod git_operations;
mod secret;
mod tui;

use clap::Parser;
use cli::Args;
use colored::*;
use config::Config;
use file_operations::{create_dotfiles_repo, find_dotfiles, print_file_tree};
use git_operations::init_git_repo;
use std::fs;
use std::path::Path;
use tui::{get_repo_path, select_dotfiles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = fs::read_to_string(args.config)?;
    let config: Config = serde_yaml::from_str(&config)?;

    println!("{}", "Welcome to Dotfiles Manager!".green().bold());

    let repo_path = get_repo_path()?;

    println!("{}", "Searching for dotfiles...".yellow());
    let found_dotfiles = find_dotfiles(&config.dotfiles);

    println!("{}", "Found the following dotfiles:".green());
    for dotfile in &found_dotfiles {
        println!("  {}", dotfile.to_string_lossy().blue());
    }

    let selected_dotfiles = select_dotfiles(&found_dotfiles)?;

    println!("{}", "Creating dotfiles repo...".yellow());
    create_dotfiles_repo(&repo_path, &selected_dotfiles)?;
    println!("{}", "Initialising git...".yellow());
    init_git_repo(&repo_path)?;

    println!("{}", "Dotfiles repo created successfully!".green().bold());
    println!("Repo location: {}\n", repo_path.cyan());

    print_file_tree(Path::new(&repo_path), 0)?;

    Ok(())
}
