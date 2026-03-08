# Zielbild für Type-System & Type Checker (Rust-ähnlich, explizite Referenzen)

## 1) Kurzfassung
Das Language-Frontend soll in drei Phasen arbeiten:  
`Lexer/Parser -> Type Checker -> Codegen`.  
Erlaubte Primitive werden `Int`, `Float`, `Bool`, plus `Struct`, `Enum` (Rust-fähig mit Payloads) und `&T`-Referenzen.  
Referenzen bleiben explizit (kein Auto-Referenzieren), aber der Checker erlaubt automatische Dereferenzierung in Kontexten, in denen ein Wert erwartet wird.  
`match` wird als Kernmechanik für fat enums aufgenommen, `if/while` erwarten `Bool`, Funktions-Rückgabepfade werden typpositions-konsistent geprüft.

## 2) Änderungsvorhaben (entscheidungsfest)

### A. AST + Parser (Sprachsyntax)
- `Type`-Syntax im AST erweitern:
  - `Int`, `Float`, `Bool`
  - `Ident(String)` für benannte Typen
  - `StructType { name, fields }`
  - `EnumType { name, variants }` mit Varianten `Name` / `Name(T)` / `Name { field: Type }`
  - `Ref(Type)` (ohne `&mut` im MVP)
  - `Fn(...) -> ...` als Funktions-Typ, wenn nötig
- Parser-Änderungen:
  - Typannotationen für Parameter und Rückgabewerte verpflichtend
  - `Struct`/`Enum`-Top-level-Deklarationen
  - `match`-Ausdruck inkl. Varianten-Pattern (inkl. Destrukturierung)
  - Referenzoperatoren: `&expr` (Adressbildung), `*expr` (explizite Deref im Code)
  - `Bool`-Literal `true/false`, `Float`-Literal

### B. Type Checker als neue Pipeline-Phase
- Neue Module: `src/typing/` (oder `src/type_checker/`) mit:
  - `TypeEnv` / Symboltabellen (globale Typen + Funktionen + lokale Scopes)
  - `Typer` als Walk über AST mit Fehler-Mapping auf Span
  - `typed_ast`- oder `TypedExpr`-Abbildung (typannotiert/annotierbar)
- Kernregeln:
  - Typkonsistenz für BinOps und Call-Arity + Typhierarchie
  - `if/while`-Bedingung ist zwingend `Bool`
  - Funktion: `return`-Pfade mit deklariertem Rückgabetyp vereinbar
  - Struct/Enum-Construction/Field-/Variant-Zugriff typsicher
  - `match`-Auswertung:
    - jede Variante muss zur Enum-Definition passen
    - optional zuerst exhaustive checks (als harte Anforderung aufgenommen)
  - Referenzen:
    - Explizites Referenz-Binding/Übergeben erforderlich
    - Kein Auto-Referenz
    - Auto-Deref in Konsum-Kontexten (z. B. `f` erwartet `T`, aber bekommt `&T`, wo sinnvoll)
  - Fehlermeldungen: klarer Code + Erwartet/Gefunden + Span

### C. Werte-/Laufzeitmodell (für struct/enum + refs)
- Referenzen als eigene Runtime-Handles (i32), keine Pointer-Arithmetik.
- Einfache erste Repräsentation:
  - Skalare: `Int`/`Bool`/`Float` als direkte Werttypen
  - `&T`: indirekt via Handle
  - Struktur-/Enum-Werte als indirekte Heap-Objekte (Fat-Enum = Tag + Payload)
- Folge:
  - Codegen-Typen müssen auf gemischte Wasm-Value-Typen vorbereitet werden (`i32`, `i64`, `f64` und ggf. Heap-Handles)
  - Allocation/Deallocation erst minimalistisch über einfachen Heap/Allocator-Stub planen; später optional RC/Drop-Strategie
- `main`-Argument-Laufzeit:
  - Für die erste Version bleiben CLI-Args auf Integer-Defaults, aber Typprofile im `main`-Checker erkennt nicht-Int-Formen als Frontend-Fehler (mit klarer Nachricht). Float/Bool-CLI-Pfade können danach ergänzt werden.

### D. Compiler-Pipeline-Anpassung
- `compiler.rs`:
  - Nach Parsing: neuen Typchecker-Call einbauen
  - `CompileError` um `TypeCheck`-Variante erweitern
  - `--check` bleibt echte Frontend-Validierung (keine codegen-spezifischen Runtime-Errors als Typfehler)
- CLI/Fehlermeldungen:
  - eigener Fehlercode für Type Errors (z. B. `E-TCxx`)

### E. README-Update (nach Spez-Konsolidierung)
- Abschnitt ergänzen:
  - „Type-System-Design“
  - explizite Referenzregeln
  - auto-deref Verhalten
  - float/int/bool + structs/enums syntax
  - Pipeline-Diagramm inkl. Type Checker
  - kurze Beispiele: `&` / `*` / `match` / `enum` Varianten

## 3) Testplan
- Parser-Tests:
  - Struct- und Enum-Deklaration
  - Type-Annotationen an Funktionen
  - `&`/`*`/`match` Parsing
- Type-Check-Tests:
  - Typfehler bei `Int + Bool`, `Float` in falschem Kontext
  - `if`/`while` mit Nicht-Bool
  - Auto-deref-Erfolg/Fail (explizit vs implizit Referenz)
  - match exhaustiveness + Varianteninhalt-Typen
  - move-by-value ohne `clone`-Fallback prüfen
- Integration:
  - `cargo run -- --check` auf validen und invaliden Beispielen
  - bestehende Tests auf neue `CompileError`-Verteilung umziehen, wo nötig

## 4) Annahmen und Defaults
- Alle `let`-Bindungen sind standardmäßig mutable (wie gewünscht, Python-nah).
- Keine `&mut`-Referenzen im aktuellen MVP.
- Funktions-Parameter und Rückgabetypen sind typannotiert; lokale `let`-Typen dürfen inferiert werden.
- `Bool` ist backendseitig `i32` (`0/1`).
- `Float` ist initial nur als `f64`.
- Enum-Syntax: `enum E { A, B(T), C { x: T } }`, Match ist Teil des Kernspez.
