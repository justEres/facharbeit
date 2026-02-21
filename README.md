> Facharbeits-Archiv (eingefroren): https://github.com/justEres/facharbeit/tree/facharbeit-archiv

# Facharbeit (Programmiersprache)

Dieses Repository fokussiert jetzt auf die Implementierung der Programmiersprache.
Der vorherige Stand mit Facharbeit/Typst-Material bleibt dauerhaft im Branch `facharbeit-archiv`.

## Projekt starten

```bash
cargo run -- examples/add_compare.eres
```

Optionale Debug-Ausgaben:

```bash
cargo run -- examples/add_compare.eres --print-tokens --print-ast --print-wat
```

## Projektplan

### Phase 1: Aufräumen und Grundlage stabilisieren

1. Refactoring der Compiler-Pipeline (`lexer`, `parser`, `ast`, `codegen`, `runner`) mit klareren Modulgrenzen.
2. Interne API vereinheitlichen (Fehler-Typen, Namenskonventionen, Datenflüsse).
3. Dokumentation im Code ausbauen und mit `cargo doc` eine verlässliche Entwicklerdoku erzeugen.
4. Kleine Testbasis für Lexer/Parser/Codegen schaffen, damit Refactorings sicher bleiben.

### Phase 2: Type-System planen und implementieren

1. Typmodell definieren (primitive Typen, Funktionen, Structs, Enums, Referenzen).
2. Type-Checking als eigene Compiler-Phase einführen.
3. Fehlermeldungen für Typfehler verbessern (Ort, Ursache, erwarteter/gefundenen Typ).
4. Vorbereitung für spätere Generics und Trait-ähnliche Konzepte.

### Phase 3: Language Design (Rust-Lite / Rust-Script)

1. Syntax orientiert sich an Rust.
2. Objektorientierte Nutzung über Methoden an `struct`s (impl-artig).
3. Kein Borrow-Checker: stattdessen einfacher Runtime-Ansatz mit Reference Counting.
4. Fat Enums (Sum Types mit Daten pro Variante) als Kernfeature.
5. Ziel bleibt WebAssembly als Laufzeitplattform für eine pragmatische Skriptsprache.

## Geplante Features (Vorschläge)

1. Pattern Matching auf Enums mit `match` und Exhaustiveness-Check.
2. `impl`-Blöcke für Methoden und assoziierte Funktionen.
3. Generics für `struct`, `enum` und Funktionen.
4. Traits/Limits in vereinfachter Form (Interfaces ohne komplexes Lifetimesystem).
5. Modul-System mit `mod`/`use` und klaren Sichtbarkeiten (`pub`).
6. Fehlerbehandlung über `Result`/`Option`-artige Standardtypen.
7. String- und Collections-Basics (`Vec`-ähnlich) in einer kleinen Standardbibliothek.
8. Type Inference für lokale Variablen und Funktionsrückgaben, wo eindeutig.
9. `let`/`mut`-Semantik und kontrollierte Immutability für lesbaren Script-Code.
10. Tooling: Formatter, bessere CLI-Fehlerausgaben, optionales REPL.
