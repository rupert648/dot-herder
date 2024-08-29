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

## Using DotHerder

1. Use the existing yaml file (under `config.yaml`) or create your own config, this contains a list of the dotfiles you want to search for. Here's an example:

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

It would be great if config.yaml needed very little editing and covered most common .dotfiles, so please feel free to open a PR and add any you yourself find helpful!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
