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

Nur prüfen (ohne Ausführung):

```bash
cargo run -- examples/add_compare.eres --check
```

Explizite `main`-Argumente:

```bash
cargo run -- examples/add_compare.eres --args "1,2,3"
```

Artefakte in Dateien schreiben:

```bash
cargo run -- examples/add_compare.eres --emit-tokens tokens.txt --emit-ast ast.txt --emit-wat out.wat --emit-wasm out.wasm
```

Mini-REPL (Expressions oder vollständige Programme):

```bash
cargo run -- --repl
```

Beispiele im REPL:

```text
eres> 1 + 2 * 3
= 7

eres> fn main() -> Int { return 40 + 2; }
= 42
```

## CLI Beispiele

Expression schnell per REPL:

```bash
cargo run -- --repl
# dann z.B. eingeben: (8 + 2) * 5
```

Nur kompilieren/checken:

```bash
cargo run -- examples/add_compare.eres --check
```

Mehrere Dateien über `use` laden:

```bash
cargo run -- path/to/main.eres
```

Programm mit konkreten Argumenten ausführen:

```bash
cargo run -- examples/add_compare.eres --args "1,3"
```

WAT/WASM für Inspektion exportieren:

```bash
cargo run -- examples/add_compare.eres --emit-wat build/out.wat --emit-wasm build/out.wasm
```

## Beispiele im `examples`-Ordner

Im `examples/`-Ordner stehen aktuell sowohl vollständige Laufzeitbeispiele als auch reine Type-System-Validierungsbeispiele:

- `add_compare.eres` (Legacy-Beispiel, weiterhin nutzbar)
- `run_arith.eres`
- `run_float_cond.eres`
- `check_refs_enums.eres`
- `check_aggregates.eres`
- `check_match.eres`

Konventionen:

- `run_*.eres`: werden vollständig über `compile_source` kompiliert und ausgeführt.
- `check_*.eres`: werden mit `compile_source_check` validiert (Frontend-only).

Tests zum schnellen Durchlauf:

```bash
cargo test compile_examples_check_pass
cargo test compile_examples_runtime
cargo test
```

Die Test-Assertions prüfen:
- Parsing/Typprüfung bei `check_*`-Dateien
- vollständige Kompilierung + Laufzeitresultat bei `run_*`-Dateien
- erwartete Codegen-Limitierungen (`CodegenError`) für aktuelle Grenzen (z. B. Frontend-Features ohne Backend-Unterbau)

## Module

Das Sprach-Frontend unterstützt jetzt flache Dateimodule:

```rust
use "./math.eres";

fn main() -> Int {
    return helper();
}
```

Regeln in v1:

- `use` erwartet immer einen String-Pfad auf eine `.eres`-Datei
- Pfade sind relativ zur importierenden Datei
- importierte Top-Level-Symbole werden global in einen gemeinsamen Scope gemerged
- doppelte Funktions-/Struct-/Enum-Namen sind harte Fehler
- Namespaces, `mod`, Aliase und selektive Imports gibt es in dieser Phase noch nicht

## Host Stdlib

Die Standardbibliothek wird implizit aus Rust bereitgestellt und steht global zur Verfügung.
Aktuell registrierte Host-Funktionen sind unter anderem:

- `print(String) -> Unit`
- `print_int(Int) -> Unit`
- `print_float(Float) -> Unit`
- `print_bool(Bool) -> Unit`
- `len(String) -> Int`
- `add_one(Int) -> Int`
- `is_positive(Int) -> Bool`
- `half(Float) -> Float`

Die ABI dafür lebt in der Workspace-Crate `crates/eres_abi`.
Weitere Rust-Crates können über das Makro `eres_host_function!` neue Host-Funktionen für `eres` registrieren.
Benannte Rust-`struct`s und `enum`s können über `#[derive(EresAbi)]` in die ABI eingebunden werden.

## Type-System-Design

Die Sprach-Pipeline ist jetzt:

`Lexer -> Parser -> Type Checker -> Codegen`

- Primitive Typen: `Int`, `Float`, `Bool`, `String`
- Listen: `List<T>` (homogene variable Länge)
- Tupel-Typen: `(Int, Float)` / `(Int, Bool, List<Int>)`
- Referenztypen: `&T` (nur explicit, kein Auto-Referenzieren)
- Funktions-Typen: `fn(Int, Float) -> Bool`
- Sum-Typen: `enum Name { A, B(T), C { x: T } }`
- Aggregierte Typen: `struct Name { x: T, y: T }`
- Listen-Methoden (Front-End): `xs.len()`, `xs.get(i)`, `xs.push(v)`, `xs.pop()`

Listen-/Tupel-Syntax:

- Listenliteral: `[1, 2, 3]`, `[]` (nur mit Zieltyp, z. B. `let xs: List<Int> = [];`)
- Tupelliteral: `(1, 2.0, true)`
- String-Literal: `"hello"`, inklusive einfacher Escapes wie `\n`, `\t`, `\"`
- Tupel-Typen sind feste Länge.
- Tupelindizierung im Typpfad über Feldzugriff: `value.0` (erstes Feld), `value.1` (zweites Feld), ...

### Explizite Referenzen + Auto-Deref

- Referenzbildung erfolgt mit `&expr` und Dereferenzierung mit `*expr`.
- Übergaben sind explizit: `&x` für `&T`, nicht `x`.
- Der Type Checker erlaubt Auto-Deref, wenn eine Referenz dort sitzt, wo ein Wert erwartet wird.
- Auto-Deref wird auch in Initialisierungen (z. B. Struct/Enum Payloads) und bei Rückgabe-/Funktionsargumenten angewendet.
- `[]` ohne Typkontext ist ein Typfehler (`TypeError`).

Beispiel:

```rust
fn inc(x: Int) -> Int { return x + 1; }

fn main() -> Int {
    let x: Int = 41;
    let p = &x;
    let y = *p;   // erlaubt, weil `Int` erwartet wird
    return inc(y);
}
```

### Structs / Enums / Match

```rust
enum Result {
    Ok,
    Err(Int),
    Pair { x: Int, y: Float }
}

fn demo(v: Result) -> Int {
    return match v {
        Ok => 1,
        Err(code) => code,
        Pair { x, y } => x,
    };
}
```

`match` prüft:

- alle Pattern referenzieren echte Varianten
- die Anzahl der Varianten ist vollständig (`non-exhaustive` wird verhindert)
- alle Arm-Typen sind konsistent

### Aktueller Backend-Status

- `Int` liegt als `i64`, `Bool` als `i32`, `Float` als `f64` in WebAssembly vor.
- `String`, Listen, Tupel, Structs und Enums laufen an der Host-Grenze über Runtime-Handles (`i32`) mit nominaler Typprüfung.
- String-Literale und String-Vergleiche (`==`, `!=`) werden über Runtime-Imports in Wasm eingebunden.
- `struct`, `enum` und Referenz-Expressions (`&`, `*`) werden als sprachinterne Konstruktionen weiterhin noch nicht direkt in Wasm gelowered und erzeugen dort klare Fehlermeldungen.
- Listen- und Tupel-Methoden (`.len()`, `.get()`, `.push()`, `.pop()`) sind Frontend-typisiert; das Codegen ist dafür noch nicht implementiert.
- Listen und Tupel sind als Aggregate im Typ-System enthalten; Host-Funktionen können sie bereits vollständig roundtrippen.

### Host-ABI-Modell

- Skalare Werte bleiben direkt: `Int`, `Float`, `Bool`, `Unit`
- Nicht-Skalare laufen als Handles: `String`, `List<T>`, Tupel, Structs, Enums
- Benannte Typen sind nominal und werden in Rust über Typ-Deskriptoren registriert
- Host-Funktionen können komplexe Werte lesen und neue Werte zurückgeben, ohne rohe Handles anfassen zu müssen

Rust-Seite, vereinfacht:

```rust
use eres_abi::{EresAbi, eres_host_function};

#[derive(Clone, EresAbi)]
struct User {
    name: String,
    active: bool,
}

fn make_user() -> User {
    User { name: "Ada".into(), active: true }
}

let host = eres_host_function!(make_user, name = "make_user", params = [], result = User);
```

### Listen/Tupel-Beispiele

```rust
fn head(x: List<Int>) -> Int { return x[0]; }
fn head2(x: List<Int>) -> Int { return x.get(0); }

fn meta(x: (Int, Float)) -> Int { return x.0; }

fn pair() -> (Int, Float) { return (1, 2.0); }
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

## Tooling Backlog

1. `.eres` Formatter (`fmt`-Subcommand) bauen, damit Code-Stil automatisch vereinheitlicht wird.

## VS Code Support

Es gibt jetzt eine erste VS-Code-Integration unter `editors/vscode`:

- TextMate-Syntax-Highlighting fuer `.eres`
- Language-Configuration fuer Kommentare/Klammern
- LSP-Client fuer Diagnosen und Hover

Language Server lokal starten:

```bash
cargo run --bin eres-lsp
```

Extension bauen:

```bash
cd editors/vscode
npm install
npm run build
```

Standardmaessig startet die Extension den Server ueber `cargo run --quiet --bin eres-lsp`
im Workspace-Root. Alternativ kann `eres.languageServer.path` auf ein gebautes Binary zeigen.
