# Usage

## Create symlink to binary

1. Create symlink
    ```bash
   cp target/release/<BIN_FILE> /usr/bin/
   ln -s $(pwd)/target/release/<BIN_FILE> /usr/bin/<BIN_FILE>
   ```

## Build
_Use [Rust](https://www.rust-lang.org/) for build._

1. Build binary
    ```bash
    cargo build --release
    ```
1. Create symlink