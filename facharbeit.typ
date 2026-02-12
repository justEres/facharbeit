

#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 5cm),
  numbering: "1 von 1",
  number-align: center
)


#set text(
  lang: "de",
  size: 12pt, 
  font: "Times New Roman"
)

// Nutzt die lokal installierte Schriftfamilie (siehe ~/.local/share/fonts)
#set par(justify: true, leading: 0.75em)

#show heading: set block(above: 0.9em, below: 0.8em)
#show heading.where(level: 1): set text(size: 15pt)
#show heading.where(level: 2): set text(size: 13pt)
#show heading.where(level: 3): set text(size: 12pt)

#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#import "vendor/codly/codly.typ": *

#show: codly-init.with()
#codly(
  number-format: none,
  zebra-fill: none,
  stroke: none,
  lang-stroke: none,
  lang-fill: none,
  display-name: false,
  display-icon: false,
  breakable: true,
  smart-indent: true,
)

#let code-box(body) = box(
  width: 100%,
  stroke: 0.5pt,
  inset: 6pt,
)[
  #set text(size: 9pt)
  #set text(hyphenate: false)
  #set par(justify: false)
  #body
]

#let diagram-box(body) = box(
  width: 100%,
  stroke: 0.5pt,
  inset: 6pt,
)[
  #body
]

#let source-file(path, lang: "text") = [
=== #path
[
  #set text(size: 9pt)
  #set text(hyphenate: false)
  #set par(justify: false)
  #raw(read(path), block: true, lang: lang)
]
]

#show figure.where(kind: "diagram"): set figure(
  supplement: [Abbildung],
  numbering: "1",
  gap: 0.4em,
)
#show figure.where(kind: "code"): set figure(
  supplement: [Codebeispiel],
  numbering: "1",
  gap: 0.4em,
)
#show figure.caption.where(kind: "diagram"): it => [
  #set text(
    size: 9pt,
    fill: rgb("#555555"),
    style: "italic",
  )
  #align(left)[#it]
]
#show figure.caption.where(kind: "code"): it => [
  #set text(
    size: 9pt,
    fill: rgb("#555555"),
    style: "italic",
  )
  #align(left)[#it]
]
#align(center)[
  #v(3cm)

  #text(size: 18pt, weight: "bold")[
    Eigene Programmiersprache? WebAssembly macht's möglich!
  ]

  #v(1cm)

  #text(size: 14pt, style: "italic")[
    Inwiefern erleichtert WebAssembly den Bau eigener Compiler für Amateurentwickler?”
  ]

  #v(3cm)

  Facharbeit im Seminarfach  
  »Zukunft des Digitalen«

  #v(2cm)

  Vorgelegt von:  
  *Erik Tschöpe*

  #v(2cm)

  Abgabedatum: 19. Februar 2026
]

#pagebreak()


#outline(title: "Inhaltsverzeichnis")

#pagebreak()

#set heading(numbering: "1.1")

= Einleitung

Unterschiedliche Programmiersprachen haben die verschiedensten Funktionen, ihre eigene Syntax oder sind speziell auf einen Anwendungsfall zugeschnitten.
So ist es ein Traum vieler Entwickler sich ihre eigene Programmiersprache zu erschaffen die perfekt an die eigenen Bedürfnisse angepasst ist.

Trotzdem bleibt die Entwicklung einer eigenen Programmiersprache für viele Entwickler ein fernes Ziel, da sie häufig mit einem hohen technischen Aufwand verbunden ist und viel Erfahrung erfordert.
Ein wesentlicher Grund dafür liegt im Bau eines sogenannten Compilers. 

Ein Compiler ist ein Programm, welches Quellcode in eine für den Computer ausführbare Form übersetzt. Der komplexeste teil ist dabei das Backend, welches für die Erzeugung von plattformspezifischem Maschinencode verantwortlich ist. Unterschiedliche Prozessorarchitekturen und Betriebssysteme erfordern jeweils eigene 
Lösungen, was den Entwicklungsaufwand stark erhöht. 
Aus diesem Grund werden Compiler in der Regel von größeren Entwicklerteams oder 
Unternehmen realisiert und nur selten von einzelnen Amateurentwicklern.

Mit der Einführung von WebAssembly (WASM) im Jahr 2017 wurde ein neuer Ansatz vorgestellt, der genau an dieser Stelle ansetzt. 
WebAssembly stellt ein standardisiertes, plattformunabhängiges Ausführungsformat dar @wasm-spec, 
das von modernen Webbrowsern und zunehmend auch außerhalb des Webs unterstützt wird. 
Statt direkt Maschinencode für eine bestimmte Architektur zu erzeugen, können Compiler 
WebAssembly als gemeinsames Ziel verwenden. 
Dadurch werden viele hardware- und betriebssystemspezifische Details abstrahiert.

Diese Entwicklung wirft die Frage auf, ob WebAssembly den Einstieg in den Compilerbau 
grundlegend erleichtert. 
Insbesondere stellt sich die Frage, ob es durch WebAssembly erstmals realistisch wird, 
dass auch Amateurentwickler eigene, funktionsfähige Compiler entwickeln können.

Aus diesem Zusammenhang ergibt sich die Leitfrage dieser Facharbeit:
*Inwiefern erleichtert WebAssembly den Bau eigener Compiler für Amateurentwickler?*

Zur Beantwortung dieser Frage werden zunächst die grundlegenden Konzepte des 
Compilerbaus sowie die Funktionsweise von WebAssembly erläutert. 
Anschließend wird in einem praktischen Selbstversuch ein einfacher Mini-Compiler 
für eine eigens definierte Programmiersprache entwickelt, der WebAssembly-Bytecode 
generiert. 
Auf Basis dieser Erfahrungen werden die Chancen und Grenzen von WebAssembly 
als Compilation Target für Hobby-Compiler bewertet.

#pagebreak()


= Compiler
- Einordnung: Grundbegriffe des Compilerbaus
- Fokus: Frontend, Backend, Zielformat WASM
- Bezug zur Leitfrage: technischer Aufwand vs. Vereinfachung durch WASM

== Was ist ein Compiler?
// - Übersetzt Quellcode in eine andere Darstellung (meist maschinennahe Form).
// - Zielformen: Maschinencode, Bytecode oder Zwischenrepräsentation (IR) zur Weiterverarbeitung.
// - Abgrenzung zu Interpreter: Interpreter führt Code direkt aus, Compiler erzeugt ausführbare Repräsentation.
// - Vorteil: schnellere Ausführung, Optimierungen vorab möglich.
// - Nachteil: zusätzlicher Übersetzungsschritt, Fehler erst beim Kompilieren sichtbar.

Einen Compiler ist ein Programm, welches Programmcode in eine andere für Computer verständliche Form übersetzt. Dabei ist es ganz egal, ob es Binärcode für eine bestimmte Prozessorarchitektur, Bytecode für eine Virtuelle Maschine oder eine Zwischenrepräsentation für die Weiterverarbeitung ist. In Abgrenzung zu einem Interpreter führt ein Compiler den Code nicht direkt aus, sondern übersetzt ihn nur und führt dabei optional Optimierungen durch. Kompilierte Programme laufen dadurch in der Regel schneller als interpretierte Programme, da die Übersetzung bereits vor der Ausführung stattfindet und Optimierungen vorgenommen werden können.

Zusätzliche Stichpunkte:
- Der Übersetzungsvorgang wird in der Praxis oft als Teil einer Toolchain betrachtet (inkl. Assembler und Linker) @gcc-overall-options.
- Für Entwickler ist wichtig: Compiler-Fehler sind Diagnoseausgaben zur Übersetzungszeit und treten vor der Programmausführung auf @gcc-overall-options.



== Aufbau eines Compilers (Frontend, Backend)
// - Eingabephase: Quellcode wird gelesen und in Tokens zerlegt (Lexing).
// - Syntaxanalyse: Parser baut einen Syntaxbaum (AST).
// - Semantische Analyse: Typprüfung, Namensauflösung, Fehlerdiagnosen.
// - Zwischenrepräsentation (IR): vereinheitlichte Form für Optimierungen.
// - Optimierungen: z.B. konstante Ausdrücke ausrechnen, ungenutzten Code entfernen.
// - Backend: IR in Zielcode übersetzen (z.B. Maschinencode oder WASM).
// - Codeerzeugung: Registerallokation, Instruktionsauswahl, Plattformdetails.
// - Ausgabe: Binärdatei, Objektdatei oder Bytecode.

Ein Compiler ist grundlegend in mehrere Teile unterteilt, die jeweils klar abgegrenzte Aufgaben übernehmen: Lexing (Erzeugung von Token aus Quelltext), Parsing (Aufbau eines abstrakten Syntaxbaums, AST), semantische Analyse (Typprüfung, Namensauflösung, Scope- und Fehlerprüfung), eine Zwischenrepräsentation und Optimierungsphase (IR‑Transformationen, konstante Auswertung, Dead‑Code‑Elimination) sowie das Backend (Code‑ bzw. Bytecode‑Generierung, z. B. für WebAssembly). Diese Modularität erleichtert Entwicklung, Testbarkeit und Wiederverwendbarkeit der einzelnen Komponenten.

Zusätzliche Stichpunkte:
- Die Trennung in Frontend und Backend erlaubt, mehrere Sprachen auf dasselbe Backend abzubilden.
- Zwischenrepräsentationen entkoppeln Sprachsyntax und Zielplattform und erleichtern Optimierungen @llvm-langref.
- In realen Toolchains werden Vorverarbeitung, Kompilierung, Assemblierung und Linking als getrennte Schritte modelliert @gcc-overall-options.

Quellen: @wiki-compiler; @radford-compiler-phases

== Einführung in WebAssembly
// - WebAssembly (WASM): binäres, plattformunabhängiges Ausführungsformat.
// - Entstanden als Ziel für Webbrowser, inzwischen auch für Server und Tools nutzbar.
// - Ziel: nahe an nativer Geschwindigkeit, aber portabel und sicher.
// - Struktur: Module mit Funktionen, Speicher, Tabellen, Imports/Exports.
// - Ausführung in einer Sandbox; Zugriff auf Systemfunktionen über Host-Imports.
// - Unterstützte Sprachen: z.B. C/C++, Rust, AssemblyScript (via Compiler-Toolchains).
// - Relevanz für Compilerbau: einheitliches Target, weniger Plattformdetails im Backend.

WebAssembly (WASM) ist ein binäres, plattformunabhängiges Ausführungsformat, das ursprünglich für die Ausführung in Webbrowsern entwickelt wurde, aber inzwischen auch außerhalb des Webs, z.B. auf Servern oder in Tools, genutzt werden kann. Es zielt darauf ab, eine nahe an nativer Geschwindigkeit liegende Ausführung zu ermöglichen, während es gleichzeitig portabel und sicher bleibt. WASM-Module bestehen aus Funktionen, Speicher, Tabellen sowie Import- und Exportdefinitionen. Die Ausführung erfolgt in einer Sandbox-Umgebung, wobei der Zugriff auf Systemfunktionen über Host-Imports erfolgt. WASM wird von vielen Sprachen unterstützt, darunter C/C++, Rust und AssemblyScript, die über Compiler-Toolchains in WASM übersetzt werden können. Für den Compilerbau bietet WASM ein einheitliches Target, wodurch viele plattformspezifische Details im Backend entfallen.

Zusätzliche Stichpunkte:
- WASM ist als Stack-Maschine definiert: Instruktionen arbeiten primär auf einem Operand-Stack @wasm-spec.
- Module werden vor der Ausführung validiert (z.B. Typkonsistenz von Instruktionen) @wasm-w3c-core.
- Textformat (WAT) und Binärformat bilden dieselbe Modulstruktur ab; WAT ist vor allem für Debugging und Lernen nützlich @wasm-spec.


Beispiel (Quellcode → WAT):

#figure(kind: "code", caption: [Programmbeispiel: Addition in eigener Sprache (Quelle: add.eres)])[
  #code-box[
```rust
fn add(a, b) -> Int {
    return a + b;
}

fn main(){
    print(add(7, 5));
    return;
}
```
]
]

#figure(kind: "code", caption: [Generiertes WAT zum Additionsbeispiel (aus `add.eres`)])[
  #code-box[
```rust
(module
  (type (;0;) (func (param i64)))  // Funktion mit 1 Parameter, kein Rückgabewert
  (type (;1;) (func (param i64 i64) (result i64)))  // Funktion mit 2 Parametern und Rückgabe
  (type (;2;) (func))  // Funktion ohne Parameter, ohne Rückgabe
  (import "env" "print_i64" (func (;0;) (type 0)))  // Import: print_i64 aus env
  (export "add" (func 1))  // Export: Funktion 1 heißt add
  (export "main" (func 2))  // Export: Funktion 2 heißt main

  (func (;1;) (type 1) (param i64 i64) (result i64)  // Funktion add(a,b) -> i64
    local.get 0  // lade Parameter a
    local.get 1  // lade Parameter b
    i64.add  // addiere a + b
    return  // Ergebnis zurückgeben
    i64.const 0  // (Default-Return, falls kein return)
    return
  )

  (func (;2;) (type 2)  // Funktion main()
    i64.const 7  // konstante 7 auf den Stack
    i64.const 5  // konstante 5 auf den Stack
    call 1  // rufe add(7,5) auf
    call 0  // rufe print_i64(result) auf
    return  // Ende
  )
)
```
]
]


#pagebreak()

= Selbstversuch
- Ziel: Umsetzbarkeit der Theorie im eigenen Mini-Compiler prüfen
- Fokus: vollständige Pipeline von Quelltext bis Ausführung
- Ergebnisartefakte: Tokens, AST, WAT, Laufzeitausgabe

== Funktionsumfang der eigenen Programmiersprache
// - Minimaler Datentyp: `Int` (Ganzzahl, intern `i64`).
// - Programmbau: nur Funktionen, keine globalen Variablen.
// - Statements: `let`, `return`, `if/else`, `while`, Ausdrucks-Statement.
// - Ausdrücke: Literale, Variablen, Binäroperationen, Funktionsaufrufe.
// - Vergleichsoperatoren: `==`, `!=`, `<`, `<=`, `>`, `>=`.
// - Rückgabetyp optional: `-> Int` oder ohne Rückgabe.
// - Print-Funktion als Host-Import (`print`).

Diese Sprache ist bewusst minimalistisch gehalten, um den Fokus auf die Kernkonzepte des Compilerbaus zu legen. Sie unterstützt nur einen Datentyp (`Int`), der intern als 64-Bit-Ganzzahl (`i64`) umgesetzt wird. Es gibt keine globalen Variablen, sondern nur Funktionen, die lokale Variablen über `let`-Statements definieren können. Kontrollstrukturen umfassen `if/else` und `while`, während Ausdrücke Literale, Variablen, Binäroperationen und Funktionsaufrufe erlauben. Vergleichsoperatoren ermöglichen einfache Bedingungen. Rückgabetypen sind optional, und eine Host-Funktion `print` ermöglicht die Ausgabe von Werten.

#figure(kind: "code", caption: [AST-Datenstrukturen (Quelle: src/ast.rs)])[
  #code-box[
```rust
#[derive(Debug)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Return(Option<Expr>),
    Expr(Expr),
    If { cond: Expr, then_block: Vec<Stmt>, else_block: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub return_type: Option<Type>,
}
```
]
]

Erläuterung:
- `Stmt` beschreibt die möglichen Anweisungen der Sprache.
- `FunctionDecl` hält Signatur und Funktionskörper zusammen.

== Lexer / Scanner
// - Aufgabe: Quelltext in Tokenliste umwandeln (inkl. Position/Span).
// - Erkannt: Schlüsselwörter (`fn`, `let`, `if`, `else`, `while`, `return`).
// - Erkannt: Literale (`Int`) und Identifier.
// - Operatoren und Trennzeichen: `+ - * / % ( ) { } , ; : ->`.
// - Fehlerbehandlung: unerwartete Zeichen, ungültige Zahlen.

Der Lexer liest den Quelltext Zeichen für Zeichen und gruppiert sie in sinnvolle Einheiten, sogenannte Tokens. Er erkennt Schlüsselwörter wie `fn`, `let`, `if`, `else`, `while` und `return`, die eine spezielle Bedeutung haben. Außerdem identifiziert er Literale (z.B. Ganzzahlen) und Identifier (z.B. Funktions- oder Variablennamen). Operatoren und Trennzeichen werden ebenfalls als eigene Token klassifiziert. Bei der Verarbeitung des Quelltexts muss der Lexer auch Fehler erkennen, z.B. wenn ein unerwartetes Zeichen auftaucht oder eine Zahl ungültig formatiert ist.

#figure(kind: "diagram", caption: [Zustandsmodell des Lexers])[
  #diagram-box[
  #set text(size: 8.5pt)
  #diagram(
  node-stroke: 0.7pt,
  spacing: 1.5em,
  node((-3.5,4), [Start], corner-radius: 2pt),
  node((-1.5,4), [Zeichen?], corner-radius: 2pt),
  edge((-3.5,4), (-1.5,4), "->"),

  node((0,0), [Buchstabe/\_], corner-radius: 2pt),
  edge((-1.5,4), (0,0), "-|>"),
  node((2,0), [In\ Ident], corner-radius: 50%),
  edge((0,0), (2,0), "->"),
  node((4,0), [Emit\ Ident/Keyword], corner-radius: 2pt),
  edge((2,0), (4,0), "->", [Ende], label-pos: 55%, label-side: left, label-sep: 2pt),

  node((0,2), [Ziffer], corner-radius: 2pt),
  edge((-1.5,4), (0,2), "-|>"),
  node((2,2), [In\ Zahl], corner-radius: 50%),
  edge((0,2), (2,2), "->"),
  node((4,2), [Emit\ Zahl], corner-radius: 50%),
  edge((2,2), (4,2), "->", [Ende], label-pos: 55%, label-side: left, label-sep: 2pt),

  node((0,4), [Op/Trenner], corner-radius: 2pt),
  edge((-1.5,4), (0,4), "->"),
  node((2,4), [Emit\ Operator], corner-radius: 2pt),
  edge((0,4), (2,4), "->"),

  node((0,6), [Whitespace], corner-radius: 2pt),
  edge((-1.5,4), (0,6), "-|>"),
  node((2,6), [Skip], corner-radius: 2pt),
  edge((0,6), (2,6), "->"),

  node((0,8), [Unbekannt], corner-radius: 2pt),
  edge((-1.5,4), (0,8), "-|>"),
  node((2,8), [Fehler], corner-radius: 2pt),
  edge((0,8), (2,8), "->"),
)
]
]

#figure(kind: "code", caption: [Token-Typen (Quelle: src/token.rs)])[
  #code-box[
```rust
pub enum TokenKind {
    // Schlüsselwörter
    Let, Fn, If, Else, While, Return,

    // Bezeichner + Literale
    Ident(String), Int(i64),

    // Operatoren
    Plus, Minus, Star, Slash, Percentage, Equal,

    // Vergleichsoperatoren
    EqualEqual, NotEqual, Less, LessEqual, Greater, GreaterEqual,

    // Trenner
    LParen, RParen, LBrace, RBrace, Semicolon, Comma,

    // Typen / Hinweise
    Colon, Arrow, IntType,

    // Dateiende
    EOF,
}
```
]
]

Erläuterung:
- Die Tokenliste ist die gemeinsame Sprache zwischen Lexer und Parser.
- `Ident(String)` und `Int(i64)` tragen bereits konkrete Werte.


#figure(kind: "code", caption: [Identifier-Lexing (Quelle: src/lexer.rs)])[
  #code-box[
```rust
pub fn lex_ident(&mut self, first_char: char) -> TokenKind {
    let mut ident_str = first_char.to_string();

    // Solange Buchstaben/Ziffern folgen, weiter einsammeln
    while let Some(c) = self.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ident_str.push(c);
            self.bump();
        } else {
            break;
        }
    }

    // Schlüsselwörter werden erkannt, sonst normaler Bezeichner
    match ident_str.as_str() {
        "let" => TokenKind::Let,
        "fn" => TokenKind::Fn,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "return" => TokenKind::Return,
        "Int" => TokenKind::IntType,
        _ => TokenKind::Ident(ident_str),
    }
}
```
]
]

Erläuterung:
- Zeichenfolge wird gesammelt und dann gegen Schlüsselwörter geprüft.
- Alles, was kein Schlüsselwort ist, wird als `Ident(...)` behandelt.

#pagebreak()

== Parser
// - Aufbau eines AST aus Tokens.
// - Einstieg: `parse_program` sammelt Funktionen bis `EOF`.
// - Funktionen: `fn name(params) -> Int { ... }`.
// - Block: Sequenz von Statements in `{ ... }`.
// - Ausdrucksparser mit Präzedenzregeln für Operatoren.

Der Parser nimmt die vom Lexer erzeugte Tokenliste und baut daraus einen abstrakten Syntaxbaum (AST) auf, der die hierarchische Struktur des Programms widerspiegelt. Der Einstiegspunkt ist die Funktion `parse_program`, die alle Funktionen im Quelltext sammelt, bis sie das End-Token (`EOF`) erreicht. Jede Funktion wird durch `fn name(params) -> Int { ... }` definiert, wobei der Rückgabetyp optional ist. Blöcke werden als Sequenzen von Statements in `{ ... }` dargestellt. Für Ausdrücke wird ein spezieller Parser mit Präzedenzregeln implementiert, um die korrekte Bindung von Operatoren sicherzustellen.

Zusätzliche Stichpunkte:
- Der gewählte Ansatz entspricht einem rekursiven Abstieg, bei dem Nichtterminale durch Funktionen umgesetzt werden @llvm-kaleidoscope-parser.
- Operator-Präzedenz wird typischerweise über eine Prioritätstabelle gesteuert, damit z.B. `*` stärker bindet als `+` @llvm-kaleidoscope-parser.
- Der AST trennt konkrete Syntax (Tokens, Klammern) von semantisch relevanter Struktur (Ausdrücke, Statements) @llvm-kaleidoscope-parser.
- [MUSS] Operator-Präzedenz im eigenen Parser konkret zeigen: `1 + 2 * 3` wird als `1 + (2 * 3)` geparst.
- [MUSS] Assoziativität festhalten: linksassoziativ für `+ - * /`, Vergleichsoperatoren nur in klaren Paaren.
- [MUSS] Fehlerstrategie erwähnen: Parser bricht bei erstem harten Fehler ab (kein Recovery/Synchronisation).
- [MUSS] Kurzer Ablauf `parse_expr`: `parse_primary` -> Binäroperationen nach Präzedenz -> AST-Knoten.
- [NICE] Mini-Beispiel AST-Form: Quelle `a + b * c` -> `Add(Var(a), Mul(Var(b), Var(c)))`.
- [NICE] Scope-Verhalten bei Blöcken benennen: Variablenzuordnung über lokale Indizes statt echter Symboltabellen-Hierarchie.
- [NICE] Optional 1 Tabelle mit Operatoren + Priorität + Assoziativität.
- [STREICHEN] sehr detaillierte Randfälle (z.B. alle nicht unterstützten Syntaxformen) nur kurz nennen statt breit ausführen.


#figure(kind: "diagram", caption: [Zustandsmodell der Statement-Parserlogik])[
  #diagram-box[
  #set text(size: 8.5pt)
  #diagram(
  node-stroke: 0.7pt,
  spacing: 1.5em,
  node((-3.5,4), [Stmt\ Start], corner-radius: 2pt),
  node((-1.5,4), [Token?], corner-radius: 2pt),
  edge((-3.5,4), (-1.5,4), "->"),

  node((0,0), [`let`], corner-radius: 2pt),
  edge((-1.5,4), (0,0), "-|>"),
  node((2,0), [parse_let], corner-radius: 2pt),
  edge((0,0), (2,0), "->"),
  node((4,0), [Ende], corner-radius: 2pt),
  edge((2,0), (4,0), "->", [;], label-pos: 55%, label-side: left, label-sep: 2pt),

  node((0,2), [`return`], corner-radius: 2pt),
  edge((-1.5,4), (0,2), "-|>"),
  node((2,2), [parse_return], corner-radius: 2pt),
  edge((0,2), (2,2), "->"),
  node((4,2), [Ende], corner-radius: 2pt),
  edge((2,2), (4,2), "->", [;], label-pos: 55%, label-side: left, label-sep: 2pt),

  node((0,4), [`if`], corner-radius: 2pt),
  edge((-1.5,4), (0,4), "->"),
  node((2,4), [parse_if], corner-radius: 2pt),
  edge((0,4), (2,4), "->"),
  node((4,4), [Ende], corner-radius: 2pt),
  edge((2,4), (4,4), "->", [`else` optional], label-pos: 55%, label-side: left, label-sep: 2pt),

  node((0,6), [`while`], corner-radius: 2pt),
  edge((-1.5,4), (0,6), "-|>"),
  node((2,6), [parse_while], corner-radius: 2pt),
  edge((0,6), (2,6), "->"),
  node((4,6), [Ende], corner-radius: 2pt),
  edge((2,6), (4,6), "->", [Block], label-pos: 55%, label-side: left, label-sep: 2pt),

  node((0,8), [sonst], corner-radius: 2pt),
  edge((-1.5,4), (0,8), "-|>"),
  node((2,8), [parse_expr_stmt], corner-radius: 2pt),
  edge((0,8), (2,8), "->"),
  node((4,8), [Ende], corner-radius: 2pt),
  edge((2,8), (4,8), "->", [;], label-pos: 55%, label-side: left, label-sep: 2pt),
)
]
]

Hinweis: `parse_expr` arbeitet rekursiv (z.B. Klammern, Binäroperatoren).

#figure(kind: "code", caption: [Parser-Einstieg (`parse_program`) (Quelle: src/parser.rs)])[
  #code-box[
```rust
pub fn parse_program(&mut self) -> Result<Program, ParseError> {
    let mut functions = Vec::new();

    while self.peek().kind != TokenKind::EOF {
        let func = self.parse_function()?;
        functions.push(func);
    }
    Ok(Program { functions })
}
```
]
]

Erläuterung:
- Der Parser sammelt alle Funktionen bis zum End-Token.
- Ergebnis ist ein `Program` als Einstiegsknoten des AST.

#figure(kind: "code", caption: [Funktionsparser Teil 1 (Quelle: src/parser.rs)])[
  #code-box[
```rust
fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
    // 1) Start mit "fn"
    self.expect(TokenKind::Fn)?;

    // 2) Funktionsname lesen
    let name = match self.bump().kind.clone() {
        TokenKind::Ident(s) => s,
        tok => {
            return Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: Token {
                    kind: tok,
                    span: self.peek().span.clone(),
                },
            });
        }
    };

    // 3) Parameterliste öffnen
    self.expect(TokenKind::LParen)?;
```
]
]

Erläuterung:
- Erwartet `fn`, danach den Namen und die Parameterliste.
- Rückgabetyp ist optional und wird nur bei `-> Int` gesetzt.
- Der Funktionskörper ist ein Block mit Statements.

// Optional streichen (reiner Layout-Text):
// Fortsetzung: Parameterliste

#figure(kind: "code", caption: [Funktionsparser Teil 2 (Quelle: src/parser.rs)])[
  #code-box[
```rust
    let mut params = Vec::new();
    if self.peek().kind != TokenKind::RParen {
        // Parameter: name, name, name
        loop {
            match self.bump().kind.clone() {
                TokenKind::Ident(s) => params.push(s),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "parameter name".to_string(),
                        found: self.peek().clone(),
                    });
                }
            }

            if self.peek().kind == TokenKind::Comma {
                self.bump();
            } else {
                break;
            }
        }
    }

    self.expect(TokenKind::RParen)?;
```
]
]

// Optional streichen (reiner Layout-Text):
// Fortsetzung: Rückgabetyp und Funktionskörper

#figure(kind: "code", caption: [Funktionsparser Teil 3 (Quelle: src/parser.rs)])[
  #code-box[
```rust
    // 4) Optionaler Rückgabetyp
    let return_type = if self.peek().kind == TokenKind::Arrow {
        self.bump();
        self.expect(TokenKind::IntType)?;
        Some(crate::ast::Type::Int)
    } else {
        None
    };

    // 5) Funktionskörper (Block)
    let body = self.parse_block()?;

    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}
```
]
]

#pagebreak()

== Codegen
// - Ziel: WASM-Bytecode erzeugen.
// - Eigene kleine IR (`IrInstruction`) als Zwischenschicht.
// - Mapping von IR auf `wasm_encoder::Instruction`.
// - ModuleGen: sammelt Typen, Imports, Funktionen, Exports, Code.
// - Host-Funktion `print` als Import (`env.print_i64`).

Der Codegenerator nimmt den AST und übersetzt ihn in eine eigene Zwischenrepräsentation (IR), die aus einer linearen Folge von Instruktionen besteht. Diese IR abstrahiert von den Details der WASM-Generierung und ermöglicht eine klarere Trennung zwischen der Logik der Codeerzeugung und den spezifischen Anforderungen des WASM-Formats. Anschließend wird die IR in echte WASM-Bytecode-Instruktionen umgewandelt, wobei die `wasm_encoder`-Bibliothek verwendet wird, um Module, Funktionen, Imports und Exports zu definieren. Die Host-Funktion `print` wird als Import unter dem Namen `env.print_i64` bereitgestellt, damit sie im generierten WASM-Modul aufgerufen werden kann.

Zusätzliche Stichpunkte:
- Eine IR erleichtert spätere Optimierungen, weil Transformationen nicht mehr direkt auf Quellsyntax arbeiten.
- Der Einsatz einer IR entspricht der Praxis großer Compiler-Infrastrukturen (z.B. LLVM IR als zentrale Zwischenschicht) @llvm-langref.
- Die Abbildung von Kontrollfluss (`if`, `while`) auf explizite Instruktionsfolgen ist ein zentraler Schritt vom AST zum Zielcode.
- [MUSS] Mapping `if/else` in WASM herausstellen: Bedingung auf Stack -> `if` -> optional `else` -> `end`.
- [MUSS] Mapping `while` in WASM herausstellen: `block` + `loop`, invertierte Bedingung mit `br_if`, Rücksprung mit `br 0`.
- [MUSS] Return-Verhalten dokumentieren: explizites `return` im AST + Fallback-Return im generierten Code.
- [NICE] Kurze Tabelle `AST-Konstrukt -> IR -> WASM` (je 1 Mini-Beispiel für Addition, Vergleich, `if/else`, `while`).
- [NICE] Offener Punkt: Dead Code nach `return` wird noch emittiert (keine DCE/CFG-Bereinigung).
- [STREICHEN] zu viele WASM-Details ohne direkten Bezug zu deinem eigenen Codepfad.

#figure(kind: "code", caption: [IR-Instruktionen (Quelle: src/codegen/ir.rs)])[
  #code-box[
```rust
pub enum IrInstruction{
    I64Const(i64),
    I64Eqz,
    BrIf(u32),
    Br(u32),
    LocalSet(u32),
    LocalGet(u32),
    Call(u32),
    If(BlockType),
    Else,
    Block(BlockType),
    Loop(BlockType),
    Drop,
    Return,
    End,

    // Arithmetik
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64Eq,
    I64LtS,
    I64GtS,
    I64ExtendI32S,
    I32Eqz,
}
```
]
]

Erläuterung:
- `I64Const` entspricht dem Laden einer Konstante im WASM-Stack.
- `LocalGet/LocalSet` stehen für Variablenzugriffe.
- `I64Add` und `I64Eq` sind direkte WASM-Arithmetik/Vergleiche.
- Kontrollfluss wird über `If/Else/Block/Loop/End` abgebildet.

#figure(kind: "code", caption: [Funktions-Emission (Quelle: src/codegen/module.rs)])[
  #code-box[
```rust
pub fn emit_function(&mut self, func: &FunctionDecl) {
    let mut r#gen = FuncGen {
        locals: Vec::new(),
        local_map: HashMap::new(),
        instructions: Vec::new(),
        has_return: func.return_type.is_some(),
    };

    // Parameter werden auf lokale Indizes gemappt
    for (i, name) in func.params.iter().enumerate() {
        r#gen.local_map.insert(name.clone(), i as u32);
    }

    // Statements -> IR-Instruktionen
    for stmt in &func.body {
        emit_stmt(stmt, &mut r#gen, &self.func_indices);
    }
}
```
]
]

Erläuterung:
- Parameter werden zu lokalen Indizes abgebildet.
- Statements erzeugen eine lineare Folge von IR-Instruktionen.
- Danach folgt die Umwandlung der IR-Instruktionen in echtes WASM.

#pagebreak()

== Ausführung und Beispiel
- WASM wird lokal ausgeführt, nicht im Browser.
- Runtime: `wasmtime` (lädt Bytecode, instanziert Modul, ruft `main` auf).
- Host-Import `print_i64` wird in Rust bereitgestellt, damit `print(...)` funktioniert.
- Ablauf: Quelltext → Tokens → AST → WASM‑Bytes → Wasmtime ausführen.
- `Engine`, `Module`, `Store` und `Instance` sind die zentralen Bausteine der Wasmtime-Ausführung @wasmtime-crate-docs.
- Imports müssen beim Instanziieren bereitgestellt werden, sonst schlägt das Laden des Moduls fehl @wasmtime-hello-world.
- [MUSS] Konvention des Projekts explizit machen: Einstieg über exportierte `main`.
- [MUSS] Validierung erwähnen: ungültiges WASM fällt beim Laden/Instanziieren auf.
- [NICE] Grenzen der Laufzeit klar benennen (keine Dateisystem-/Netzwerkzugriffe ohne Imports; nur freigegebene Host-Funktionen).
- [NICE] Messpunkt ergänzen: Compile-Zeit vs. Laufzeitzeit für 1-2 Testprogramme.

#figure(kind: "code", caption: [WASM-Ausführung mit Wasmtime (Quelle: src/runner.rs)])[
  #code-box[
```rust
pub fn run_wasm_bytes(bytes: &[u8], args: Vec<i64>) -> Result<Option<i64>, String> {
    let engine = Engine::default();
    let module = wasmtime::Module::from_binary(&engine, bytes)
        .map_err(|e| format!("module compile error: {}", e))?;

    let mut store = Store::new(&engine, ());

    // Host-Funktion für print(...)
    let print_func = wasmtime::Func::wrap(&mut store, |v: i64| {
        println!("{}", v);
    });

    let instance = Instance::new(&mut store, &module, &[print_func.into()])
        .map_err(|e| format!("instance error: {}", e))?;

    let func = instance
        .get_func(&mut store, "main")
        .ok_or_else(|| "function `main` not found".to_string())?;
    // ...
    func.call(&mut store, &params, &mut results_buf)
        .map_err(|e| format!("runtime error: {}", e))?;
    // ...
}
```
]
]

Beispiel (Fakultät):

#figure(kind: "code", caption: [Programmbeispiel: Fakultät (Quelle: factorial.eres)])[
  #code-box[
```rust
fn fact(n) -> Int {
    if (n <= 1) {
        return 1;
    } else {
        return n * fact(n - 1);
    }
}

fn main(){
    print(fact(10));
    return;
}
```
]
]

== Eigener Beitrag und verwendete Tools
- Eigenleistung: Sprachsyntax festgelegt (`fn`, `let`, `if/else`, `while`, `return`)
- Eigenleistung: Lexer, Parser, Codegen, Runtime-Anbindung umgesetzt
- Eigenleistung: Testfälle für Lexer, Parser und Ausführung ergänzt
- Verwendete Tools: Rust
- Verwendete Tools: `wasm-encoder`
- Verwendete Tools: `wasmtime`
- Verwendete Tools: Typst
- [NICE] Unterstützung: KI für Strukturierung/Brainstorming
// Optional streichen, falls Fokus nur auf technische Eigenleistung:
// - Unterstützung: KI für Strukturierung/Brainstorming

== Testwerkzeuge und CLI-Nutzung
- Token-Ausgabe: `cargo run -- <datei> --print-tokens`
- AST-Ausgabe: `cargo run -- <datei> --print-ast`
- WAT-Ausgabe: `cargo run -- <datei> --print-wat`
- Artefakt: Token-Stream (Lexer)
- Artefakt: AST (Parser)
- Artefakt: WAT (Codegen)
- [MUSS] Fehlerfall demonstrieren (z.B. unerwartetes Token) inkl. Position/Context der Fehlermeldung.
- [MUSS] Testmatrix (kurz, tabellarisch): arithmetische Ausdrücke, Vergleich + `if/else`, Schleife, Rekursion (`fact`), Grenzfall (leere Parameterliste/fehlendes Semikolon).
- [NICE] Gegenüberstellung "erwartete Ausgabe vs. tatsächliche Ausgabe" für 2-3 Programme.
- [STREICHEN] zu viele CLI-Varianten ohne Erkenntnisgewinn (auf Kernbefehle begrenzen).


#pagebreak()

= Fazit
- Zusammenfassung der zentralen Ergebnisse
- Persönliche Stellungnahme zur Leitfrage
- Selbstreflexion: Herausforderungen und Verbesserungen

// - Einheitliches Ziel (WASM) statt viele Plattformen
// - Backend-Aufwand reduziert
// - Frontend bleibt komplex
// - Abhängigkeit von Host-Imports/Sandbox
// - Eignung für Hobby-Compiler: deutlich besser, aber nicht trivial

// - Selbstversuch
//     - mehr proof of concept als geeignet für produktion
//     - sehr gut zum lernen  

// Optional streichen (bereits als Stichpunkte darunter enthalten):
// Um die Leitfrage zu beantworten: WebAssembly erleichtert den Bau eigener Compiler für Amateurentwickler erheblich, da es ein einheitliches Ziel bietet und viele plattformspezifische Details im Backend abstrahiert. Allerdings bleibt die Entwicklung eines Compilers eine komplexe Aufgabe, insbesondere im Frontend (Lexing, Parsing, semantische Analyse). Die Abhängigkeit von Host-Imports und der Sandbox-Umgebung von WASM kann ebenfalls Einschränkungen mit sich bringen. Insgesamt ist WASM eine vielversprechende Plattform für Hobby-Compiler, aber es erfordert dennoch ein gewisses Maß an technischem Verständnis und Aufwand.

- Zusammenfassung: WASM reduziert Backend-Komplexität
- Persönliche Stellungnahme: Ziel der Arbeit erreicht / Leitfrage beantwortet
- Selbstreflexion: größte Herausforderungen (Parserlogik, Codegen, Tooling)
- Selbstreflexion: nächste Iteration (Fehlerdiagnostik, Typen, Sprachumfang)
- [MUSS] Leitfrage final gewichten: WASM senkt Einstiegshürde im Backend, Gesamtkomplexität bleibt mittel-hoch wegen Frontend + Semantik.
- [MUSS] Konkrete Antwortformel für den Schlusssatz: "erleichtert deutlich, ersetzt aber kein Compiler-Grundwissen".
- [NICE] Ausblick (nächste Iteration): Typchecker, bessere Fehlermeldungen (Zeile/Spalte + Recovery), kleine Optimierungen (constant folding, dead code).
- [STREICHEN] Wiederholung der Einleitungsaussagen ohne neue Bewertung.

#pagebreak()


= Quellen
#bibliography("bibliography.yaml", title: none)


= Anhang: Quellcode

Anhang-Inhaltsverzeichnis:

#outline(
  title: "Quellcode-Übersicht",
  target: heading.where(level: 3),
)

#pagebreak()

#source-file("factorial.eres", lang: "rust")
#source-file("src/ast.rs", lang: "rust")
#source-file("src/lexer.rs", lang: "rust")
#source-file("src/main.rs", lang: "rust")
#source-file("src/parser.rs", lang: "rust")
#source-file("src/runner.rs", lang: "rust")
#source-file("src/token.rs", lang: "rust")
#source-file("src/codegen/expr.rs", lang: "rust")
#source-file("src/codegen/ir.rs", lang: "rust")
#source-file("src/codegen/mod.rs", lang: "rust")
#source-file("src/codegen/module.rs", lang: "rust")
#source-file("src/codegen/stmt.rs", lang: "rust")

#pagebreak()
