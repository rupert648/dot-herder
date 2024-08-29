use std::process::Command;

pub fn init_git_repo(repo_path: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()?;
    Ok(())
}
