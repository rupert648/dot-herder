# DotHerder 🐑

DotHerder is a friendly CLI tool that helps you round up and manage your dotfiles with ease. It searches for common configuration files, lets you choose which ones to include, and creates a neat little repository for them. No more scattered configs!

## What DotHerder Does

- 🔍 Searches your system for common dotfiles
- ✅ Lets you select which files to include
- 📁 Creates a new repository for your dotfiles
- 🔗 Symlinks the selected files to the new repo
- 🌳 Displays a pretty tree structure of your new dotfiles repo

## Building DotHerder

1. Make sure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone https://github.com/yourusername/dotherder.git
   cd dotherder
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Using DotHerder

1. Create a YAML configuration file (or use the existing one under config.yaml) listing the dotfiles you want to search for. Here's an example:

   ```yaml
   dotfiles:
     - name: .zshrc
       path: ~/
     - name: .vimrc
       path: ~/
     - name: .gitconfig
       path: ~/
   ```

2. Run DotHerder:
   ```
   ./target/release/dotherder --config dotfiles_config.yaml
   ```

3. Follow the prompts to select your dotfiles and create your new repo.

4. Enjoy your newly organized dotfiles!

## Contributing

Feel free to open issues or submit pull requests if you have ideas for improvements or new features. Let's make dotfile management a breeze together!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
