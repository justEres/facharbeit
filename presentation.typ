#import "@preview/touying:0.7.1": *
#import themes.simple: *
#import "vendor/codly/codly.typ": *

#let bg = rgb("#0b1220")
#let bg-soft = rgb("#111c2f")
#let panel = rgb("#162338")
#let text-main = rgb("#e6edf7")
#let text-muted = rgb("#9fb1c9")
#let accent = rgb("#6ea8ff")

#show: simple-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: [WebAssembly als Abkürzung zum eigenen Compiler?],
    subtitle: [Präsentation zur Facharbeit],
    author: [Erik Tschöpe],
    date: [12. April 2026],
  ),
)

#show: codly-init.with()

#codly(
  fill: panel,
  radius: 10pt,
  stroke: none,
  inset: 12pt,
  languages: (
    rust: (name: "Rust", color: rgb("#f08d49")),
    bash: (name: "Shell", color: rgb("#8fd18a")),
    wat: (name: "WAT", color: rgb("#c48cff")),
  ),
)

#set page(
  fill: bg,
  margin: (x: 1.3cm, y: 1.0cm),
)

#set text(
  font: "Noto Sans CJK JP",
  fill: text-main,
  size: 22pt,
)

#set par(justify: false, leading: 0.9em)

#show heading.where(level: 1): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #v(0.4fr)
  #text(size: 15pt, fill: accent, tracking: 0.08em, weight: "medium")[KAPITEL]
  #v(0.35em)
  #text(size: 32pt, weight: "bold", fill: text-main)[#it.body]
]

#show heading.where(level: 2): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #v(0.55fr)
  #rect(width: 1.2cm, height: 0.09cm, radius: 999pt, fill: accent)
  #v(0.45em)
  #text(size: 28pt, weight: "bold", fill: text-main)[#it.body]
  #v(0.45em)
]

#let meta(body) = text(size: 14pt, fill: text-muted, tracking: 0.07em, weight: "medium")[#body]
#let lead(body) = text(size: 16pt, fill: text-muted)[#body]
#let card(body) = rect(
  fill: panel,
  stroke: (paint: rgb("#253754"), thickness: 0.8pt),
  radius: 14pt,
  inset: 16pt,
)[#body]

#slide[
  #v(0.34fr)
  #meta[PRÄSENTATION]
  #v(0.45em)
  #text(size: 34pt, weight: "bold")[WebAssembly als Abkürzung zum eigenen Compiler?]
  #v(0.5em)
  #lead[Von der wissenschaftlichen Arbeit zur deutlich direkteren Präsentation.]
  #v(1.3em)
  #text(size: 14pt, fill: text-muted)[Erik Tschöpe]
]

#slide[
  #v(0.18fr)
  #meta[STRUKTUR]
  #v(0.35em)
  #text(size: 28pt, weight: "bold")[Gliederung]
  #v(0.8em)
  #grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    gutter: 14pt,
    card([
      #text(size: 14pt, fill: accent, weight: "bold")[01 Einstieg]
      #v(0.35em)
      #text(size: 16pt)[Warum überhaupt?]
      #linebreak()
      #text(size: 16pt)[Leitfrage]
    ]),
    card([
      #text(size: 14pt, fill: accent, weight: "bold")[02 Grundlagen]
      #v(0.35em)
      #text(size: 16pt)[Compiler]
      #linebreak()
      #text(size: 16pt)[Frontend und Backend]
      #linebreak()
      #text(size: 16pt)[Warum WebAssembly?]
    ]),
    card([
      #text(size: 14pt, fill: accent, weight: "bold")[03 Selbstversuch]
      #v(0.35em)
      #text(size: 16pt)[Ziel des Projekts]
      #linebreak()
      #text(size: 16pt)[Sprache, Lexer, Parser]
      #linebreak()
      #text(size: 16pt)[Codegen und Ausführung]
    ]),
    card([
      #text(size: 14pt, fill: accent, weight: "bold")[04 Bewertung]
      #v(0.35em)
      #text(size: 16pt)[Vereinfachungen]
      #linebreak()
      #text(size: 16pt)[Bleibende Komplexität]
      #linebreak()
      #text(size: 16pt)[Fazit und Fragen]
    ]),
  )
]

= Einstieg

== Warum überhaupt ein eigener Compiler?

Compiler wirken oft wie ein Thema für große Teams oder Universitäten. Genau deshalb ist die Frage spannend, ob eine Einzelperson heute realistischer in dieses Feld einsteigen kann. Meine Präsentation startet also nicht bei Details, sondern bei der Motivation hinter dem ganzen Projekt.

== Die Leitfrage

Im Zentrum steht die Frage, ob WebAssembly den Bau eigener Compiler für Amateurentwickler spürbar erleichtert. Dabei geht es nicht darum, ob Compiler plötzlich einfach werden. Es geht darum, ob die schwierigste technische Hürde kleiner wird.

= Grundlagen

== Was macht ein Compiler?

Ein Compiler übersetzt Quellcode in eine Form, die ein Rechner ausführen kann. Dafür zerlegt er den Text, versteht seine Struktur und erzeugt am Ende Zielcode. Für die Präsentation reicht die Vorstellung einer klaren Pipeline mit mehreren aufeinander aufbauenden Schritten.

== Frontend und Backend

Im Frontend wird der Quelltext verstanden, also in Tokens, Syntax und Bedeutung zerlegt. Im Backend wird daraus ausführbarer Zielcode erzeugt. Diese Trennung ist wichtig, weil WebAssembly vor allem den Backend-Teil vereinfachen kann.

== Warum WebAssembly?

WebAssembly ist ein standardisiertes, plattformunabhängiges Zielformat. Dadurch muss ich nicht direkt nativen Maschinencode für verschiedene Systeme erzeugen. Für Hobbyprojekte ist genau das attraktiv, weil viele Plattformdetails aus dem eigenen Compiler verschwinden.

= Selbstversuch

== Ziel des Projekts

Statt nur Literatur auszuwerten, habe ich einen kleinen Compiler selbst gebaut. So konnte ich die Theorie direkt gegen ein echtes Projekt testen. Der Selbstversuch zeigt, an welchen Stellen WebAssembly tatsächlich hilft und an welchen nicht.

== Eigene Sprache im Überblick

Die Sprache ist bewusst klein gehalten und konzentriert sich auf Funktionen, Variablen, Bedingungen und Schleifen. Dadurch bleibt das Projekt überschaubar, ohne die eigentliche Compiler-Pipeline zu verfälschen. Das Ziel war kein fertiges Produkt, sondern ein nachvollziehbarer Prototyp.

== Lexer

Der Lexer ist der erste technische Schritt und zerlegt den Quelltext in Tokens. Hier entscheidet sich, ob aus bloßen Zeichen sinnvolle Bausteine wie Schlüsselwörter, Namen und Operatoren werden. Der Schritt ist noch relativ mechanisch, aber absolut grundlegend für alles Weitere.

```rust
fn main() {
    let value = 42;
    print(value);
}
```

== Parser

Der Parser baut aus diesen Tokens eine Struktur, die die Grammatik des Programms sichtbar macht. Erst hier wird also klar, was zusammengehört und wie Ausdrücke gebunden sind. Besonders wichtig war dabei die Behandlung von Blöcken, Funktionsdefinitionen und Operator-Präzedenz.

```rust
fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
    self.expect(TokenKind::Fn)?;
    let name = self.expect_ident()?;
    let body = self.parse_block()?;
    Ok(FunctionDecl { name, body, params: vec![], return_type: None })
}
```

== Codegen nach WebAssembly

Im Codegen wird aus der abstrakten Programmstruktur schließlich WebAssembly. Genau hier liegt der Kern meiner Leitfrage, denn ich musste keinen nativen Zielcode für eine konkrete CPU erzeugen. Stattdessen übersetzt der Compiler in ein kompaktes, validierbares Zwischenformat mit klaren Regeln.

```wat
(func (export "main")
  i64.const 7
  i64.const 5
  i64.add
  call 0
)
```

== Ausführung mit Wasmtime

Der generierte Code wurde nicht nur erzeugt, sondern auch direkt ausgeführt. Dafür habe ich Wasmtime als Laufzeit genutzt und eine Host-Funktion für `print` eingebunden. So lässt sich am Ende überprüfen, ob aus dem eigenen Quellcode wirklich ein funktionierendes Programm geworden ist.

```bash
cargo run -- factorial.eres --print-wat
cargo run -- factorial.eres
```

= Bewertung

== Was WebAssembly vereinfacht

WebAssembly nimmt dem Projekt vor allem einen großen Teil der Plattformabhängigkeit ab. Ich musste keine Registerbelegung, kein Linking für verschiedene Betriebssysteme und keine nativen Binärformate selbst lösen. Dadurch konnte ich mich stärker auf Sprachdesign und Compilerlogik konzentrieren.

== Wo die Komplexität bleibt

Die eigentliche Denkarbeit verschwindet trotzdem nicht. Lexer, Parser, Fehlerbehandlung und saubere interne Datenstrukturen bleiben anspruchsvoll. WebAssembly macht also nicht den ganzen Compilerbau leicht, sondern verschiebt den Fokus auf die wirklich sprachlichen Probleme.

== Fazit

Mein Ergebnis ist deshalb weder ein blindes Ja noch ein Nein. WebAssembly erleichtert den Einstieg deutlich, aber es ersetzt kein Verständnis für Compilerbau. Für Amateurentwickler wird das Ziel realistischer, jedoch nicht trivial.

= Abschluss

== Fragen?

Wenn ich diese Präsentation weiter ausbaue, kommen hier am Ende noch die wichtigsten Learnings in einem Satz zusammen. Für den Vortrag ist das die Stelle, an der ich auf Rückfragen, Kritik oder technische Nachfragen eingehen kann. So endet die Präsentation offen statt nur mit einer Abschlussfolie.
