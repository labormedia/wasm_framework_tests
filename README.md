# wasm_framework_tests
Tests for multi_threaded wasm calls.

## Installation
`cargo add wasm32-wasi`

`cargo build --target wasm32-wasi`

### Running functions with Wasmtime
`wasmtime --invoke print_hello target/wasm32-wasi/debug/wasm_framework_tests.wasm`

### Installing WasmEdge
https://wasmedge.org/book/en/quick_start/install.html