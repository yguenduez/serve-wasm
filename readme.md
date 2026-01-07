# serve-wasm

A simple, lightweight HTTP server specifically designed for serving WebAssembly applications with the correct security headers.

## Features

- Serves static files from the current directory
- Automatically adds required WASM headers:
  - `Cross-Origin-Opener-Policy: same-origin`
  - `Cross-Origin-Embedder-Policy: require-corp`
  - `Cross-Origin-Resource-Policy: cross-origin`
- Serves `index.html` at the root path

## Installation

```sh
cargo install serve-wasm
```

## Usage

```sh
serve-wasm 8080
```
