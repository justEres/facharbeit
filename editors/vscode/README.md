# VS Code support for `eres`

## Development

```bash
cd editors/vscode
npm install
npm run build
```

### Run in VS Code

1. Open `/home/eres/coding/facharbeit/editors/vscode` in VS Code.
2. Go to `Run and Debug`.
3. Start `Run eres Extension` or press `F5`.
4. In the new Extension Development Host window, open `/home/eres/coding/facharbeit`.
5. Open any `.eres` file to activate the extension.

The extension starts the language server in one of two ways:

- `eres.languageServer.path` set: starts that binary directly
- default: runs `cargo run --quiet --bin eres-lsp` in the workspace root

## Current features

- Syntax highlighting for `.eres`
- Diagnostics from lexer, parser and type checker
- Hover for:
  - functions with full signatures
  - parameters and locals with inferred/declared types
  - structs with field types
  - enums with variant shapes
- Go to Definition for:
  - functions
  - parameters
  - local variables
  - structs and enums
- Completion for:
  - functions
  - parameters
  - local variables
  - structs and enums
  - enum variants after `EnumName::`
  - list methods after `.`
- Document Symbols / Outline for top-level items and function-local symbols

## Logs

- Extension-side logs: `View > Output > eres Language Server`
- Extension host logs: `View > Output > Log (Extension Host)`

Open this folder in VS Code and press `F5` to launch an Extension Development Host.
