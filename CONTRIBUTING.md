# Contributing

## Ziel

Dieses Repository entwickelt eine eigene Programmiersprache und Compiler-Pipeline (Lexer -> Parser -> Codegen -> Wasm Runtime).
Änderungen sollen den Code robuster, klarer und testbar halten.

## Entwicklungs-Workflow

1. Kleine, isolierte Änderungen umsetzen.
2. Nach jedem relevanten Schritt Tests laufen lassen:

```bash
cargo test
```

3. Vor dem Commit Lints prüfen:

```bash
cargo clippy --tests -- -D warnings
```

4. API-Dokumentation lokal bauen:

```bash
cargo doc --no-deps
```

## Refactoring-Regeln

- Keine stillen Panics auf Nutzerinput (`[]`-Indexing, `unwrap`) in Compilerpfaden.
- Fehler als `Result<_, ErrorType>` propagieren und sauber in der CLI ausgeben.
- Öffentliche Typen/Funktionen mit Rustdoc versehen (`///`), inklusive kurzer Zweckbeschreibung.
- Tests bei Bugfixes oder neuen Edgecases direkt ergänzen.

## Test-Schwerpunkte

- Lexer: Tokenisierung, Kommentare, ungültige Zeichen.
- Parser: Precedence, Blockstruktur, Fehlersituationen.
- Codegen/Runner: Kontrollfluss (`if`/`while`/`return`), Arity-Mismatches, Host-Calls (`print`).

## Commit-Stil

- Präfixe verwenden: `refactor:`, `fix:`, `docs:`, `test:`
- Commit-Nachrichten auf eine klar abgegrenzte Änderung beschränken.
