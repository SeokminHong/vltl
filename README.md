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

2. Register your abbreviations as usual

   ```fish
   abbr -a L --position anywhere --set-cursor "% | less"
   ```

3. Type Korean by mistake — vltl corrects it before `abbr` expands

   When you type `cat foo.txt ㅣ` and press space:
   - `ㅣ` is converted to `L`
   - fish `expand-abbr` runs and expands `L` to `| less`
   - The Korean trigger `ㅣ` is auto-registered as an abbr for next time
   - On macOS, the IME switches to English

   vltl binds `space`, `enter`, and `;` so that any Korean token is converted to its QWERTY equivalent before fish's native abbreviation expansion runs.

## Development

### Running Tests

#### Unit Tests

Run Rust unit tests:

```sh
cargo test
```

#### E2E Tests

End-to-end tests verify the fish shell abbr integration. To run them:

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
- Function definitions after sourcing init (`__vltl_convert_and_expand`, `__vltl_auto_register_abbr`, etc.)
- Korean-to-English conversion via `vltl convert`
- Korean detection via `vltl has-korean`
- Automatic abbr registration for Korean triggers
- Preservation of abbr options (`--position anywhere`, `--set-cursor`, etc.)
- No duplicate abbr registration
- `VLTL_PATH` environment variable support
- IME switching on macOS

### GitHub Actions

The project uses GitHub Actions for continuous integration:

- **Build workflow** (`.github/workflows/build.yml`): Runs unit tests and builds the project
- **E2E Tests workflow** (`.github/workflows/e2e.yml`): Runs end-to-end tests with fish shell

Both workflows run on every push and pull request to the `main` branch.
