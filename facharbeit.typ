

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

// Lokale Schriftdateien einbinden (Fallback, falls Systemfont fehlt)
#set text(font: "fonts/Times New Roman.ttf")
#set par(justify: true, leading: 0.75em)

#show heading.where(level: 1): set text(size: 13pt)

#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

#let code-box(body) = box(
  width: 100%,
  stroke: 0.5pt,
  inset: 6pt,
)[
  #set text(size: 9pt)
  #body
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

#set heading(numbering: "1.")

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
== Was ist ein Compiler?
- Übersetzt Quellcode in eine andere Darstellung (meist maschinennahe Form).
- Zielformen: Maschinencode, Bytecode oder Zwischenrepräsentation (IR) zur Weiterverarbeitung.
- Abgrenzung zu Interpreter: Interpreter führt Code direkt aus, Compiler erzeugt ausführbare Repräsentation.
- Vorteil: schnellere Ausführung, Optimierungen vorab möglich.
- Nachteil: zusätzlicher Übersetzungsschritt, Fehler erst beim Kompilieren sichtbar.

Einen Compiler ist ein Programm, welches Programmcode in eine andere für Computer verständliche Form übersetzt. Dabei ist es ganz egal, ob es Binärcode für eine bestimmte Prozessorarchitektur, Bytecode für eine Virtuelle Maschine oder eine Zwischenrepräsentation für die Weiterverarbeitung ist. In Abgrenzung zu einem Interpreter führt ein Compiler den Code nicht direkt aus, sondern übersetzt ihn nur und führt dabei optional Optimierungen durch. Kompilierte Programme laufen dadurch in der Regel schneller als interpretierte Programme, da 



== Aufbau eines Compilers (Frontend, Backend)
- Eingabephase: Quellcode wird gelesen und in Tokens zerlegt (Lexing).
- Syntaxanalyse: Parser baut einen Syntaxbaum (AST).
- Semantische Analyse: Typprüfung, Namensauflösung, Fehlerdiagnosen.
- Zwischenrepräsentation (IR): vereinheitlichte Form für Optimierungen.
- Optimierungen: z.B. konstante Ausdrücke ausrechnen, ungenutzten Code entfernen.
- Backend: IR in Zielcode übersetzen (z.B. Maschinencode oder WASM).
- Codeerzeugung: Registerallokation, Instruktionsauswahl, Plattformdetails.
- Ausgabe: Binärdatei, Objektdatei oder Bytecode.
Quellen: @wiki-compiler; @radford-compiler-phases

== Einführung in WebAssembly
- WebAssembly (WASM): binäres, plattformunabhängiges Ausführungsformat.
- Entstanden als Ziel für Webbrowser, inzwischen auch für Server und Tools nutzbar.
- Ziel: nahe an nativer Geschwindigkeit, aber portabel und sicher.
- Struktur: Module mit Funktionen, Speicher, Tabellen, Imports/Exports.
- Ausführung in einer Sandbox; Zugriff auf Systemfunktionen über Host-Imports.
- Unterstützte Sprachen: z.B. C/C++, Rust, AssemblyScript (via Compiler-Toolchains).
- Relevanz für Compilerbau: einheitliches Target, weniger Plattformdetails im Backend.

Beispiel (Quellcode → WAT):

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
#footnote[Quelle: add.eres]

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


#pagebreak()

= Selbstversuch

== Funktionsumfang der eigenen Programmiersprache
- Minimaler Datentyp: `Int` (Ganzzahl, intern `i64`).
- Programmbau: nur Funktionen, keine globalen Variablen.
- Statements: `let`, `return`, `if/else`, `while`, Ausdrucks-Statement.
- Ausdrücke: Literale, Variablen, Binäroperationen, Funktionsaufrufe.
- Vergleichsoperatoren: `==`, `!=`, `<`, `<=`, `>`, `>=`.
- Rückgabetyp optional: `-> Int` oder ohne Rückgabe.
- Print-Funktion als Host-Import (`print`).

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
#footnote[Quelle: src/ast.rs]

Erläuterung:
- `Stmt` beschreibt die möglichen Anweisungen der Sprache.
- `FunctionDecl` hält Signatur und Funktionskörper zusammen.

== Lexer / Scanner
- Aufgabe: Quelltext in Tokenliste umwandeln (inkl. Position/Span).
- Erkannt: Schlüsselwörter (`fn`, `let`, `if`, `else`, `while`, `return`).
- Erkannt: Literale (`Int`) und Identifier.
- Operatoren und Trennzeichen: `+ - * / % ( ) { } , ; : ->`.
- Fehlerbehandlung: unerwartete Zeichen, ungültige Zahlen.

#diagram(
  node-stroke: 0.7pt,
  spacing: 2.0em,
  node((-2,4), [Start], corner-radius: 2pt),
  edge((-2,4), (2,0), "-|>", [Buchstabe/\_], label-pos: 75%, label-side: left, label-sep: 4pt),
  node((2,0), [In\ Ident], corner-radius: 2pt),
  edge((2,0), (4,0), "-|>", [Ende\ Ident], label-pos: 70%, label-side: right, label-sep: 3pt),
  node((4,0), [Emit\ Ident/Keyword], corner-radius: 2pt),

  edge((-2,4), (2,2), "-|>", [Ziffer], label-pos: 75%, label-side: left, label-sep: 4pt),
  node((2,2), [In\ Zahl], corner-radius: 2pt),
  edge((2,2), (4,2), "-|>", [Ende\ Zahl], label-pos: 70%, label-side: right, label-sep: 3pt),
  node((4,2), [Emit\ Zahl], corner-radius: 2pt),

  edge((-2,4), (2,4), "-|>", [Op/Trenner], label-pos: 60%, label-side: left, label-sep: 4pt),
  node((2,4), [Emit\ Operator], corner-radius: 2pt),

  edge((-2,4), (2,6), "-|>", [Whitespace], label-pos: 55%, label-side: left, label-sep: 4pt),
  node((2,6), [Skip], corner-radius: 2pt),

  edge((-2,4), (2,8), "-|>", [Unbekannt], label-pos: 70%, label-side: right, label-sep: 4pt),
  node((2,8), [Fehler], corner-radius: 2pt),
)

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
#footnote[Quelle: src/token.rs]

Erläuterung:
- Die Tokenliste ist die gemeinsame Sprache zwischen Lexer und Parser.
- `Ident(String)` und `Int(i64)` tragen bereits konkrete Werte.

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
#footnote[Quelle: src/lexer.rs]

Erläuterung:
- Zeichenfolge wird gesammelt und dann gegen Schlüsselwörter geprüft.
- Alles, was kein Schlüsselwort ist, wird als `Ident(...)` behandelt.

#pagebreak()

== Parser
- Aufbau eines AST aus Tokens.
- Einstieg: `parse_program` sammelt Funktionen bis `EOF`.
- Funktionen: `fn name(params) -> Int { ... }`.
- Block: Sequenz von Statements in `{ ... }`.
- Ausdrucksparser mit Präzedenzregeln für Operatoren.

#diagram(
  node-stroke: 0.7pt,
  spacing: 2.0em,
  node((-2,4), [Stmt\ Start], corner-radius: 2pt),
  edge((-2,4), (2,0), "-|>", [`let`], label-pos: 55%, label-side: left, label-sep: 4pt),
  node((2,0), [parse_let], corner-radius: 2pt),
  edge((2,0), (4,0), "-|>", [;], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((4,0), [Ende], corner-radius: 2pt),

  edge((-2,4), (2,2), "-|>", [`return`], label-pos: 75%, label-side: right, label-sep: 0pt),
  node((2,2), [parse_return], corner-radius: 2pt),
  edge((2,2), (4,2), "-|>", [;], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((4,2), [Ende], corner-radius: 2pt),

  edge((-2,4), (2,4), "-|>", [`if`], label-pos: 45%, label-side: left, label-sep: 4pt),
  node((2,4), [parse_if], corner-radius: 2pt),
  edge((2,4), (4,4), "-|>", [`else`?], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((4,4), [then/else\ Block], corner-radius: 2pt),
  edge((4,4), (6,4), "-|>", [Ende], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((6,4), [Ende], corner-radius: 2pt),

  edge((-2,4), (2,6), "-|>", [`while`], label-pos: 55%, label-side: left, label-sep: 4pt),
  node((2,6), [parse_while], corner-radius: 2pt),
  edge((2,6), (4,6), "-|>", [Block], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((4,6), [while\ Body], corner-radius: 2pt),
  edge((4,6), (6,6), "-|>", [Ende], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((6,6), [Ende], corner-radius: 2pt),

  edge((-2,4), (2,8), "-|>", [sonst], label-pos: 55%, label-side: left, label-sep: 4pt),
  node((2,8), [parse_expr_stmt], corner-radius: 2pt),
  edge((2,8), (4,8), "-|>", [;], label-pos: 60%, label-side: right, label-sep: 3pt),
  node((4,8), [Ende], corner-radius: 2pt),
)

Hinweis: `parse_expr` arbeitet rekursiv (z.B. Klammern, Binäroperatoren).

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
#footnote[Quelle: src/parser.rs]

Erläuterung:
- Der Parser sammelt alle Funktionen bis zum End-Token.
- Ergebnis ist ein `Program` als Einstiegsknoten des AST.

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
#footnote[Quelle: src/parser.rs]

Erläuterung:
- Erwartet `fn`, danach den Namen und die Parameterliste.
- Rückgabetyp ist optional und wird nur bei `-> Int` gesetzt.
- Der Funktionskörper ist ein Block mit Statements.

Fortsetzung: Parameterliste

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
#footnote[Quelle: src/parser.rs]

Fortsetzung: Rückgabetyp und Funktionskörper

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
#footnote[Quelle: src/parser.rs]

#pagebreak()

== Codegen
- Ziel: WASM-Bytecode erzeugen.
- Eigene kleine IR (`IrInstruction`) als Zwischenschicht.
- Mapping von IR auf `wasm_encoder::Instruction`.
- ModuleGen: sammelt Typen, Imports, Funktionen, Exports, Code.
- Host-Funktion `print` als Import (`env.print_i64`).

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
#footnote[Quelle: src/codegen/ir.rs]

Erläuterung:
- `I64Const` entspricht dem Laden einer Konstante im WASM-Stack.
- `LocalGet/LocalSet` stehen für Variablenzugriffe.
- `I64Add` und `I64Eq` sind direkte WASM-Arithmetik/Vergleiche.
- Kontrollfluss wird über `If/Else/Block/Loop/End` abgebildet.

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
#footnote[Quelle: src/codegen/module.rs]

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
#footnote[Quelle: src/runner.rs]

Beispiel (Fakultät):

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
#footnote[Quelle: factorial.eres]


#pagebreak()

= Fazit

- Einheitliches Ziel (WASM) statt viele Plattformen
- Backend-Aufwand reduziert
- Frontend bleibt komplex
- Abhängigkeit von Host-Imports/Sandbox
- Eignung für Hobby-Compiler: deutlich besser, aber nicht trivial

#pagebreak()


= Quellen
#bibliography("bibliography.yaml", title: none)


= Todo

- Diagramme / Programmcode durchnummerieren

- Einleitungssätze für kapitel vorm ersten inhalt

- was hab ich selbst gemacht
- was an tools hab ich benutzt:
    - rust als Programmiersprache
    - wasm encoder 

- tools zum testen:
    - tokens 
    - ast 
    - wat 


- Fazit:
    - Zusammenfassung
    - persönliche Stellungnahme
    - selbstreflektion
        - herausforderungen
        - was beim nächsten mal anders machen

    
