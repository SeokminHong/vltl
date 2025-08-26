# vltl (피시)

Replace a 2-set Korean keyboard typo to the English command. It also changes your IME mode the English.

## Installation

### Homebrew

```sh
brew tap seokminhong/brew
brew install vltl
# Or
brew install seokminhong/brew/vltl
```

### Cargo install

```sh
cargo install vltl
```

## Usage

1. Add the script to your config.fish file

   ```fish
   # ~/.config/fish/config.fish
   vltl init | source
   ```

2. Try Korean typo

   ```
   $ ㅔㅞㅡ --version
   vltl: New alias (ㅔㅞㅡ -> pnpm)
   10.5.0
   ```

https://github.com/user-attachments/assets/3118923a-edd9-4fc0-9688-ad3a0bee7a23
