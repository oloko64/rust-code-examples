# Rust FFI (Foreign Function Interface)

Using Rust code to create a `.so` shared library that can be used by Node.js.

## Rust

### Compile the shared library

```bash
cd rust
cargo build --release
```

## Node.js

### Run the Node.js import example

```bash
cd node
yarn install
node index.js
```