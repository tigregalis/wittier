# `wittier`

Automatically generate WebAssembly Interface Type (WIT) definitions for Rust functions and types for a library, and their wasmtime host implementations.

Clone the target library.

Run the following command in the root.

```bash
cargo +nightly rustdoc --lib -- -Z unstable-options --output-format=json
# Alternatively you could potentially use `rustdoc_json`?
```

Copy the `target/doc/<crate-name>.json` file.

Run this tool on that file.
