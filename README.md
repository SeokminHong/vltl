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

## Development

### Running Tests

#### Unit Tests

Run Rust unit tests:

```sh
cargo test
```

#### E2E Tests

End-to-end tests are written in Fish shell. To run them:

1. Install fish shell:
   ```sh
   # On Ubuntu/Debian
   sudo apt-get install fish
   
   # On macOS
   brew install fish
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the e2e tests:
   ```sh
   PATH=$PATH:$(pwd)/target/release fish e2e_test.fish
   ```

The e2e tests verify:
- Korean to English conversion (`vltl convert`)
- Korean detection (`vltl has-korean`)
- Fish shell integration (`vltl init`)
- Command checking functionality

### GitHub Actions

The project uses GitHub Actions for continuous integration:

- **Build workflow** (`.github/workflows/build.yml`): Runs unit tests and builds the project
- **E2E Tests workflow** (`.github/workflows/e2e.yml`): Runs end-to-end tests with fish shell

Both workflows run on every push and pull request to the `main` branch.
