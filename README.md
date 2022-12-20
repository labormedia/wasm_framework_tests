# wasm_framework_tests
Tests for multi_threaded wasm calls.

## Installation
`rustup target add wasm32-wasi`

`cargo build --target wasm32-wasi`

### Running functions with Wasmtime
`wasmtime --invoke print_hello target/wasm32-wasi/debug/wasm_framework_tests.wasm`

### Installing WasmEdge
https://wasmedge.org/book/en/quick_start/install.html
https://wasmedge.org/book/en/sdk/rust.html

### Running functions with WasmEdge
`wasmedge --reactor target/wasm32-wasi/release/wasm_framework_tests.wasm http_server`

## wasmedgec optimization
wasmedgec target/wasm32-wasi/debug/examples/libp2p.wasm target/wasm32-wasi/debug/examples/libp2p.so