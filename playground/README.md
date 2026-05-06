# Eres Playground

Live version:

https://justeres.github.io/facharbeit/

This folder contains the static browser playground for the Eres compiler.
The Rust compiler is compiled to WebAssembly, loaded directly in the browser, and used to compile Eres source code on the client side.

The playground includes:

* an editable source pane with lightweight syntax highlighting
* a token viewer with compact token cards
* an AST viewer with a scrollable and zoomable tree diagram
* a WAT viewer with syntax highlighting
* runtime output from the compiled `main` function
* a curated set of commented demo programs

## Local Development

Build the browser package from the repository root:

```bash
./scripts/build-playground.sh
```

Then serve the `playground/` directory with any static file server:

```bash
cd playground
python3 -m http.server 8000
```

Open `http://localhost:8000` in a browser.

## Deployment

The playground is designed for static hosting.
The GitHub Pages deployment publishes the built contents of `playground/` after running the WebAssembly build step in CI.
