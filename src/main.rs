use clap::Parser;
use colored::*;
use dialoguer::{Input, MultiSelect};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use symlink::symlink_file;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    config: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dotfile {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DotfilesList {
    dotfiles: Vec<Dotfile>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = fs::read_to_string(&args.config)?;
    let dotfiles_list: DotfilesList = serde_yaml::from_str(&config)?;

    println!("{}", "Welcome to Dotfiles Manager!".green().bold());

    let repo_path: String = Input::new()
        .with_prompt("Enter the path for your .dotfiles repo".cyan().to_string())
        .interact_text()?;

    println!("{}", "Searching for dotfiles...".yellow());
    let found_dotfiles = find_dotfiles(&dotfiles_list.dotfiles);

    println!("{}", "Found the following dotfiles:".green());
    for dotfile in &found_dotfiles {
        println!("  {}", dotfile.to_string_lossy().blue());
    }

    let selected_dotfiles = select_dotfiles(&found_dotfiles)?;

    println!("{}", "Creating dotfiles repo...".yellow());
    create_dotfiles_repo(&repo_path, &selected_dotfiles)?;

    println!("{}", "Dotfiles repo created successfully!".green().bold());
    println!("Repo location: {}\n", repo_path.cyan());

    print_file_tree(Path::new(&repo_path), 0)?;

    Ok(())
}

fn find_dotfiles(dotfiles: &[Dotfile]) -> Vec<PathBuf> {
    dotfiles
        .iter()
        .filter_map(|dotfile| {
            let path =
                PathBuf::from(shellexpand::tilde(&dotfile.path).to_string()).join(&dotfile.name);
            if path.exists() {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn select_dotfiles(dotfiles: &[PathBuf]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let dotfiles_strings: Vec<String> = dotfiles
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    let selections = MultiSelect::new()
        .with_prompt("Select dotfiles to include".cyan().to_string())
        .items(&dotfiles_strings)
        .defaults(&vec![true; dotfiles.len()])
        .interact()?;

    Ok(selections
        .into_iter()
        .map(|i| dotfiles[i].clone())
        .collect())
}

fn create_dotfiles_repo(
    repo_path: &str,
    dotfiles: &[PathBuf],
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(repo_path)?;

    for dotfile in dotfiles {
        let target = Path::new(repo_path).join(dotfile.file_name().unwrap());
        symlink_file(dotfile, &target)?;
        println!(
            "  {} {}",
            "Symlinked:".green(),
            dotfile.to_string_lossy().blue()
        );
    }

    println!("{}", "Initializing git repository...".yellow());
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()?;

    println!("{}", "Git repository initialized.".green());

    Ok(())
}

fn print_file_tree(dir: &Path, level: usize) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.file_name().unwrap() == ".git" {
                continue;
            }

            let prefix = if level == 0 {
                "".to_string()
            } else {
                format!("{}└─ ", "   ".repeat(level - 1))
            };

            if path.is_dir() {
                println!(
                    "{}{}/",
                    prefix.blue(),
                    path.file_name().unwrap().to_string_lossy().cyan()
                );
                print_file_tree(&path, level + 1)?;
            } else {
                println!(
                    "{}{}",
                    prefix.blue(),
                    path.file_name().unwrap().to_string_lossy().green()
                );
            }
        }
    }
    Ok(())
}
