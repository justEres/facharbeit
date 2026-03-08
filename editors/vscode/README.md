# VS Code support for `eres`

## Development

```bash
cd editors/vscode
npm install
npm run build
```

The extension starts the language server in one of two ways:

- `eres.languageServer.path` set: starts that binary directly
- default: runs `cargo run --quiet --bin eres-lsp` in the workspace root

Open this folder in VS Code and press `F5` to launch an Extension Development Host.
