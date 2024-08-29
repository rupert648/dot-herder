use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dotfile {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub dotfiles: Vec<Dotfile>,
}
