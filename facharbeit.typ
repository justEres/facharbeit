#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 5cm),
  numbering: "1",
  number-align: right + bottom
)

#set text(
  lang: "de",
  size: 12pt,
  font: "Times New Roman"
)

// Nutzt die lokal installierte Schriftfamilie (siehe ~/.local/share/fonts)
#set par(justify: true, leading: 0.95em)

#show heading: set block(above: 0.45cm, below: 0.45cm)
#show heading.where(level: 1): set text(size: 18pt)
#show heading.where(level: 2): set text(size: 15pt)
#show heading.where(level: 3): set text(size: 12pt)

#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#import "vendor/codly/codly.typ": *

#let render-code-blocks = true // auf false setzen, um Codeblöcke auszublenden

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

#let source-file(path, lang: "text") = if render-code-blocks [
  === #path
  #block[
    #set text(size: 9pt)
    #set text(hyphenate: false)
    #set par(justify: false)
    #raw(read(path), block: true, lang: lang)
  ]
] else []

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
#show figure.where(kind: "code"): it => {
  if render-code-blocks { it } else { [] }
}
#show raw.where(block: false): it => box(

  fill: rgb("#f0f0f0"),
  radius: 2pt,
  outset: (x: 3pt, y: 2pt),
)[
  #it
]
#set page(margin: 0pt, numbering: none)
#image("formblaetter.pdf", page: 1, width: 100%, height: 100%)
#pagebreak()
#image("formblaetter.pdf", page: 2, width: 100%, height: 100%)
#pagebreak()
#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 5cm),
  numbering: "1",
  number-align: right + bottom
)

#set page(numbering: none)
#outline(title: "Inhaltsverzeichnis")

#pagebreak()

#set page(
  numbering: "1",
  number-align: right + bottom,
)

#set heading(numbering: "1.1")

= Einleitung

Programmiersprachen variieren in ihrer Syntax, ihrem Funktionsumfang und ihren Einsatzbereichen.
So ist es ein Traum vieler Entwickler, sich eine eigene Programmiersprache zu erschaffen, die perfekt an die eigenen Bedürfnisse angepasst ist.

Trotzdem bleibt die Entwicklung einer eigenen Programmiersprache für viele Entwickler ein fernes Ziel, da sie häufig mit einem hohen technischen Aufwand verbunden ist und viel Erfahrung erfordert.
Ein wesentlicher Grund dafür liegt im Bau eines sogenannten Compilers.

Ein Compiler ist ein Programm, das Quellcode in eine für den Computer ausführbare Form übersetzt. Der komplexeste Teil ist dabei das Backend, das für die Erzeugung von plattformspezifischem Maschinencode verantwortlich ist. Unterschiedliche Prozessorarchitekturen und Betriebssysteme erfordern jeweils eigene
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
als Zielplattform für Hobby-Compiler bewertet.

Im weiteren Verlauf wird untersucht, inwiefern WebAssembly den Einstieg in den Compilerbau erleichtert. Im Fokus stehen dabei die plattformunabhängige Ausführung und die Abstraktion von Hardwaredetails, durch die sich die Implementierung stärker auf Sprach- und Compilerlogik konzentrieren kann.

= Vorwissen: Compiler und WebAssembly
// - Einordnung: Grundbegriffe des Compilerbaus
// - Fokus: Frontend, Backend, Zielformat WASM
// - Bezug zur Leitfrage: technischer Aufwand vs. Vereinfachung durch WASM
Dieses Kapitel führt in die Grundkonzepte des Compilerbaus ein und erläutert die Architektur eines Compilers. Der Fokus liegt auf dem Verständnis, warum WebAssembly als Zielplattform eine vielversprechende Vereinfachung für Hobby-Compiler darstellt.

== Was ist ein Compiler?
// - Übersetzt Quellcode in eine andere Darstellung (meist maschinennahe Form).
// - Zielformen: Maschinencode, Bytecode oder Zwischenrepräsentation (IR) zur Weiterverarbeitung.
// - Abgrenzung zu Interpreter: Interpreter führt Code direkt aus, Compiler erzeugt ausführbare Repräsentation.
// - Vorteil: schnellere Ausführung, Optimierungen vorab möglich.
// - Nachteil: zusätzlicher Übersetzungsschritt, Fehler erst beim Kompilieren sichtbar.

Ein Compiler ist ein Programm, das Programmcode in eine andere, für Computer verständliche Form übersetzt. Dabei kann das Ziel Binärcode für eine bestimmte Prozessorarchitektur, Bytecode für eine virtuelle Maschine oder eine Zwischenrepräsentation für die Weiterverarbeitung sein. In Abgrenzung zu einem Interpreter führt ein Compiler den Code nicht direkt aus, sondern übersetzt ihn und führt dabei optional Optimierungen durch. Kompilierte Programme laufen dadurch in der Regel schneller als interpretierte Programme, da die Übersetzung bereits vor der Ausführung stattfindet und Optimierungen vorgenommen werden können. Zusätzlich erleichtern moderne Compiler den Entwicklern das Leben, indem sie häufige Fehler schon beim Übersetzen des Quellcodes finden und verständliche Fehlermeldungen ausgeben, während Interpreter Fehler erst zur Laufzeit sichtbar machen, was die Fehlersuche erschwert @ibm-compiler. Wenn der Begriff "Compiler" fällt, ist selten nur der reine Übersetzungsvorgang gemeint, sondern oft die gesamte Toolchain, die auch Assembler und Linker umfasst, um aus Quellcode eine ausführbare Datei zu erzeugen @gcc-overall-options.

== Aufbau eines Compilers (Frontend, Backend)
// - Eingabephase: Quellcode wird gelesen und in Tokens zerlegt (Lexing).
// - Syntaxanalyse: Parser baut einen Syntaxbaum (AST).
// - Semantische Analyse: Typprüfung, Namensauflösung, Fehlerdiagnosen.
// - Zwischenrepräsentation (IR): vereinheitlichte Form für Optimierungen.
// - Optimierungen: z.B. konstante Ausdrücke ausrechnen, ungenutzten Code entfernen.
// - Backend: IR in Zielcode übersetzen (z.B. Maschinencode oder WASM).
// - Codeerzeugung: Registerallokation, Instruktionsauswahl, Plattformdetails.
// - Ausgabe: Binärdatei, Objektdatei oder Bytecode.

Ein Compiler ist grundlegend in mehrere Teile unterteilt, die jeweils klar abgegrenzte Aufgaben übernehmen: Lexing (Erzeugung von Token aus Quelltext), Parsing (Aufbau eines abstrakten Syntaxbaums, AST), semantische Analyse (Typprüfung, Namensauflösung, Scope- und Fehlerprüfung), eine Zwischenrepräsentation und Optimierungsphase (IR-Transformationen, konstante Auswertung, Dead-Code-Elimination) sowie das Backend (Code- bzw. Bytecode-Generierung, z.B. für WebAssembly). Diese Modularität erleichtert Entwicklung, Testbarkeit und Wiederverwendbarkeit der einzelnen Komponenten. Zusätzlich bietet diese Struktur die Möglichkeit, verschiedene Frontends (für unterschiedliche Sprachen) mit demselben Backend zu kombinieren, was die Flexibilität erhöht. Moderne Compiler sind genau entlang solcher Schritte aufgebaut @rustc-overview.

== Einführung in WebAssembly
// - WebAssembly (WASM): binäres, plattformunabhängiges Ausführungsformat.
// - Entstanden als Ziel für Webbrowser, inzwischen auch für Server und Tools nutzbar.
// - Ziel: nahe an nativer Geschwindigkeit, aber portabel und sicher.
// - Struktur: Module mit Funktionen, Speicher, Tabellen, Imports/Exports.
// - Ausführung in einer Sandbox; Zugriff auf Systemfunktionen über Host-Imports.
// - Unterstützte Sprachen: z.B. C/C++, Rust, AssemblyScript (via Compiler-Toolchains).
// - Relevanz für Compilerbau: einheitliches Target, weniger Plattformdetails im Backend.

WebAssembly (WASM) ist ein binäres, plattformunabhängiges Ausführungsformat, das ursprünglich für die Ausführung in Webbrowsern entwickelt wurde, aber inzwischen auch außerhalb des Webs, z.B. auf Servern oder in Tools, genutzt werden kann. Es zielt darauf ab, eine nahe an nativer Geschwindigkeit liegende Ausführung zu ermöglichen, während es gleichzeitig portabel und sicher bleibt. WASM-Module bestehen aus Funktionen, Speicher, Tabellen sowie Import- und Exportdefinitionen, und die Ausführung erfolgt in einer Sandbox-Umgebung mit Zugriff auf Systemfunktionen über Host-Imports. WASM wird von vielen Sprachen unterstützt, darunter C/C++, Rust und AssemblyScript, die über Compiler-Toolchains in WASM übersetzt werden können @mdn-wasm-concepts.

Für den Compilerbau bietet WASM eine einheitliche Zielplattform, wodurch viele plattformspezifische Details im Backend entfallen. Dadurch ist WASM besonders attraktiv für Hobby-Compiler, weil sich die Komplexität der Codegenerierung reduziert und der Fokus stärker auf Sprachlogik und Semantik liegt.

Das Grundprinzip, nach dem WASM arbeitet, ist die Stack-Maschine, bei der Instruktionen primär auf einem Operand-Stack operieren. Zum Beispiel nimmt die `add`-Instruktion die obersten zwei Werte vom Stack, addiert sie und legt das Ergebnis wieder auf den Stack. Dies ermöglicht eine einfache und effiziente Ausführung von Anweisungen, da keine expliziten Register oder Speicheradressen benötigt werden @mdn-wasm-text-format; @wasm-spec.

WebAssembly wird in Modulen verpackt, die Funktionen, Speicher, Tabellen sowie Import- und Exportdefinitionen enthalten. Ein valides WASM-Modul muss bestimmte Regeln erfüllen, damit es von der Laufzeitumgebung akzeptiert wird. Dazu gehören ein klarer Aufbau des Moduls, die Konsistenz von Funktionssignaturen, die Korrektheit des Kontrollflusses und die Gültigkeit referenzierter Indizes. Der Validator prüft diese Regeln vor der Ausführung, und bei Verstoß wird das Modul nicht instanziiert @wasm-w3c-core; @wasm-spec.

Hier ist ein Beispiel für Quellcode in unserer eigenen Sprache, der eine einfache Addition durchführt, sowie das daraus generierte WebAssembly-Textformat (WAT). Das Beispiel zeigt, wie eine Funktion `add` definiert wird, die zwei Ganzzahlen addiert, und eine `main`-Funktion, die diese Addition ausführt und das Ergebnis über eine Host-Import-Funktion `print` ausgibt.

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

Aus diesem Beispielprogramm wird folgendes WAT generiert, das die gleiche Logik in WebAssembly-Textformat darstellt. Es definiert die Funktion `add`, die zwei `i64`-Parameter entgegennimmt und deren Summe zurückgibt, sowie die `main`-Funktion, die `add(7, 5)` aufruft und das Ergebnis mit der importierten Funktion `print_i64` ausgibt.

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

= Selbstversuch
// - Ziel: Umsetzbarkeit der Theorie im eigenen Mini-Compiler prüfen
// - Fokus: vollständige Pipeline von Quelltext bis Ausführung
// - Ergebnisartefakte: Tokens, AST, WAT, Laufzeitausgabe

Die Konzepte des Compilerbaus und die Funktionsweise von WebAssembly wurden nun theoretisch erläutert. Im folgenden Abschnitt wird die praktische Umsetzbarkeit dieser Konzepte mit einem eigenen Mini-Compiler überprüft, der eine eigens definierte, minimalistische Programmiersprache in WebAssembly-Bytecode übersetzt. Dabei wird die gesamte Pipeline von der Quelltexteingabe über die Tokenisierung, das Parsing und die semantische Analyse bis zur Codegenerierung und Ausführung durchlaufen. Ziel ist es, die technischen Schritte anhand konkreter Artefakte wie Token-Stream, AST, generiertem WAT und Laufzeitausgabe nachvollziehbar zu machen.

== Methodik des Selbstversuchs

Methodisch handelt es sich um eine prototypische Implementierung mit bewusst begrenztem Sprachumfang. Geprüft wird jede Stufe der Übersetzung mit passenden Zwischenergebnissen (Token-Stream, AST und WAT) sowie anschließend die End-to-End-Ausführung über Wasmtime. Die Auswertung erfolgt durch den Vergleich von erwartetem und tatsächlichem Verhalten anhand eigener Testprogramme; der Fokus liegt dabei auf Nachvollziehbarkeit der Pipeline, nicht auf Produktionsreife oder umfassender Optimierung. Um die Reproduzierbarkeit zu sichern, wurden die Testläufe mit festen Eingabedateien und konsistenten CLI-Optionen durchgeführt und die Zwischenausgaben jeweils dokumentiert.

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
pub enum Stmt { // Mögliche Anweisungen in der Sprache
    Let { name: String, value: Expr },
    Return(Option<Expr>),
    Expr(Expr),
    If { cond: Expr, then_block: Vec<Stmt>, else_block: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
}

#[derive(Debug)]
pub struct FunctionDecl { // Hält Signatur und Funktionskörper zusammen
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub return_type: Option<Type>,
}
```
]
]

// Erläuterung:
// - `Stmt` beschreibt die möglichen Anweisungen der Sprache.
// - `FunctionDecl` hält Signatur und Funktionskörper zusammen.

== Lexer / Scanner
// - Aufgabe: Quelltext in Tokenliste umwandeln (inkl. Position/Span).
// - Erkannt: Schlüsselwörter (`fn`, `let`, `if`, `else`, `while`, `return`).
// - Erkannt: Literale (`Int`) und Identifier.
// - Operatoren und Trennzeichen: `+ - * / % ( ) { } , ; : ->`.
// - Fehlerbehandlung: unerwartete Zeichen, ungültige Zahlen.

Der Lexer liest den Quelltext Zeichen für Zeichen und gruppiert sie in sinnvolle Einheiten, sogenannte Tokens. Er erkennt Schlüsselwörter wie `fn`, `let`, `if`, `else`, `while` und `return`, die eine spezielle Bedeutung haben. Außerdem identifiziert er Literale (z.B. Ganzzahlen) und Identifier (z.B. Funktions- oder Variablennamen). Operatoren und Trennzeichen werden ebenfalls als eigene Token klassifiziert. Bei der Verarbeitung des Quelltexts muss der Lexer auch Fehler erkennen, z.B. wenn ein unerwartetes Zeichen auftaucht oder eine Zahl ungültig formatiert ist @crafting-scanning; @rustc-parser.
Die verschiedenen Token-Typen werden in einem Enum `TokenKind` modelliert, das die verschiedenen Kategorien von Tokens abdeckt, einschließlich Schlüsselwörtern, Literalen, Operatoren und Fehlern.

#figure(kind: "code", caption: [Token-Typen (Quelle: src/token.rs)])[
  #code-box[
```rust
pub enum TokenKind {
    // Schlüsselwörter
    Let, Return, If, While,
    // Inhalte
    Ident(String), Int(i64),
    // Operatoren / Trenner
    Plus, Minus, Star, Slash, Equal,
    LParen, RParen, LBrace, RBrace, Semicolon,
    // Ende / Fehler
    EOF,
    Error,
}
```
]
]

Beim Hauptlauf des Lexers wird der Quelltext zeichenweise durchlaufen. Zunächst werden alle Whitespace-Zeichen übersprungen, da sie für die Syntax keine Bedeutung haben. Sobald ein nicht-Whitespace-Zeichen gefunden wird, entscheidet der Lexer, ob es sich um den Beginn eines Identifiers, einer Zahl oder eines Operators handelt. Ein Identifier oder Schlüsselwort beginnt mit einem Buchstaben oder Unterstrich, gefolgt von alphanumerischen Zeichen oder Unterstrichen. Eine Zahl besteht ausschließlich aus Ziffern. Operatoren und Trennzeichen werden direkt erkannt. Sobald ein Token vollständig erkannt ist (z.B. wenn kein weiteres Zeichen mehr zum aktuellen Token passt), wird es ausgegeben.

#figure(kind: "code", caption: [Vereinfachter Lexer-Hauptlauf (Quelle: src/lexer.rs)])[
  #code-box[
```rust
fn next_token(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> TokenKind {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next(); // Whitespace überspringen
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            return lex_ident(chars); // Identifier oder Schlüsselwort
        }
        if c.is_ascii_digit() {
            return lex_number(chars); // Zahl
        }

        chars.next();
        return match c {
            '+' => TokenKind::Plus, // Operatoren und Trennzeichen
            '-' => TokenKind::Minus,
            ';' => TokenKind::Semicolon,
            _ => TokenKind::Error,
        };
    }

    TokenKind::EOF // Ende des Quelltexts
}
```
]
]

Bei der Erkennung von Identifiers wird erst am Ende entschieden, ob es sich um ein Schlüsselwort handelt, indem die gesammelte Zeichenfolge mit bekannten Schlüsselwörtern verglichen wird. Wenn ein unbekanntes Zeichen auftaucht, erzeugt der Lexer sofort einen Fehler-Token, ohne weiter zu parsen.

#figure(kind: "code", caption: [Identifier-Lexing (Quelle: src/lexer.rs)])[
  #code-box[
```rust
fn keyword_or_ident(text: String) -> TokenKind {
    match text.as_str() {
        "let" => TokenKind::Let, // Deklaration von Variablen
        "return" => TokenKind::Return, // Rückgabe von Funktionen
        "if" => TokenKind::If, // Bedingte Anweisungen
        "while" => TokenKind::While, // Schleifen
        _ => TokenKind::Ident(text), // Identifier
    }
}

```
]
]

== Parser
// - Aufbau eines AST aus Tokens.
// - Einstieg: `parse_program` sammelt Funktionen bis `EOF`.
// - Funktionen: `fn name(params) -> Int { ... }`.
// - Block: Sequenz von Statements in `{ ... }`.
// - Ausdrucksparser mit Präzedenzregeln für Operatoren.
//

Der Parser nimmt die vom Lexer erzeugte Tokenliste und baut daraus einen abstrakten Syntaxbaum (AST, Abstract Syntax Tree) auf, der die hierarchische Struktur des Programms widerspiegelt. Die in diesem Selbstversuch verwendete Strategie nennt sich rekursiver Abstieg, wobei durch Rekursion die hierarchische Natur der Sprache direkt in der Parserlogik abgebildet wird (@crafting-parsing-expr).

Unterschieden wird zwischen Statements, die vollständige Anweisungen darstellen (z.B. `let x = 5;` oder `return x;`), und Expressions, die einen Wert liefern (z.B. `x + 2` oder `add(7, 5)`). In einem Statement wie `let y = x + 2;` ist das gesamte Konstrukt ein Statement, während `x + 2` die Expression ist, die den Wert für die Variable `y` liefert. Ebenso ist in `print(add(7, 5));` das gesamte Konstrukt ein Statement, während `add(7, 5)` die Expression ist, deren Ergebnis an die `print`-Funktion übergeben wird.

=== Rekursiver Abstieg

So können beispielsweise Blöcke, die aus einer Sequenz von Statements bestehen, einfach durch eine Funktion `parse_block` umgesetzt werden, die so lange Statements parst, bis sie das schließende `}` findet.
Der Einstiegspunkt ist die Funktion `parse_program`, die alle Funktionen im Quelltext sammelt, bis sie das End-Token (`EOF`) erreicht. Jede Funktion wird durch `fn name(params) -> Int { ... }` definiert, wobei der Rückgabetyp optional ist. Blöcke werden als Sequenzen von Statements in `{ ... }` dargestellt.

#figure(kind: "code", caption: [Parser-Einstieg (`parse_program`) (vereinfacht, Quelle: src/parser.rs)])[
  #code-box[
```rust
fn parse_program(&mut self) -> Result<Program, ParseError> {
    let mut functions = Vec::new(); // Funktionsliste.

    while self.peek().kind != TokenKind::EOF { // Bis Dateiende.
        let func = self.parse_function()?; // Eine Funktion parsen.
        functions.push(func); // Hinzufügen.
    }

    Ok(Program { functions }) // AST zurück.
}
```
]
]

#figure(kind: "code", caption: [Block-Parser (`parse_block`) (vereinfacht, Quelle: src/parser.rs)])[
  #code-box[
```rust
fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
    self.expect(TokenKind::LBrace)?; // `{` erwartet.
    let mut statements = Vec::new(); // Statements sammeln.

    while self.peek().kind != TokenKind::RBrace { // Bis `}`.
        let stmt = self.parse_stmt()?; // Statement parsen.
        statements.push(stmt); // Speichern.
    }

    self.expect(TokenKind::RBrace)?; // `}` erwartet.
    Ok(statements) // Block zurück.
}
```
]
]

=== Operator-Präzedenz

Um die korrekte Bindung von Operatoren zu gewährleisten, wird ein Präzedenzsystem implementiert. Dabei wird jedem Operator eine Präzedenzstufe zugeordnet, die bestimmt, in welcher Reihenfolge die Operatoren ausgewertet werden. Zum Beispiel bindet `*` stärker als `+`, sodass `1 + 2 * 3` als `1 + (2 * 3)` interpretiert wird. Dies wird durch eine Prioritätstabelle gesteuert, die die Präzedenz der Operatoren definiert. @crafting-parsing-expr

#figure(kind: "code", caption: [Operator-Präzedenz (vereinfacht, Quelle: src/parser.rs)])[
  #code-box[
```rust
fn precedence(kind: &TokenKind) -> Option<u8> {
    match kind {
        TokenKind::Star | TokenKind::Slash => Some(20), // Hoch.
        TokenKind::Plus | TokenKind::Minus => Some(10), // Niedrig.
        _ => None, // Kein Operator.
    }
}
```
]
]

Wenn der Parser ein erwartetes Token nicht findet, bricht er sofort mit einem Fehler (`Err`) ab und meldet, was erwartet wurde und was tatsächlich gefunden wurde.

#figure(kind: "code", caption: [Funktionsparser (Quelle: src/parser.rs)])[
  #code-box[
```rust
fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
    self.expect(TokenKind::Fn)?; // `fn`.
    let name = self.expect_ident()?; // Name.

    self.expect(TokenKind::LParen)?; // `(`.
    let params = self.parse_params()?; // Parameter.
    self.expect(TokenKind::RParen)?; // `)`.

    let return_type = if self.peek().kind == TokenKind::Arrow {
        self.bump(); // `->`.
        self.expect(TokenKind::IntType)?; // Typ.
        Some(Type::Int) // Mit Rückgabetyp.
    } else {
        None // Ohne Rückgabetyp.
    };

    let body = self.parse_block()?; // Funktionsblock.

    Ok(FunctionDecl {
        name, // Name
        params, // Parameter
        body, // Statements
        return_type, // Rückgabetyp
    })
}
```
]
]

== Codegen
// - Ziel: WASM-Bytecode erzeugen.
// - Eigene kleine IR (`IrInstruction`) als Zwischenschicht.
// - Mapping von IR auf `wasm_encoder::Instruction`.
// - ModuleGen: sammelt Typen, Imports, Funktionen, Exports, Code.
// - Host-Funktion `print` als Import (`env.print_i64`).

Der Codegenerator nimmt den AST und übersetzt ihn in eine eigene Zwischenrepräsentation (IR, Intermediate Representation), die aus einer linearen Folge von Instruktionen besteht. Diese IR abstrahiert von den Details der WASM-Generierung und ermöglicht eine klarere Trennung zwischen der Logik der Codeerzeugung und den spezifischen Anforderungen des WASM-Formats. Zusätzlich erleichtert die IR spätere Optimierungen, da Transformationen nicht mehr direkt auf der Quellsyntax arbeiten müssen. Der Einsatz einer IR entspricht der Praxis großer Compiler-Infrastrukturen, wie z.B. LLVM IR als zentrale Zwischenschicht @llvm-langref.
In diesem Fall ist die IR eine einfache Aufzählung von Instruktionen, die direkt auf den WASM-Stack operieren. Zum Beispiel wird ein `if`-Statement in der IR durch eine Sequenz von Instruktionen dargestellt, die die Bedingung evaluieren, dann eine `If`-Instruktion mit einem Blocktyp für den `then`-Teil und optional eine `Else`-Instruktion für den `else`-Teil enthält. Ähnlich wird eine `while`-Schleife durch einen `Block` und einen `Loop` mit entsprechenden Sprunginstruktionen (`BrIf`, `Br`) modelliert.

#figure(kind: "code", caption: [IR-Instruktionen (Quelle: src/codegen/ir.rs)])[
  #code-box[
```rust
pub enum IrInstruction{
    I64Const(i64), // Laden einer Konstanten
    I64Eqz, // Vergleich: ist 0?
    BrIf(u32), // Bedingter Sprung (z.B. für `if` oder `while`)
    Br(u32), // Unbedingter Sprung (z.B. für `break` in `while`)
    LocalSet(u32), // Setzen einer lokalen Variable
    LocalGet(u32), // Laden einer lokalen Variable
    Call(u32), // Funktionsaufruf
    If(BlockType), // Wenn-Block
    Else, // Sonst-Block
    Block(BlockType), // Block
    Loop(BlockType), // Schleife
    Drop, // Wert vom Stack entfernen
    Return, // Rückgabe
    End, // Ende

    // Arithmetik
    I64Add, // Addition
    I64Sub, // Subtraktion
    I64Mul, // Multiplikation
    I64DivS, // Division
    I64Eq, // Gleichheit
    I64LtS, // Kleiner als
    I64GtS, // Größer als
    I64ExtendI32S, // Erweiterung von i32 nach i64, nötig für Vergleiche mit 0
    I32Eqz, // Vergleich: ist 0?
}
```
]
]

Die IR wird letztendlich in echte WASM-Bytecode-Instruktionen umgewandelt, wobei die `wasm_encoder`-Bibliothek verwendet wird, um Module, Funktionen, Imports und Exports zu definieren.

#figure(kind: "code", caption: [Funktions-Emission (Quelle: src/codegen/module.rs)])[
  #code-box[
```rust
pub fn emit_function(&mut self, func: &FunctionDecl) {
    let mut gen = FuncGen {
        locals: Vec::new(), // Lokale Variablen (inkl. Parameter)
        local_map: HashMap::new(), // Mapping von Namen zu lokalen Indizes
        instructions: Vec::new(), // IR-Instruktionen
        has_return: func.return_type.is_some(), // Rückgabetyp vorhanden?
    };

    // Parameter werden auf lokale Indizes gemappt
    for (i, name) in func.params.iter().enumerate() {
        gen.local_map.insert(name.clone(), i as u32);
    }

    // Statements -> IR-Instruktionen
    for stmt in &func.body {
        emit_stmt(stmt, &mut gen, &self.func_indices);
    }
}
```
]
]

Die Host-Funktion `print` wird als Import unter dem Namen `env.print_i64` bereitgestellt, damit sie im generierten WASM-Modul aufgerufen werden kann und die Ausgabe von Ganzzahlen über die Konsole ermöglicht.
== Ausführung und Beispiel
Für diesen Selbstversuch wird der generierte WASM-Bytecode lokal mit der `wasmtime`-Laufzeit ausgeführt, anstatt ihn in einem Browser zu verwenden, auch wenn dies theoretisch möglich wäre. `wasmtime` ermöglicht das Laden von WASM-Modulen, die Instanziierung von Funktionen und den Aufruf von exportierten Funktionen wie `main` @wasmtime-crate-docs; @wasmtime-hello-world. Die Host-Funktion `print_i64` wird in Rust bereitgestellt und als Import in das WASM-Modul eingebunden, damit die `print(...)`-Funktion im generierten Code funktioniert. Der Ablauf umfasst die Umwandlung des Quelltexts in Tokens durch den eigenen Lexer, den Aufbau eines AST durch den Parser und die Generierung von WASM-Bytes durch den Codegenerator. Anschließend erfolgt die Ausführung mit Wasmtime.

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

    // Instanziierung mit Importen (z.B. `env.print_i64`)
    let instance = Instance::new(&mut store, &module, &[print_func.into()])
        .map_err(|e| format!("instance error: {}", e))?;

    // Aufruf der exportierten `main`-Funktion
    let func = instance
        .get_func(&mut store, "main")
        .ok_or_else(|| "function `main` not found".to_string())?;
    func.call(&mut store, &params, &mut results_buf)
        .map_err(|e| format!("runtime error: {}", e))?;
}
```
]
]

Die Ausführung eines Programms in dieser Programmiersprache startet also immer mit der `main`-Funktion, die als Einstiegspunkt dient. Alle anderen Funktionen müssen von `main` oder von anderen Funktionen aufgerufen werden, damit sie ausgeführt werden. Wenn `main` nicht definiert ist oder nicht exportiert wird, schlägt die Instanziierung des WASM-Moduls fehl, da der Einstiegspunkt fehlt.

=== Beispielprogramm: Fakultät
Hier ein Beispielprogramm, das die Fakultät einer Zahl berechnet. Es zeigt die Verwendung von Funktionen, Rekursion und die Ausgabe über die `print`-Funktion. Das Programm definiert eine Funktion `fact`, die die Fakultät berechnet, und eine `main`-Funktion, die `fact(10)` aufruft und das Ergebnis ausgibt.

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

Wenn man dieses Programm mit `cargo run -- factorial.eres` ausführt, wird die Ausgabe `3628800` (die Fakultät von 10) in der Konsole angezeigt, was die korrekte Funktion des Compilers und der Laufzeit bestätigt.

== Eigener Beitrag und verwendete Tools
Die Eigenleistung umfasst die vollständige Implementierung eines Mini-Compilers von der Quelltextanalyse (Lexer, Parser) über die Codegenerierung bis zur Ausführung des generierten WASM-Codes. Die Sprachsyntax wurde von mir definiert, und alle Komponenten des Compilers wurden eigenständig entwickelt. Zusätzlich habe ich Testfälle erstellt, um die Funktionalität von Lexer, Parser und der Laufzeit zu validieren.
Die verwendeten Tools umfassen die Programmiersprache Rust für die Implementierung, die `wasm-encoder`-Bibliothek für die Generierung von WASM-Bytecode, die `wasmtime`-Laufzeit für die Ausführung des generierten Codes, Git für Versionsverwaltung sowie Typst für die Erstellung dieser Facharbeit. Das zugehörige Projekt-Repository ist unter #link("https://github.com/justEres/facharbeit")[`github.com/justEres/facharbeit`] verfügbar. KI-Unterstützung wurde für die Strukturierung der Arbeit und das Brainstorming von Ideen genutzt, jedoch nur wenig für die eigentliche Code-Implementierung.

== Testwerkzeuge und CLI-Nutzung
Um die Entwicklung zu vereinfachen und die Funktionalität zu demonstrieren, wurden verschiedene Testwerkzeuge und CLI-Optionen implementiert. Die wichtigsten Befehle sind:

Token-Ausgabe (zeigt die Arbeit des Lexers):
#figure(kind: "code", caption: [CLI: Token-Stream prüfen (`--print-tokens`)])[
  #code-box[
```bash
$ cargo run -- add.eres --print-tokens
Tokens:
[
  Token { kind: Fn, ... },
  Token { kind: Ident("add"), ... },
  Token { kind: LParen, ... },
  ...
  Token { kind: EOF, ... }
]
```
]
]

AST-Ausgabe (zeigt die vom Parser erzeugte Baumstruktur):
#figure(kind: "code", caption: [CLI: AST prüfen (`--print-ast`)])[
  #code-box[
```bash
$ cargo run -- add.eres --print-ast
AST:
Program {
  functions: [
    FunctionDecl { name: "add", ... },
    FunctionDecl { name: "main", ... },
  ],
}
```
]
]

WAT-Ausgabe (zeigt die Codegen-Ausgabe als WebAssembly-Text):
#figure(kind: "code", caption: [CLI: WAT prüfen (`--print-wat`)])[
  #code-box[
```bash
$ cargo run -- add.eres --print-wat
Generated WAT:
(module
  (import "env" "print_i64" ...)
  (export "add" (func 1))
  (export "main" (func 2))
  ...
)
```
]
]

// === Ergebnistabelle (Selbstversuch)
// - Zweck: erwartetes und tatsächliches Verhalten pro Testfall strukturiert vergleichen.
// - Ausfüllen: kurze, überprüfbare Aussagen (kein Fließtext).
// - Bewertung: `ok`, `teilweise`, `nicht ok`.

// #table(
//   columns: (1.6fr, 2.1fr, 2.1fr, 0.8fr),
//   stroke: 0.5pt,
//   inset: 4pt,
//   table.header(
//     [Testfall],
//     [Erwartetes Ergebnis],
//     [Tatsächliches Ergebnis],
//     [Bewertung],
//   ),
//   [`add.eres` (`--print-tokens`)],
//   [Tokenfolge vollständig, inkl. `EOF`],
//   [Tokenfolge entspricht Erwartung],
//   [ok],
//   [`add.eres` (`--print-ast`)],
//   [AST mit `add` und `main` korrekt aufgebaut],
//   [AST-Struktur wie erwartet],
//   [ok],
//   [`add.eres` (`--print-wat`)],
//   [WAT-Modul mit Import/Export und aufrufbarer `main`],
//   [WAT wird erzeugt, Ausführung liefert `12`],
//   [ok],
//   [`factorial.eres` (Ausführung)],
//   [rekursive Fakultät für `10` ergibt `3628800`],
//   [Konsolenausgabe `3628800`],
//   [ok],
//   [Fehlerfall (z.B. fehlendes `;`)],
//   [Parser meldet klaren Fehler statt Absturz],
//   [eintragen],
//   [eintragen],
// )

= Fazit
Um die Leitfrage zu beantworten: WebAssembly erleichtert den Bau eigener Compiler für Amateurentwickler erheblich, da es ein einheitliches Ziel bietet und viele plattformspezifische Details im Backend abstrahiert. Allerdings bleibt die Entwicklung eines Compilers eine komplexe Aufgabe, insbesondere im Frontend (Lexing, Parsing, semantische Analyse). Die Abhängigkeit von Host-Imports und der Sandbox-Umgebung von WASM kann ebenfalls Einschränkungen mit sich bringen. Insgesamt ist WASM eine vielversprechende Plattform für Hobby-Compiler, aber es erfordert dennoch ein gewisses Maß an technischem Verständnis und Aufwand.

Mein persönliches Fazit ist, dass die Entwicklung dieses Mini-Compilers eine äußerst lehrreiche Erfahrung war. Sie hat mir geholfen, die theoretischen Konzepte des Compilerbaus in die Praxis umzusetzen und die Herausforderungen zu verstehen, die mit der Erstellung eines Compilers verbunden sind. Obwohl das Projekt eher ein Proof of Concept als ein produktionsreifer Compiler ist, hat es mir wertvolle Einblicke gegeben und meine Fähigkeiten im Bereich Compilerentwicklung deutlich verbessert.

Die größte Herausforderung lag in der Implementierung der Parserlogik, insbesondere bei der Handhabung von Operatorpräzedenz und der Fehlerbehandlung. Auch die Codegenerierung für WASM war anspruchsvoll, da ich mich mit den Details der WASM-Instruktionen und der Modulstruktur auseinandersetzen musste. Zusätzlich war das Tooling rund um die Compilerentwicklung, insbesondere Testen und Debuggen, zeitaufwendig.

Um das Projekt weiterzuführen, wären Verbesserungen in der Fehlerdiagnostik wünschenswert, z.B. durch genauere Fehlermeldungen mit Zeilen- und Spaltenangaben sowie die Möglichkeit zur Fehlererholung. Ein weiterer Schritt wäre die Implementierung eines Typcheckers, um statische Typfehler zu erkennen. Schließlich könnte der Sprachumfang erweitert werden, um weitere Datentypen, Kontrollstrukturen oder Funktionen zu unterstützen.

#pagebreak()

= Quellen
#box(width: 0pt, height: 0pt, clip: true)[
  #bibliography("bibliography.yaml", title: none)
]

Ausschließlich Online-Quellen, da keine gedruckten Bücher verwendet wurden. Alle Quellen sind frei zugänglich und wurden zum Zeitpunkt des Zugriffs überprüft. Die Auswahl der Quellen basiert auf ihrer Relevanz für die Themen Lexer, Parser, Codegen, WASM und Compilerbau im Allgemeinen.

#let refs = yaml("bibliography.yaml")
#let ref-order = (
  "wasmtime-hello-world",
  "wasmtime-crate-docs",
  "gcc-overall-options",
  "ibm-compiler",
  "llvm-langref",
  "mdn-wasm-concepts",
  "mdn-wasm-text-format",
  "crafting-parsing-expr",
  "crafting-scanning",
  "rustc-overview",
  "rustc-parser",
  "wasm-w3c-core",
  "wasm-spec",
)

#let de-date(d) = {
  let parts = d.split("-")
  if parts.len() == 3 {
    [#parts.at(2).#parts.at(1).#parts.at(0)]
  } else {
    [#d]
  }
}

#set par(justify: false, leading: 0.35em)
#for (i, key) in ref-order.enumerate() [
  #let r = refs.at(key)
  #(i + 1). #r.author: #r.title. Verfügbar unter: #link(r.url)[#r.url]. Zugriff am #de-date(r.accessed).
  #v(0.45em)
]
#set par(justify: true, leading: 0.95em)

= Einsatz von KI

Für die Facharbeit wurde KI als unterstützendes Hilfsmittel für Strukturierung, sprachliche Überarbeitung und Layout in Typst genutzt. Die inhaltliche Entwicklung des Selbstversuchs, der Compiler-Code und die technischen Entscheidungen wurden eigenständig erarbeitet.

#pagebreak()

#set page(margin: 0pt, numbering: none)
#image("formblaetter.pdf", page: 3, width: 100%, height: 100%)
#pagebreak()

#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 5cm),
  numbering: none,
)

= Anhang: Quellcode

#metadata(none) <appendix-code-start>

#outline(
  title: "Quellcode-Übersicht",
  target: heading.where(level: 3).after(<appendix-code-start>),
)

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
