# Game of Life Web

A React implementation of Conway's Game of Life using Rust WebAssembly.

## Prerequisites

- Rust toolchain with wasm32 target
- Node.js and npm
- wasm-bindgen-cli

## Building the WASM Module

```bash
# Build WASM and generate bindings
just build-wasm
```

Or manually:

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown -p wasm
mkdir -p game-of-life-web/wasm
wasm-bindgen --target web --out-dir game-of-life-web/wasm target/wasm32-unknown-unknown/release/wasm.wasm
```

## Running the React App

```bash
cd game-of-life-web
npm install
npm run dev
```

## Features

- **Interactive Grid**: Click on cells to toggle them alive/dead
- **Animation Controls**: Start, pause, and reset the simulation
- **Speed Control**: Adjust the simulation speed with a slider
- **Generation Counter**: Track the current generation number
- **Live Cell Counter**: See how many cells are currently alive

## Game Rules

1. Any live cell with fewer than two live neighbors dies (underpopulation)
2. Any live cell with two or three live neighbors lives on
3. Any live cell with more than three live neighbors dies (overpopulation)
4. Any dead cell with exactly three live neighbors becomes alive (reproduction)

## Controls

- **Click on grid**: Toggle cell state
- **Start/Pause**: Toggle animation
- **Reset**: Reset to initial glider pattern
- **Clear**: Clear all cells
- **Speed slider**: Adjust animation speed (10ms - 500ms)

## Technical Details

- **Frontend**: React with TypeScript
- **Build Tool**: Vite
- **WASM**: Rust compiled to WebAssembly
- **Rendering**: HTML5 Canvas
