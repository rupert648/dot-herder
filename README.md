# DotHerder 🐑

DotHerder is a friendly CLI tool that helps you round up and manage your dotfiles with ease. It searches for common configuration files, lets you choose which ones to include, and creates a neat little repository for them. No more scattered configs!

## What DotHerder Does

- 🔍 Searches your system for common dotfiles
- ✅ Lets you select which files to include
- ⚠️  Checks those files for any sus secrets
- 📁 Creates a new repository for your dotfiles
- 🔗 Symlinks the selected files to the new repo
- 🌳 Displays a pretty tree structure of your new dotfiles repo

## Building DotHerder

1. Make sure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone https://github.com/rupert648/dot-herder.git
   cd dot-herder
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Use the existing yaml file (under `config.yaml`) or create your own config (see config section):

   ```yaml
   dotfiles:
     - name: ${HOME}/.zshrc
     - name: ${HOME}/.bash_profile
     - name: ${HOME}/.bashrc
     - name: ${HOME}/.vimrc
   ```

2. Run DotHerder:
#### Basic Syntax
dot-herder [OPTIONS] --config <CONFIG>
Copy
##### Options

- `-c, --config <CONFIG>`: Path to the configuration file (required)
- `--home <HOME>`: Optional home directory path. Defaults to home (~/)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

##### Examples

1. Using a config file with default home directory:
```bash
dot-herder --config ~/.dotfiles/config.yaml
```
3. Specifying a custom home directory:
```bash
dot-herder --config ~/.dotfiles/config.yaml --home /custom/home/path
```
5. Viewing help information:
```bash
dot-herder --help
```
7. Follow the prompts to select your dotfiles and create your new repo.

8. Enjoy your newly organized dotfiles!

## Config Structure

The `dotfiles` section contains a list of dotfile entries:

- `name`: Path to the dotfile, using `${HOME}` as a placeholder for the user's home directory. Home is replaced by the `--home` argument, defaulting to `~`

## Contributing

Feel free to open issues or submit pull requests if you have ideas for improvements or new features. Let's make dotfile management a breeze together!

It would be great if config.yaml needed very little editing and covered most common .dotfiles, so please feel free to open a PR and add any you yourself find helpful!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
