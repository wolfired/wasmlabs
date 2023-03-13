# wasmlabs

# Build res

```bash

rustc --crate-type cdylib --edition 2021 --target wasm32-unknown-unknown --out-dir res res/lib.rs

```

# Build and Run

```bash

cargo build --bin wasm_bin_reader
cargo run --bin wasm_bin_reader

```

# Debug with gdb

```bash

./tmux_gdb.sh --args ./target/debug/wasm_bin_reader

```

# Issue

```

#
# warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
#
# add the following to .gdbinit
# https://github.com/rust-lang/rust/issues/33159
#
add-auto-load-safe-path ~/.rustup/toolchains
dir ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/etc

``

