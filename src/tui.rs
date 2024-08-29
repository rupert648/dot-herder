use colored::*;
use dialoguer::{Input, MultiSelect};
use std::path::PathBuf;

use crate::file_operations::is_valid_repo_path;

fn prompt_for_repo_path() -> Result<String, std::io::Error> {
    Input::new()
        .with_prompt("Enter the path for your .dotfiles repo".cyan().to_string())
        .interact_text()
}

pub fn get_repo_path() -> Result<String, Box<dyn std::error::Error>> {
    let mut path: String;
    loop {
        path = prompt_for_repo_path()?;
        match is_valid_repo_path(&path) {
            Ok(()) => break,
            Err(e) => {
                println!("Invalid path: {}", e);
                println!("Please supply another");
            }
        }
    }

    Ok(path)
}

pub fn select_dotfiles(dotfiles: &[PathBuf]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
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
