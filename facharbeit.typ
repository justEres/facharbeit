

#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 5cm),
  numbering: "1 von 1",
  number-align: right + bottom
)


#set text(
  lang: "de",
  size: 12pt, 
  font: "Times New Roman"
)

// Nutzt die lokal installierte Schriftfamilie (siehe ~/.local/share/fonts)
#set par(justify: true, leading: 0.95em)

#show heading: set block(above: 0.9em, below: 0.3cm)
#show heading.where(level: 1): set block(below: 0.75cm)
#show heading.where(level: 1): set text(size: 16pt)
#show heading.where(level: 2): set block(below: 0.45cm)
#show heading.where(level: 2): set text(size: 13pt)
#show heading.where(level: 3): set block(below: 0.3cm)
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
#set page(margin: 0pt, numbering: none)
#image("formblaetter.pdf", page: 1, width: 100%, height: 100%)
#pagebreak()
#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 5cm),
  numbering: "1 von 1",
  number-align: right + bottom
)


#set page(numbering: none)
#outline(title: "Inhaltsverzeichnis")

#pagebreak()

#set page(
  numbering: "1 von 1",
  number-align: right + bottom,
)

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

Die Ergebnisse zeigen, dass WebAssembly in vielerlei Hinsicht den Einstieg in den Compilerbau erleichtert. Insbesondere die plattformunabhängige Natur von WASM und die Abstraktion von Hardwaredetails ermöglichen es Entwicklern, sich auf die Implementierung der Sprache und der Compiler-Logik zu konzentrieren, ohne sich um die spezifischen Anforderungen verschiedener Zielarchitekturen kümmern zu müssen.


= Compiler
// - Einordnung: Grundbegriffe des Compilerbaus
// - Fokus: Frontend, Backend, Zielformat WASM
// - Bezug zur Leitfrage: technischer Aufwand vs. Vereinfachung durch WASM
Was ist ein Compiler? Wie ist er aufgebaut? Welche Rolle spielt das Backend? Warum ist WebAssembly als Ziel interessant?

== Was ist ein Compiler?
// - Übersetzt Quellcode in eine andere Darstellung (meist maschinennahe Form).
// - Zielformen: Maschinencode, Bytecode oder Zwischenrepräsentation (IR) zur Weiterverarbeitung.
// - Abgrenzung zu Interpreter: Interpreter führt Code direkt aus, Compiler erzeugt ausführbare Repräsentation.
// - Vorteil: schnellere Ausführung, Optimierungen vorab möglich.
// - Nachteil: zusätzlicher Übersetzungsschritt, Fehler erst beim Kompilieren sichtbar.

Einen Compiler ist ein Programm, welches Programmcode in eine andere für Computer verständliche Form übersetzt. Dabei ist es ganz egal, ob es Binärcode für eine bestimmte Prozessorarchitektur, Bytecode für eine Virtuelle Maschine oder eine Zwischenrepräsentation für die Weiterverarbeitung ist. In Abgrenzung zu einem Interpreter führt ein Compiler den Code nicht direkt aus, sondern übersetzt ihn nur und führt dabei optional Optimierungen durch. Kompilierte Programme laufen dadurch in der Regel schneller als interpretierte Programme, da die Übersetzung bereits vor der Ausführung stattfindet und Optimierungen vorgenommen werden können. Zusätzlich erleichtern moderne Compiler den Entwicklern das Leben, indem sie häufige Fehler schon beim Übersetzen des Quellcodes finden und verständliche Fehlermeldungen ausgeben, während Interpreter Fehler erst zur Laufzeit sichtbar werden, was die Fehlersuche erschwert @ibm-compiler. Wenn der Begriff "Compiler" fällt, ist selten nur der reine Übersetzungsvorgang gemeint, sondern oft die gesamte Toolchain, die auch Assembler und Linker umfasst, um aus Quellcode eine ausführbare Datei zu erzeugen @gcc-overall-options. 

// Zusätzliche Stichpunkte:
// - Der Übersetzungsvorgang wird in der Praxis oft als Teil einer Toolchain betrachtet (inkl. Assembler und Linker) @gcc-overall-options.
// - Für Entwickler ist wichtig: Compiler-Fehler sind Diagnoseausgaben zur Übersetzungszeit und treten vor der Programmausführung auf @gcc-overall-options.



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

// Zusätzliche Stichpunkte:
// - Die Trennung in Frontend und Backend erlaubt, mehrere Sprachen auf dasselbe Backend abzubilden.
// - Zwischenrepräsentationen entkoppeln Sprachsyntax und Zielplattform und erleichtern Optimierungen @rustc-overview.
// - In realen Toolchains werden Vorverarbeitung, Kompilierung, Assemblierung und Linking als getrennte Schritte modelliert @gcc-overall-options.

// Quellen: @ibm-compiler; @rustc-overview; @rustc-parser

== Einführung in WebAssembly
// - WebAssembly (WASM): binäres, plattformunabhängiges Ausführungsformat.
// - Entstanden als Ziel für Webbrowser, inzwischen auch für Server und Tools nutzbar.
// - Ziel: nahe an nativer Geschwindigkeit, aber portabel und sicher.
// - Struktur: Module mit Funktionen, Speicher, Tabellen, Imports/Exports.
// - Ausführung in einer Sandbox; Zugriff auf Systemfunktionen über Host-Imports.
// - Unterstützte Sprachen: z.B. C/C++, Rust, AssemblyScript (via Compiler-Toolchains).
// - Relevanz für Compilerbau: einheitliches Target, weniger Plattformdetails im Backend.

WebAssembly (WASM) ist ein binäres, plattformunabhängiges Ausführungsformat, das ursprünglich für die Ausführung in Webbrowsern entwickelt wurde, aber inzwischen auch außerhalb des Webs, z.B. auf Servern oder in Tools, genutzt werden kann. Es zielt darauf ab, eine nahe an nativer Geschwindigkeit liegende Ausführung zu ermöglichen, während es gleichzeitig portabel und sicher bleibt. WASM-Module bestehen aus Funktionen, Speicher, Tabellen sowie Import- und Exportdefinitionen. Die Ausführung erfolgt in einer Sandbox-Umgebung, wobei der Zugriff auf Systemfunktionen über Host-Imports erfolgt. WASM wird von vielen Sprachen unterstützt, darunter C/C++, Rust und AssemblyScript, die über Compiler-Toolchains in WASM übersetzt werden können. Für den Compilerbau bietet WASM eine einheitliche Zielplattform, wodurch viele plattformspezifische Details im Backend entfallen. Unter anderem deswegen ist WASM besonders attraktiv für Hobby-Compiler, da es die Komplexität der Codegenerierung reduziert und den Fokus auf die Sprachlogik und -semantik ermöglicht @mdn-wasm-concepts.

Das Grundprinzip nach dem Wasm arbeitet ist die Stack-Maschine, bei der Instruktionen primär auf einem Operand-Stack operieren. Zum Beispiel nimmt die `add`-Instruktion die obersten zwei Werte vom Stack, addiert sie und legt das Ergebnis wieder auf den Stack. Dies ermöglicht eine einfache und effiziente Ausführung von Anweisungen, da keine expliziten Register oder Speicheradressen benötigt werden @mdn-wasm-text-format; @wasm-spec.

// - Stack erklären: Stapel teller, add instruktion nimmt die obersten 2 und legt das erbebnis drauf

// Zusätzliche Stichpunkte:
// - WASM ist als Stack-Maschine definiert: Instruktionen arbeiten primär auf einem Operand-Stack @mdn-wasm-text-format; @wasm-spec.
// - Module werden vor der Ausführung validiert (z.B. Typkonsistenz von Instruktionen) @wasm-w3c-core.
// - Textformat (WAT) und Binärformat bilden dieselbe Modulstruktur ab; WAT ist vor allem für Debugging und Lernen nützlich @mdn-wasm-text-format; @wasm-spec.

WebAssembly wird in Modulen verpackt, die Funktionen, Speicher, Tabellen sowie Import- und Exportdefinitionen enthalten. Ein valides WASM-Modul muss bestimmte Regeln erfüllen, damit es von der Laufzeitumgebung akzeptiert wird. Dazu gehören ein klarer Aufbau des Moduls, die Konsistenz von Funktionssignaturen, die Korrektheit des Kontrollflusses und die Gültigkeit referenzierter Indizes. Der Validator prüft diese Regeln vor der Ausführung, und bei Verstoß wird das Modul nicht instanziiert @wasm-w3c-core; @wasm-spec. 


// Stichpunkte: Was ein valides WASM-Modul ausmacht
// - Das Modul hat einen klaren Aufbau (Typen, Imports, Funktionen, Exports, optional Speicher/Tabellen).
// - Alle Funktionsaufrufe passen zu den deklarierten Funktionssignaturen (Parameter und Rückgabewerte).
// - Stack-Typen stimmen bei jeder Instruktion (keine "falschen" Werte auf dem Stack).
// - Kontrollfluss-Blöcke (`if`, `block`, `loop`) sind korrekt geöffnet und geschlossen (`end`).
// - Referenzierte Indizes sind gültig (z.B. `call 1` nur, wenn Funktion 1 existiert).
// - Imports/Exports sind konsistent benannt, damit Host und Modul zusammenpassen.
// - Der Validator prüft diese Regeln vor der Ausführung; bei Verstoß wird das Modul nicht instanziiert @wasm-w3c-core; @wasm-spec.


Hier ein Beispiel von Quellcode in unserer eigenen Sprache, der eine einfache Addition durchführt und das entsprechende WebAssembly Textformat (WAT), das daraus generiert wird. Das Beispiel zeigt, wie eine Funktion `add` definiert wird, die zwei Ganzzahlen addiert, und eine `main`-Funktion, die diese Addition ausführt und das Ergebnis über eine Host-Import-Funktion `print` ausgibt.

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
// - Ziel: Umsetzbarkeit der Theorie im eigenen Mini-Compiler prüfen
// - Fokus: vollständige Pipeline von Quelltext bis Ausführung
// - Ergebnisartefakte: Tokens, AST, WAT, Laufzeitausgabe

Die Konzepte des Compilerbaus und die Funktionsweise von WebAssembly wurden nun theoretisch erläutert. Um die praktische Umsetzbarkeit dieser Konzepte zu überprüfen, wird im folgenden Abschnitt ein eigener Mini-Compiler entwickelt. Dieser Compiler soll eine eigens definierte, minimalistische Programmiersprache in WebAssembly-Bytecode übersetzen. Dabei wird die gesamte Pipeline von der Quelltexteingabe über die Tokenisierung, das Parsing, die semantische Analyse bis hin zur Codegenerierung und Ausführung durchlaufen. Ziel ist es, nicht nur die technischen Schritte zu demonstrieren, sondern auch konkrete Artefakte wie die erzeugten Tokens, den abstrakten Syntaxbaum (AST), das generierte WAT und die Laufzeitausgabe zu präsentieren und nachvollziehbar zu machen.

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
Die Verschiedenen Token-Typen werden in einem Enum `TokenKind` modelliert, das die verschiedenen Kategorien von Tokens abdeckt, einschließlich Schlüsselwörtern, Literalen, Operatoren und Fehlern.

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



// Priorisierte Stichpunkte (Lexer):
// - [MUSS] Klarer Ablauf: Whitespace überspringen -> Zahl/Identifier/Operator erkennen -> Token ausgeben.
// - [MUSS] Trennlogik erklären: Ein Token endet, sobald ein Zeichen nicht mehr zur aktuellen Klasse passt.
// - [MUSS] Schlüsselwort vs. Identifier erklären: gleiche Lesephase, Entscheidung erst am Ende.
// - [MUSS] Fehlerfall erklären: unbekanntes Zeichen erzeugt direkt einen Lexer-Fehler.
// - [NICE] Typische Mini-Beispiele angeben: `let x=3;` -> `Let, Ident(x), Equal, Int(3), Semicolon`.
// - [STREICHEN] Vollständige Auflistung jedes einzelnen Tokens im Fließtext.

Beim Hauptlauf des Lexers wird der Quelltext zeichenweise durchlaufen. Zunächst werden alle Whitespace-Zeichen übersprungen, da sie für die Syntax keine Bedeutung haben. Sobald ein nicht-Whitespace-Zeichen gefunden wird, entscheidet der Lexer, ob es sich um den Beginn eines Identifiers, einer Zahl oder eines Operators handelt. Ein Identifier oder Schlüsselwort beginnt mit einem Buchstaben oder Unterstrich, gefolgt von alphanumerischen Zeichen oder Unterstrichen. Eine Zahl besteht ausschließlich aus Ziffern. Operatoren und Trennzeichen werden direkt erkannt. Sobald ein Token vollständig erkannt ist (z.B. wenn ein Nicht-Zeichen mehr zum aktuellen Token passt), wird es ausgegeben. 


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

// #figure(kind: "diagram", caption: [Zustandsmodell des Lexers])[
//   #diagram-box[
//   #set text(size: 8.5pt)
//   #diagram(
//   node-stroke: 0.7pt,
//   spacing: 1.5em,
//   node((-3.5,4), [Start], corner-radius: 2pt),
//   node((-1.5,4), [Zeichen?], corner-radius: 2pt),
//   edge((-3.5,4), (-1.5,4), "->"),

//   node((0,0), [Buchstabe/\_], corner-radius: 2pt),
//   edge((-1.5,4), (0,0), "-|>"),
//   node((2,0), [In\ Ident], corner-radius: 50%),
//   edge((0,0), (2,0), "->"),
//   node((4,0), [Emit\ Ident/Keyword], corner-radius: 2pt),
//   edge((2,0), (4,0), "->", [Ende], label-pos: 55%, label-side: left, label-sep: 2pt),

//   node((0,2), [Ziffer], corner-radius: 2pt),
//   edge((-1.5,4), (0,2), "-|>"),
//   node((2,2), [In\ Zahl], corner-radius: 50%),
//   edge((0,2), (2,2), "->"),
//   node((4,2), [Emit\ Zahl], corner-radius: 50%),
//   edge((2,2), (4,2), "->", [Ende], label-pos: 55%, label-side: left, label-sep: 2pt),

//   node((0,4), [Op/Trenner], corner-radius: 2pt),
//   edge((-1.5,4), (0,4), "->"),
//   node((2,4), [Emit\ Operator], corner-radius: 2pt),
//   edge((0,4), (2,4), "->"),

//   node((0,6), [Whitespace], corner-radius: 2pt),
//   edge((-1.5,4), (0,6), "-|>"),
//   node((2,6), [Skip], corner-radius: 2pt),
//   edge((0,6), (2,6), "->"),

//   node((0,8), [Unbekannt], corner-radius: 2pt),
//   edge((-1.5,4), (0,8), "-|>"),
//   node((2,8), [Fehler], corner-radius: 2pt),
//   edge((0,8), (2,8), "->"),
// )
// ]
// ]

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

// Erläuterung:
// - Zeichenfolge wird gesammelt und dann gegen Schlüsselwörter geprüft.
// - Alles, was kein Schlüsselwort ist, wird als `Ident(...)` behandelt.

// #figure(kind: "code", caption: [Zahlen-Lexing (vereinfacht, Quelle: src/lexer.rs)])[
//   #code-box[
// ```rust
// fn lex_number(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> TokenKind {
//     let mut text = String::new();
//     while let Some(&c) = chars.peek() {
//         if c.is_ascii_digit() {
//             text.push(c);
//             chars.next();
//         } else {
//             break;
//         }
//     }

//     match text.parse::<i64>() {
//         Ok(value) => TokenKind::Int(value),
//         Err(_) => TokenKind::Error,
//     }
// }
// ```
// ]
// ]

#pagebreak()

== Parser
// - Aufbau eines AST aus Tokens.
// - Einstieg: `parse_program` sammelt Funktionen bis `EOF`.
// - Funktionen: `fn name(params) -> Int { ... }`.
// - Block: Sequenz von Statements in `{ ... }`.
// - Ausdrucksparser mit Präzedenzregeln für Operatoren.

Der Parser nimmt die vom Lexer erzeugte Tokenliste und baut daraus einen abstrakten Syntaxbaum (AST, Abstract Syntax Tree) auf, der die hierarchische Struktur des Programms widerspiegelt. Die hier verwendete Strategie nennt sich rekursiver Abstieg, wobei durch Rekursion die hierarchische Natur der Sprache direkt in der Parserlogik abgebildet wird. @crafting-parsing-expr So können beispielsweise Blöcke, die aus einer Sequenz von Statements bestehen, einfach durch eine Funktion `parse_block` umgesetzt werden, die so lange Statements parst, bis sie das schließende `}` findet. 




Der Einstiegspunkt ist die Funktion `parse_program`, die alle Funktionen im Quelltext sammelt, bis sie das End-Token (`EOF`) erreicht. Jede Funktion wird durch `fn name(params) -> Int { ... }` definiert, wobei der Rückgabetyp optional ist. Blöcke werden als Sequenzen von Statements in `{ ... }` dargestellt. Für Ausdrücke wird ein spezieller Parser mit Präzedenzregeln implementiert, um die korrekte Bindung von Operatoren sicherzustellen.

Zusätzliche Stichpunkte:
- Der gewählte Ansatz entspricht einem rekursiven Abstieg, bei dem Nichtterminale durch Funktionen umgesetzt werden @crafting-parsing-expr.
- Operator-Präzedenz wird typischerweise über eine Prioritätstabelle gesteuert, damit z.B. `*` stärker bindet als `+` @crafting-parsing-expr.
- Der AST trennt konkrete Syntax (Tokens, Klammern) von semantisch relevanter Struktur (Ausdrücke, Statements) @rustc-parser.
- [MUSS] Operator-Präzedenz: `*` und `/` werden vor `+` und `-` gebunden.
- [MUSS] Beispiel zur Präzedenz: `1 + 2 * 3` ergibt AST-Form `Add(1, Mul(2, 3))`.
- [MUSS] Assoziativität: `10 - 3 - 2` wird als `(10 - 3) - 2` geparst (linksassoziativ).
- [MUSS] Fehlerstrategie: bei unerwartetem Token sofort `Err(...)`, kein Weiterparsen.
- [MUSS] Ablauf `parse_expr`: zuerst linker Basis-Ausdruck, dann Operatoren in Präzedenz-Reihenfolge anhängen.
- [NICE] Mini-Beispiel AST-Form: Quelle `a + b * c` -> `Add(Var(a), Mul(Var(b), Var(c)))`.
- [NICE] Scope-Verhalten bei Blöcken benennen: Variablenzuordnung über lokale Indizes statt echter Symboltabellen-Hierarchie.
- [NICE] Optional 1 Tabelle mit Operatoren + Priorität + Assoziativität.
- [STREICHEN] sehr detaillierte Randfälle (z.B. alle nicht unterstützten Syntaxformen) nur kurz nennen statt breit ausführen.

Umsetzung der [MUSS]-Punkte (stichpunktartig):
- Präzedenz wird als Zahl modelliert (`*` > `+`).
- Die `while`-Schleife in `parse_expr` hängt so lange Operatoren an, wie deren Präzedenz hoch genug ist.
- Linksassoziativität entsteht durch `parse_expr(prec + 1)` auf der rechten Seite.
- Fehler entstehen zentral über `expect(...)` und werden als `Result::Err` weitergegeben.

#figure(kind: "code", caption: [Operator-Präzedenz (vereinfacht, Quelle: src/parser.rs)])[
  #code-box[
```rust
fn precedence(kind: &TokenKind) -> Option<u8> {
    match kind {
        TokenKind::Star | TokenKind::Slash => Some(20),
        TokenKind::Plus | TokenKind::Minus => Some(10),
        _ => None,
    }
}

fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
    let mut left = self.parse_primary()?;

    while let Some(op_prec) = precedence(&self.peek().kind) {
        if op_prec < min_prec {
            break;
        }

        let op = self.bump().kind.clone();
        let right = self.parse_expr(op_prec + 1)?; // linksassoziativ
        left = Expr::Binary(Box::new(left), op, Box::new(right));
    }

    Ok(left)
}
```
]
]

#figure(kind: "code", caption: [Statement-Dispatch (vereinfacht, Quelle: src/parser.rs)])[
  #code-box[
```rust
fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
    match self.peek().kind {
        TokenKind::Let => self.parse_let(),
        TokenKind::Return => self.parse_return(),
        TokenKind::If => self.parse_if(),
        TokenKind::While => self.parse_while(),
        _ => self.parse_expr_stmt(),
    }
}
```
]
]

#figure(kind: "code", caption: [Fehlerstrategie mit `expect` (vereinfacht, Quelle: src/parser.rs)])[
  #code-box[
```rust
fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
    let found = self.bump().kind.clone();
    if found == expected {
        Ok(())
    } else {
        Err(ParseError::new(expected, found))
    }
}
```
]
]


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
    self.expect(TokenKind::Fn)?;
    let name = self.expect_ident()?;
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
    let params = self.parse_params()?;
    self.expect(TokenKind::RParen)?;
```
]
]

// Optional streichen (reiner Layout-Text):
// Fortsetzung: Rückgabetyp und Funktionskörper

#figure(kind: "code", caption: [Funktionsparser Teil 3 (Quelle: src/parser.rs)])[
  #code-box[
```rust
    let return_type = if self.peek().kind == TokenKind::Arrow {
        self.bump();
        self.expect(TokenKind::IntType)?;
        Some(Type::Int)
    } else {
        None
    };

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

Der Codegenerator nimmt den AST und übersetzt ihn in eine eigene Zwischenrepräsentation (IR, Intermediate Representation), die aus einer linearen Folge von Instruktionen besteht. Diese IR abstrahiert von den Details der WASM-Generierung und ermöglicht eine klarere Trennung zwischen der Logik der Codeerzeugung und den spezifischen Anforderungen des WASM-Formats. Zusätzlich erleichtert die IR spätere Optimierungen, da Transformationen nicht mehr direkt auf der Quellsyntax arbeiten müssen. Der Einsatz einer IR entspricht der Praxis großer Compiler-Infrastrukturen, wie z.B. LLVM IR als zentrale Zwischenschicht @llvm-langref.

Die IR wird letztendlich in echte WASM-Bytecode-Instruktionen umgewandelt, wobei die `wasm_encoder`-Bibliothek verwendet wird, um Module, Funktionen, Imports und Exports zu definieren. 

Die Host-Funktion `print` wird als Import unter dem Namen `env.print_i64` bereitgestellt, damit sie im generierten WASM-Modul aufgerufen werden kann und die Ausgabe von Ganzzahlen über die Konsole ermöglicht.



// Zusätzliche Stichpunkte:
// - Eine IR erleichtert spätere Optimierungen, weil Transformationen nicht mehr direkt auf Quellsyntax arbeiten.
// - Der Einsatz einer IR entspricht der Praxis großer Compiler-Infrastrukturen (z.B. LLVM IR als zentrale Zwischenschicht) @llvm-langref.
// - Die Abbildung von Kontrollfluss (`if`, `while`) auf explizite Instruktionsfolgen ist ein zentraler Schritt vom AST zum Zielcode.
// - [MUSS] Mapping `if/else` in WASM herausstellen: Bedingung auf Stack -> `if` -> optional `else` -> `end`.
// - [MUSS] Mapping `while` in WASM herausstellen: `block` + `loop`, invertierte Bedingung mit `br_if`, Rücksprung mit `br 0`.
// - [MUSS] Return-Verhalten dokumentieren: explizites `return` im AST + Fallback-Return im generierten Code.
// - [NICE] Kurze Tabelle `AST-Konstrukt -> IR -> WASM` (je 1 Mini-Beispiel für Addition, Vergleich, `if/else`, `while`).
// - [NICE] Offener Punkt: Dead Code nach `return` wird noch emittiert (keine DCE/CFG-Bereinigung).
// - [STREICHEN] zu viele WASM-Details ohne direkten Bezug zu deinem eigenen Codepfad.

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

// Erläuterung:
// - `I64Const` entspricht dem Laden einer Konstante im WASM-Stack.
// - `LocalGet/LocalSet` stehen für Variablenzugriffe.
// - `I64Add` und `I64Eq` sind direkte WASM-Arithmetik/Vergleiche.
// - Kontrollfluss wird über `If/Else/Block/Loop/End` abgebildet.

#figure(kind: "code", caption: [Funktions-Emission (Quelle: src/codegen/module.rs)])[
  #code-box[
```rust
pub fn emit_function(&mut self, func: &FunctionDecl) {
    let mut gen = FuncGen {
        locals: Vec::new(),
        local_map: HashMap::new(),
        instructions: Vec::new(),
        has_return: func.return_type.is_some(),
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
// - Eigenleistung: Sprachsyntax festgelegt (`fn`, `let`, `if/else`, `while`, `return`)
// - Eigenleistung: Lexer, Parser, Codegen, Runtime-Anbindung umgesetzt
// - Eigenleistung: Testfälle für Lexer, Parser und Ausführung ergänzt
// - Verwendete Tools: Rust
// - Verwendete Tools: `wasm-encoder`
// - Verwendete Tools: `wasmtime`
// - Verwendete Tools: Typst
// - Unterstützung: KI für Strukturierung/Brainstorming

Die Eigenleistung umfasst die vollständige Implementierung eines Mini-Compilers von der Quelltextanalyse (Lexer, Parser) über die Codegenerierung bis zur Ausführung des generierten WASM-Codes. Die Sprachsyntax wurde von mir definiert, und alle Komponenten des Compilers wurden eigenständig entwickelt. Zusätzlich habe ich Testfälle erstellt, um die Funktionalität von Lexer, Parser und der Laufzeit zu validieren.
Die verwendeten Tools umfassen die Programmiersprache Rust für die Implementierung, die `wasm-encoder`-Bibliothek für die Generierung von WASM-Bytecode, die`wasmtime`-Laufzeit für die Ausführung des generierten Codes und Typst für die Erstellung dieser Facharbeit. KI-Unterstützung wurde für die Strukturierung der Arbeit und das Brainstorming von Ideen genutzt, jedoch nur wenig für die eigentliche Code-Implementierung.


== Testwerkzeuge und CLI-Nutzung
Um die Entwicklung zu vereinfachen und die Funktionalität zu demonstrieren, wurden verschiedene Testwerkzeuge und CLI-Optionen implementiert. Die wichtigsten Befehle sind:




// - Token-Ausgabe: `cargo run -- <datei> --print-tokens`
// - AST-Ausgabe: `cargo run -- <datei> --print-ast`
// - WAT-Ausgabe: `cargo run -- <datei> --print-wat`
// - Artefakt: Token-Stream (Lexer)
// - Artefakt: AST (Parser)
// - Artefakt: WAT (Codegen)
// - [MUSS] Fehlerfall demonstrieren (z.B. unerwartetes Token) inkl. Position/Context der Fehlermeldung.
// - [MUSS] Testmatrix (kurz, tabellarisch): arithmetische Ausdrücke, Vergleich + `if/else`, Schleife, Rekursion (`fact`), Grenzfall (leere Parameterliste/fehlendes Semikolon).
// - [NICE] Gegenüberstellung "erwartete Ausgabe vs. tatsächliche Ausgabe" für 2-3 Programme.
// - [STREICHEN] zu viele CLI-Varianten ohne Erkenntnisgewinn (auf Kernbefehle begrenzen).

// Kurze CLI-Beispiele mit verkürzter Ausgabe:

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




= Fazit
// - Zusammenfassung der zentralen Ergebnisse
// - Persönliche Stellungnahme zur Leitfrage
// - Selbstreflexion: Herausforderungen und Verbesserungen

// - Einheitliches Ziel (WASM) statt viele Plattformen
// - Backend-Aufwand reduziert
// - Frontend bleibt komplex
// - Abhängigkeit von Host-Imports/Sandbox
// - Eignung für Hobby-Compiler: deutlich besser, aber nicht trivial

// - Selbstversuch
//     - mehr proof of concept als geeignet für produktion
//     - sehr gut zum lernen  

// Optional streichen (bereits als Stichpunkte darunter enthalten):
Um die Leitfrage zu beantworten: WebAssembly erleichtert den Bau eigener Compiler für Amateurentwickler erheblich, da es ein einheitliches Ziel bietet und viele plattformspezifische Details im Backend abstrahiert. Allerdings bleibt die Entwicklung eines Compilers eine komplexe Aufgabe, insbesondere im Frontend (Lexing, Parsing, semantische Analyse). Die Abhängigkeit von Host-Imports und der Sandbox-Umgebung von WASM kann ebenfalls Einschränkungen mit sich bringen. Insgesamt ist WASM eine vielversprechende Plattform für Hobby-Compiler, aber es erfordert dennoch ein gewisses Maß an technischem Verständnis und Aufwand.

Mein Persönliches Fazit ist, dass die Entwicklung dieses Mini-Compilers eine äußerst lehrreiche Erfahrung war. Es hat mir geholfen, die theoretischen Konzepte des Compilerbaus in die Praxis umzusetzen und die Herausforderungen zu verstehen, die mit der Erstellung eines Compilers verbunden sind. Obwohl das Projekt mehr als Proof of Concept denn als produktionsreifer Compiler zu betrachten ist, hat es mir wertvolle Einblicke gegeben und meine Fähigkeiten im Bereich Compilerentwicklung deutlich verbessert.

Die größte Herausforderung lag in der Implementierung der Parserlogik, insbesondere bei der Handhabung von Operatorpräzedenz und der Fehlerbehandlung. Auch die Codegenerierung für WASM war komplex, da ich mich mit den Details der WASM-Instruktionen und der Modulstruktur auseinandersetzen musste. Das Tooling rund um die Entwicklung eines Compilers, einschließlich Testen und Debuggen, stellte ebenfalls eine Herausforderung dar.

Um das Projekt weiterzuführen, wären Verbesserungen in der Fehlerdiagnostik wünschenswert, z.B. durch genauere Fehlermeldungen mit Zeilen- und Spaltenangaben sowie die Möglichkeit zur Fehlererholung. Ein weiterer Schritt wäre die Implementierung eines Typcheckers, um statische Typfehler zu erkennen. Schließlich könnte der Sprachumfang erweitert werden, um weitere Datentypen, Kontrollstrukturen oder Funktionen zu unterstützen.

// - Zusammenfassung: WASM reduziert Backend-Komplexität
// - Persönliche Stellungnahme: Ziel der Arbeit erreicht / Leitfrage beantwortet
// - Selbstreflexion: größte Herausforderungen (Parserlogik, Codegen, Tooling)
// - Selbstreflexion: nächste Iteration (Fehlerdiagnostik, Typen, Sprachumfang)
// - [MUSS] Leitfrage final gewichten: WASM senkt Einstiegshürde im Backend, Gesamtkomplexität bleibt mittel-hoch wegen Frontend + Semantik.
// - [MUSS] Konkrete Antwortformel für den Schlusssatz: "erleichtert deutlich, ersetzt aber kein Compiler-Grundwissen".
// - [NICE] Ausblick (nächste Iteration): Typchecker, bessere Fehlermeldungen (Zeile/Spalte + Recovery), kleine Optimierungen (constant folding, dead code).
// - [STREICHEN] Wiederholung der Einleitungsaussagen ohne neue Bewertung.

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
  "crafting-parsing-expr",
  "crafting-scanning",
  "gcc-overall-options",
  "ibm-compiler",
  "llvm-langref",
  "mdn-wasm-concepts",
  "mdn-wasm-text-format",
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

// Formblätter (Restseiten)
#set page(margin: 0pt, numbering: none)
#image("formblaetter.pdf", page: 2, width: 100%, height: 100%)
#pagebreak()
#image("formblaetter.pdf", page: 3, width: 100%, height: 100%)
