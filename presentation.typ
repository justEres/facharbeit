#import "@preview/touying:0.7.1": *
#import themes.simple: *
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

#let bg = rgb("#0b1220")
#let panel = rgb("#162338")
#let text-main = rgb("#e6edf7")
#let text-muted = rgb("#9fb1c9")
#let accent = rgb("#6ea8ff")

// Layout-Tuning:
// - Boxbreite: width: ...
// - Boxhöhe: height: ...
// - Innenabstand: inset: ...
// - Abstand zwischen Boxen: gutter: ... / spacing: ...

#show: simple-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: [WebAssembly als Abkürzung zum eigenen Compiler?],
    subtitle: [Präsentation zur Facharbeit],
    author: [Erik Tschöpe],
    date: [14. April 2026],
  ),
)

#set page(
  fill: bg,
  margin: (x: 0.82cm, y: 0.66cm),
)

#set text(
  font: "Noto Sans CJK JP",
  fill: text-main,
  size: 22pt,
)

#set par(justify: false, leading: 1.0em)

#show heading.where(level: 1): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #text(size: 14pt, fill: text-muted, tracking: 0.07em, weight: "medium")[BLOCK]
  #v(0.18em)
  #text(size: 33pt, weight: "bold", fill: text-main)[#it.body]
]

#show heading.where(level: 2): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #text(size: 14pt, fill: text-muted)[#utils.display-current-heading(level: 1)]
  #v(0.1em)
  #text(size: 29pt, weight: "bold", fill: text-main)[#it.body]
  #v(0.22em)
  #rect(width: 1.1cm, height: 0.08cm, radius: 999pt, fill: accent)
  #v(0.3em)
]

#let meta(body) = text(size: 14pt, fill: text-muted, tracking: 0.07em, weight: "medium")[#body]
#let lead(body) = text(size: 19pt, fill: text-muted)[#body]

#let bullet-list(items) = {
  let rendered = ()
  for item in items {
    rendered.push([
      #grid(
        columns: (0.45cm, 1fr),
        gutter: 0.34cm,
        align(center + horizon)[#text(fill: accent, size: 19pt)[•]],
        text(size: 24.5pt, fill: text-main)[#item],
      )
    ])
  }
  box(width: 90%)[
    #stack(spacing: 0.56em, ..rendered)
  ]
}

#let outline-card(title, lines, height: 4.8cm) = rect(
  fill: panel,
  stroke: (paint: rgb("#253754"), thickness: 0.8pt),
  radius: 14pt,
  inset: 12pt,
  width: 100%,
  height: height,
)[
  #text(size: 12.5pt, fill: accent, weight: "bold")[#title]
  #v(0.22em)
  #for line in lines [
    #text(size: 13.2pt)[#line]
    #linebreak()
  ]
]

#let code-example(body, width: 74%) = block(width: width)[
  #show raw.where(block: true): set text(
    font: "Noto Sans Mono CJK JP",
    size: 13.2pt,
    fill: text-main,
  )
  #show raw.where(block: true): set par(leading: 0.72em)
  #box(
    inset: (x: 14pt, y: 15pt),
    fill: panel,
    stroke: 0.6pt + rgb("#24344d"),
    radius: 10pt,
    width: 100%,
  )[
    #body
  ]
]

#let syntax-card(title, tone, body, height: 4.8cm) = rect(
  fill: panel,
  stroke: 0.8pt + rgb("#253754"),
  radius: 14pt,
  inset: 13pt,
  width: 100%,
  height: height,
)[
  #text(size: 12pt, fill: tone, weight: "bold", tracking: 0.05em)[#title]
  #v(0.22em)
  #show raw.where(block: true): set text(
    font: "Noto Sans Mono CJK JP",
    size: 10.9pt,
    fill: text-main,
  )
  #show raw.where(block: true): set par(leading: 0.7em)
  #body
]

#let scratch-chip(fill-color, label) = rect(
  fill: fill-color,
  radius: 999pt,
  inset: (x: 10pt, y: 5pt),
  width: 92%,
)[
  #text(size: 11pt, fill: rgb("#08101b"), weight: "bold")[#label]
]

#let scratch-card(height: 4.8cm) = rect(
  fill: panel,
  stroke: 0.8pt + rgb("#253754"),
  radius: 14pt,
  inset: 13pt,
  width: 100%,
  height: height,
)[
  #text(size: 12pt, fill: rgb("#ffb454"), weight: "bold", tracking: 0.05em)[SCRATCH]
  #v(0.24em)
  #stack(
    spacing: 7pt,
    scratch-chip(rgb("#ffab19"), [wenn grüne Flagge angeklickt]),
    scratch-chip(rgb("#4c97ff"), [frage "Wie heißt du?" und warte]),
    scratch-chip(rgb("#59c059"), [sage (Antwort) für 2 Sekunden]),
  )
]

#let compiler-flow() = box(
  width: 92%,
  height: 6.4cm,
  inset: 10pt,
  fill: panel,
  stroke: 0.6pt + rgb("#24344d"),
  radius: 10pt,
)[
  #align(center + horizon)[
    #set text(fill: text-main, size: 12.5pt)
    #diagram(
      cell-size: 14mm,
      node-fill: rgb("#27456d"),
      node-stroke: 1.3pt + accent,
      edge-stroke: 1.4pt + accent,
      node((0, 0), [Quelltext], name: <source>, width: 2.7cm, height: 1.0cm, corner-radius: 8pt),
      node((1.6, 0), [Lexer], name: <lexer>, width: 2.2cm, height: 1.0cm, corner-radius: 8pt),
      node((3.2, 0), [Parser], name: <parser>, width: 2.2cm, height: 1.0cm, corner-radius: 8pt),
      node((4.8, 0), [Codegen], name: <codegen>, width: 2.4cm, height: 1.0cm, corner-radius: 8pt),
      node((6.5, 0), [WebAssembly], name: <wasm>, width: 3.0cm, height: 1.0cm, corner-radius: 8pt),
      edge(<source>, <lexer>, "->"),
      edge(<lexer>, <parser>, "->"),
      edge(<parser>, <codegen>, "->"),
      edge(<codegen>, <wasm>, "->"),
      node((1.6, 1.45), [Tokenliste], width: 2.6cm, height: 0.82cm, corner-radius: 7pt),
      edge(<lexer>, (1.6, 1.1), "->"),
      node((3.2, 1.45), [AST], width: 1.8cm, height: 0.82cm, corner-radius: 7pt),
      edge(<parser>, (3.2, 1.1), "->"),
    )
  ]
]

#let feature-table() = box(width: 94%)[
  #set text(size: 15pt)
  #table(
    columns: (2.35fr, 1.15fr, 1.5fr),
    stroke: none,
    inset: 8pt,
    fill: (x, y) => if y == 0 { rgb("#20314a") } else if calc.odd(y) { panel } else { rgb("#132033") },
    align: (left, center, left),
    [*Bereich*], [*Im Projekt*], [*Bewusst weggelassen*],
    [Datentypen], [Nur `Int`], [Keine Floats, Strings, Arrays],
    [Kontrollfluss], [`if`, `while`, `return`], [Kein `for`, kein `match`],
    [Funktionen + Variablen], [Eigene Funktionen, `let`], [Keine Methoden, keine Overloads, kein komplexes Typsystem],
    [Komfort], [`print(...)`], [Keine Bibliothek, keine Collections],
  )
]

#let pill(label, tone: accent, fill-color: panel) = rect(
  fill: fill-color,
  stroke: 0.8pt + rgb("#29405f"),
  radius: 999pt,
  inset: (x: 15pt, y: 12pt),
)[
  #text(size: 14pt, fill: tone, weight: "bold")[#label]
]

#let compact-pill(label, tone: accent, fill-color: panel) = rect(
  fill: fill-color,
  stroke: 0.8pt + rgb("#29405f"),
  radius: 999pt,
  inset: (x: 12pt, y: 6pt),
)[
  #text(size: 13pt, fill: tone, weight: "bold")[#label]
]

#let flow-card(title, body, width: 100%, height: auto, inset: 12pt) = rect(
  fill: panel,
  stroke: 0.8pt + rgb("#253754"),
  radius: 14pt,
  inset: inset,
  width: width,
  height: height,
)[
  #text(size: 11.5pt, fill: accent, weight: "bold", tracking: 0.05em)[#title]
  #v(0.2em)
  #body
]

#let logo-badge(path, label, tone: accent) = rect(
  fill: panel,
  stroke: 0.8pt + rgb("#253754"),
  radius: 14pt,
  inset: 10pt,
  width: 100%,
  height: 3.15cm,
)[
  #align(center + horizon)[
    #box(fill: rgb("#f3f7fb"), inset: 9pt, radius: 999pt)[
      #image(path, height: 0.82cm)
    ]
    #v(0.16em)
    #text(size: 12.5pt, fill: tone, weight: "bold")[#label]
  ]
]

#let title-visual() = grid(
  columns: (1fr, auto, 1fr, auto, 1fr, auto, 1fr),
  gutter: 10pt,
  align: center + horizon,
  pill([Eigene Sprache], tone: rgb("#9ad0ff")),
  text(fill: text-muted, size: 18pt)[→],
  pill([Compiler], tone: accent),
  text(fill: text-muted, size: 18pt)[→],
  pill([WebAssembly], tone: rgb("#8ec07c")),
  text(fill: text-muted, size: 18pt)[→],
  pill([Ausführung], tone: rgb("#ffb454")),
)

#let question-visual() = box(width: 92%)[
  #grid(
    columns: (1fr, auto, 1fr, auto, 1fr),
    gutter: 12pt,
    align: center + horizon,
    flow-card([AUSGANGSFRAGE], height: 2.8cm)[
      #align(center + horizon)[#text(size: 18pt, weight: "bold")[Beste Sprache?]]
    ],
    text(fill: text-muted, size: 20pt)[→],
    flow-card([NÄCHSTER GEDANKE], height: 2.8cm)[
      #align(center + horizon)[#text(size: 18pt, weight: "bold")[Eigene Sprache?]]
    ],
    text(fill: text-muted, size: 20pt)[→],
    flow-card([FACHARBEIT], height: 2.8cm)[
      #align(center + horizon)[#text(size: 18pt, weight: "bold")[Mit WASM leichter?]]
    ],
  )
]

#let project-visual() = box(width: 92%)[
  #grid(
    columns: (1fr, auto, 1fr, auto, 1fr),
    gutter: 12pt,
    align: center + horizon,
    flow-card([EINGABE], height: 3.85cm)[
      #text(size: 17pt, weight: "bold")[Eigene Sprache]
      #v(0.18em)
      #text(size: 13pt, fill: text-muted)[klein und bewusst reduziert]
    ],
    text(fill: text-muted, size: 20pt)[→],
    flow-card([UMWANDLUNG], height: 3.85cm)[
      #text(size: 17pt, weight: "bold")[Compiler]
      #v(0.18em)
      #text(size: 13pt, fill: text-muted)[selbst gebaut in Rust]
    ],
    text(fill: text-muted, size: 20pt)[→],
    flow-card([ERGEBNIS], height: 3.85cm)[
      #text(size: 17pt, weight: "bold")[WebAssembly]
      #v(0.18em)
      #text(size: 13pt, fill: text-muted)[ausführbarer Zielcode]
    ],
  )
]

#let compiler-interpreter-visual() = grid(
  columns: (1fr, 1fr),
  gutter: 12pt,
  flow-card([INTERPRETER], height: 6.6cm)[
    #text(size: 15pt, fill: text-muted)[typische Beispiele]
    #v(0.18em)
    #grid(
      columns: (1fr, 1fr),
      gutter: 9pt,
      logo-badge("assets/logos/python.svg", [Python], tone: rgb("#7cc7ff")),
      logo-badge("assets/logos/javascript.svg", [JavaScript], tone: rgb("#f0c24b")),
    )
  ],
  flow-card([COMPILER], height: 6.6cm)[
    #text(size: 15pt, fill: text-muted)[typische Beispiele]
    #v(0.18em)
    #grid(
      columns: (1fr, 1fr),
      gutter: 9pt,
      logo-badge("assets/logos/c.svg", [C], tone: rgb("#9ad0ff")),
      logo-badge("assets/logos/rust.svg", [Rust], tone: rgb("#d2d8e2")),
    )
  ],
)

#let wasm-visual() = grid(
  columns: (1fr, 0.9fr, 1fr),
  gutter: 14pt,
  flow-card([BROWSER], height: 4.25cm)[
    #text(size: 18pt, weight: "bold")[Browser]
    #v(0.18em)
    #text(size: 13pt, fill: text-muted)[WebAssembly direkt im Web]
    #v(0.5em)
    #pill([Chrome / Firefox / Safari], tone: rgb("#9ad0ff"), fill-color: rgb("#112030"))
  ],
  rect(
    fill: rgb("#243a5a"),
    stroke: 0.8pt + accent,
    radius: 16pt,
    inset: 14pt,
    width: 100%,
    height: 4.25cm,
  )[
    #align(center + horizon)[
      #text(size: 28pt, weight: "bold")[WASM]
      #v(0.14em)
      #text(size: 13pt, fill: text-muted)[ein gemeinsames Zielformat]
    ]
  ],
  flow-card([LAUFZEIT], height: 4.25cm)[
    #text(size: 18pt, weight: "bold")[Wasmtime]
    #v(0.18em)
    #text(size: 13pt, fill: text-muted)[auch außerhalb des Browsers]
    #v(0.5em)
    #pill([CLI / Tools / lokal], tone: rgb("#ffb454"), fill-color: rgb("#112030"))
  ],
)

#let project-sentence-visual() = box(width: 92%)[
  #grid(
    columns: (1fr, auto, 1fr, auto, 1fr, auto, 1.2fr),
    gutter: 10pt,
    align: center + horizon,
    pill([Eigener Code], tone: rgb("#9ad0ff")),
    text(fill: text-muted, size: 18pt)[→],
    pill([Compiler], tone: accent),
    text(fill: text-muted, size: 18pt)[→],
    pill([WASM], tone: rgb("#8ec07c")),
    text(fill: text-muted, size: 18pt)[→],
    pill([Wasmtime / Browser], tone: rgb("#ffb454")),
  )
]

#let tools-visual() = grid(
  columns: (1.2fr, 1fr),
  gutter: 12pt,
  flow-card([IMPLEMENTIERUNG], height: 7.25cm)[
    #align(center + horizon)[
      #box(fill: rgb("#f3f7fb"), inset: 12pt, radius: 999pt)[
        #image("assets/logos/rust.svg", height: 1.05cm)
      ]
      #v(0.22em)
      #text(size: 17pt, weight: "bold")[Rust]
      #v(0.12em)
      #text(size: 13pt, fill: text-muted)[kompiliert, performant, gutes WASM-Ökosystem]
    ]
  ],
  flow-card([WEITERE TOOLS], height: 7.25cm)[
    #grid(
      columns: (1fr, 1fr),
      gutter: 9pt,
      pill([wasm-encoder], tone: rgb("#9ad0ff"), fill-color: rgb("#112030")),
      pill([wasmtime], tone: rgb("#ffb454"), fill-color: rgb("#112030")),
      pill([git], tone: rgb("#8ec07c"), fill-color: rgb("#112030")),
      pill([typst], tone: rgb("#d9b3ff"), fill-color: rgb("#112030")),
    )
  ],
)

#let evaluation-visual() = grid(
  columns: (1fr, 1fr),
  gutter: 14pt,
  flow-card([WIRD LEICHTER], height: 4.95cm)[
    #stack(
      spacing: 8pt,
      compact-pill([ein gemeinsames Ziel], tone: rgb("#8ec07c"), fill-color: rgb("#112030")),
      compact-pill([weniger Plattformdetails], tone: rgb("#8ec07c"), fill-color: rgb("#112030")),
      compact-pill([Backend überschaubarer], tone: rgb("#8ec07c"), fill-color: rgb("#112030")),
    )
  ],
  flow-card([BLEIBT SCHWIERIG], height: 4.95cm)[
    #stack(
      spacing: 8pt,
      compact-pill([Lexer und Parser], tone: rgb("#ffb454"), fill-color: rgb("#112030")),
      compact-pill([Fehlerbehandlung], tone: rgb("#ffb454"), fill-color: rgb("#112030")),
      compact-pill([Sprachdesign und Logik], tone: rgb("#ffb454"), fill-color: rgb("#112030")),
    )
  ],
)

#let final-visual() = flow-card([MITNEHMEN], width: 82%, height: 3.8cm)[
  #align(center + horizon)[
    #text(size: 18pt, weight: "bold")[Eigene Sprache bauen]
    #v(0.2em)
    #text(size: 28pt, fill: accent)[Ja.]
    #v(0.24em)
    #text(size: 14pt, fill: text-muted)[Mit WebAssembly realistischer, aber nicht plötzlich einfach.]
  ]
]

#let ast-diagram() = box(
  width: 88%,
  height: 7.7cm,
  inset: 10pt,
  fill: panel,
  stroke: 0.6pt + rgb("#24344d"),
  radius: 10pt,
)[
  #align(center + horizon)[
    #set text(fill: text-main, size: 13pt)
    #diagram(
      cell-size: 12mm,
      spacing: 1.6em,
      node-fill: rgb("#27456d"),
      node-stroke: 1.4pt + accent,
      edge-stroke: 1.4pt + accent,
      node((0, 0), [#text(fill: text-main, weight: "bold")[print(...) ]], name: <call>, width: 3.2cm, height: 1.0cm, corner-radius: 8pt),
      node((0, 1), [#text(fill: text-main)[`+`]], name: <plus>, width: 2.1cm, height: 0.95cm, corner-radius: 8pt),
      node((-1.1, 2), [#text(fill: text-main)[`7`]], name: <left>, width: 1.6cm, height: 0.9cm, corner-radius: 8pt),
      node((1.1, 2), [#text(fill: text-main)[`5`]], name: <right>, width: 1.6cm, height: 0.9cm, corner-radius: 8pt),
      edge(<call>, <plus>, "->"),
      edge(<plus>, <left>, "->"),
      edge(<plus>, <right>, "->"),
    )
  ]
]

// Ziel der Folie:
// - ruhiger Start
// - Thema sofort klar
// - nicht zu viel Text
// Sprechpunkte:
// - Facharbeit als Vortrag jetzt deutlich alltagsnäher
// - nicht nur schriftliche Ausarbeitung wiederholen
// - Fokus heute: Idee, Bau des Projekts, Bewertung
// - Leitfrage wird Schritt für Schritt beantwortet
// Beispiele / mögliche Formulierungen:
// - ich zeige nicht die ganze Facharbeit, sondern den roten Faden
// - Ziel: auch ohne Vorwissen verstehen, was ich gebaut habe
// Kürzbar:
// - Unterzeile
#empty-slide[
  #meta[PRÄSENTATION]
  #v(0.24em)
  #text(size: 45pt, weight: "bold")[WebAssembly als Abkürzung zum eigenen Compiler?]
  #v(0.28em)
  #lead[Eigene Sprache, eigener Compiler, aber wie?]
  #v(0.74em)
  #title-visual()
  #v(0.56em)
  #text(size: 14pt, fill: text-muted)[Erik Tschöpe]
]

// Ziel der Folie:
// - Publikum aktiv reinholen
// - persönlicher Einstieg statt Fachbegriff-Einstieg
// - Gesprächsatmosphäre aufbauen
// Sprechpunkte:
// - Handzeichenfrage: wer hat schon mal programmiert
// - zweite Einordnung: textbasiert, nicht Scratch
// - egal wie viel Erfahrung: Problem ist meist ähnlich
// - man merkt schnell: Sprache bestimmt stark, wie angenehm Programmieren ist
// Beispiele / mögliche Formulierungen:
// - man kennt das: Programm macht nicht ganz das, was man im Kopf hatte
// - oft liegt das nicht nur an einem selbst, sondern auch an der Sprache
// Kürzbar:
// - zweite Nachfrage zu Scratch
#empty-slide[
  #meta[EINSTIEG]
  #v(0.1em)
  #text(size: 30pt, weight: "bold")[Gliederung]
  #v(0.22em)
  #rect(width: 100%, height: 0.08cm, radius: 999pt, fill: rgb("#1a2940"))[
    #grid(
      columns: (1fr, 1fr, 1fr, 1fr),
      gutter: 14pt,
      inset: 0pt,
    )[]
  ]
  #v(0.26em)
  #grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    gutter: 14pt,
    outline-card([01 Einstieg], ([Publikumsfrage], [Motivation], [Leitfrage])),
    outline-card([02 Grundlagen], ([Compiler], [Interpreter], [WebAssembly])),
    outline-card([03 Selbstversuch], ([Sprache], [Lexer bis WAT], [Ausführung])),
    outline-card([04 Bewertung], ([Werkzeuge], [Ergebnis], [Fazit])),
  )
]

= Einstieg <touying:hidden>

// Ziel der Folie:
// - Publikum einbinden
// - Problem alltagsnah aufmachen
// - noch keine Fachbegriffe
// Sprechpunkte:
// - Frage in den Raum: wer hat schon mal programmiert
// - noch konkreter: mit textbasierter Sprache
// - viele kennen das Gefühl trotzdem auch indirekt
// - Computer versteht nur sehr genaue Anweisungen
// - kleine Unterschiede in der Sprache machen großen Unterschied
// Beispiele / mögliche Formulierungen:
// - Nicht Scratch, sondern eher Python, Java, JavaScript, so etwas
// - heute reicht schon, wenn man das Grundgefühl kennt
// Kürzbar:
// - Beispiele einzelner Sprachen
== Wer von euch hat schon mal programmiert?

#bullet-list((
  [Handzeichen: schon mal programmiert?],
  [Mit Text, nicht nur mit Blöcken],
  [Sprachen fühlen sich sehr unterschiedlich an],
))

#v(0.34em)
#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 12pt,
  scratch-card(),
  syntax-card([PYTHON], rgb("#7cc7ff"))[
```python
name = input("Wie heißt du? ")
print(name)
```
  ],
  syntax-card([JAVA], rgb("#8ec07c"))[
```java
Scanner sc = new Scanner(System.in);
String name = sc.nextLine();
System.out.println(name);
```
  ],
)

// Ziel der Folie:
// - Frust an Syntax / Sprache benennen
// - direkt zur Kernfrage führen
// Sprechpunkte:
// - Syntax kann sich künstlich oder unpraktisch anfühlen
// - manchmal will man etwas Einfaches tun und kämpft mit der Sprache
// - daraus entsteht die große Frage
// - gibt es überhaupt die beste Sprache für alle
// - Übergang: wahrscheinlich nicht
// Beispiele / mögliche Formulierungen:
// - zu viele Sonderregeln
// - Sprache passt nicht zum eigenen Denken oder zum Anwendungsfall
// Kürzbar:
// - persönliches Beispiel
== Warum denkt man über eigene Sprachen nach?

#bullet-list((
  [Syntax kann nerven],
  [Sprachen passen nicht zu jedem Problem],
  [Man wünscht sich manchmal eine eigene Lösung],
))

#v(0.34em)
#grid(
  columns: (1fr, 1fr),
  gutter: 14pt,
  syntax-card([PYTHON], rgb("#7cc7ff"))[
```python
if score > 90:
    print("sehr gut")
```
  ],
  syntax-card([JAVA], rgb("#8ec07c"))[
```java
if (score > 90) {
    System.out.println("sehr gut");
}
```
  ],
)

// Ziel der Folie:
// - Leitfrage sauber setzen
// - Facharbeit klar verankern
// - späteren roten Faden aufbauen
// Sprechpunkte:
// - es gibt nicht die universell beste Sprache
// - unterschiedliche Aufgaben, unterschiedliche Vorlieben
// - logische Folge: eigene Sprache wäre eigentlich attraktiv
// - aber: eigene Sprache heißt meistens auch eigener Compiler
// - darum die Facharbeitsfrage mit WebAssembly
// Beispiele / mögliche Formulierungen:
// - nicht jede Sprache ist schlecht, sondern für etwas anderes gebaut
// - spannend wird es, wenn der technische Aufwand sinkt
// Kürzbar:
// - Teil mit persönlichen Präferenzen
== Die Leitfrage

#bullet-list((
  [Keine universell beste Programmiersprache],
  [Eigene Sprache klingt attraktiv],
  [Frage: macht WebAssembly den Bau realistischer?],
))

#v(0.34em)
#question-visual()

= Grundlagen <touying:hidden>

// Ziel der Folie:
// - direkt sagen, was gebaut wurde
// - Projekt verständlich und klein wirken lassen
// Sprechpunkte:
// - ich habe keine komplette neue Allzwecksprache gebaut
// - bewusst kleine, abgespeckte Variante
// - dazu einen Compiler
// - Ergebnis: eigener Code wird in WebAssembly übersetzt
// - damit ist das Projekt praktisch und überschaubar
// Beispiele / mögliche Formulierungen:
// - nicht wie Rust oder Java in voller Größe
// - eher Prototyp, aber funktionierend
// Kürzbar:
// - Betonung auf reduzierter Funktionsumfang
== Was habe ich gebaut?

#bullet-list((
  [Kleine eigene Programmiersprache],
  [Bewusst reduzierter Funktionsumfang],
  [Compiler erzeugt WebAssembly],
))

#v(0.38em)
#project-visual()

// Ziel der Folie:
// - Compilerbegriff einfach machen
// - keine Toolchain-Debatte
// Sprechpunkte:
// - Compiler ist ein Übersetzer für Code
// - vorne kommt Quellcode rein
// - hinten kommt eine ausführbare Form raus
// - das kann nativer Code sein oder etwas wie WebAssembly
// - wichtig ist nur: Code wird vorher umgewandelt
// Beispiele / mögliche Formulierungen:
// - wie ein Übersetzer zwischen zwei Sprachen
// - nur eben zwischen Programmiersprache und Maschinen-naher Form
// Kürzbar:
// - Hinweis auf unterschiedliche Zielarten
== Was ist ein Compiler?

#bullet-list((
  [Übersetzt Code in eine andere Form],
  [Aus Quelltext wird etwas Ausführbares],
  [Arbeitet vor der eigentlichen Ausführung],
))

#v(0.36em)
#compiler-flow()

// Ziel der Folie:
// - Unterschied zu Interpreter einfach und merkbar erklären
// - alltagsnahe Beispiele einbauen
// Sprechpunkte:
// - Interpreter führt Code eher direkt Schritt für Schritt aus
// - Compiler übersetzt zuerst
// - das ist stark vereinfacht, reicht hier aber
// - bekannte Beispiele helfen
// - nicht zu tief, nur Grundidee
// Beispiele / mögliche Formulierungen:
// - Python und JavaScript eher interpretiert wahrgenommen
// - C und Rust eher kompiliert
// Kürzbar:
// - Einordnung mit "eher", nicht als absolute Lehrbuchtrennung
== Compiler oder Interpreter?

#bullet-list((
  [Interpreter: führt Code direkt aus],
  [Compiler: übersetzt zuerst],
  [Beispiele: Python/JS vs. C/Rust],
))

#v(0.38em)
#compiler-interpreter-visual()

// Ziel der Folie:
// - WebAssembly einführen ohne Fachjargon
// - Rolle als Zielplattform betonen
// Sprechpunkte:
// - WebAssembly ist ein standardisiertes Format
// - ursprünglich stark mit Browsern verbunden
// - inzwischen auch außerhalb nutzbar
// - für mein Projekt wichtig: gemeinsames Ziel statt eigener CPU-Ausgabe
// - damit wird das Backend einfacher
// Beispiele / mögliche Formulierungen:
// - Browser können das ausführen
// - aber auch eine Laufzeit wie Wasmtime
// Kürzbar:
// - Geschichte von WebAssembly
== Was ist WebAssembly?

#bullet-list((
  [Standardisiertes Ausführungsformat],
  [Läuft im Browser und auch außerhalb],
  [Praktisch als gemeinsames Ziel für Compiler],
))

#v(0.38em)
#wasm-visual()

// Ziel der Folie:
// - Gesamtidee des Projekts auf einen Satz verdichten
// - Brücke zum Selbstversuch
// Sprechpunkte:
// - jetzt alles noch einmal extrem einfach
// - ich habe ein Programm geschrieben, das meinen Code nimmt
// - und daraus WebAssembly macht
// - danach kann dieser Code ausgeführt werden
// - das ist der Kern des ganzen Projekts
// Beispiele / mögliche Formulierungen:
// - eigener Code rein, WebAssembly raus
// - danach startet eine Laufzeit das Ergebnis
// Kürzbar:
// - letzter Satz
== Mein Projekt in einem Satz

#bullet-list((
  [Eigener Code],
  [Compiler],
  [WebAssembly],
  [Ausführung],
))

#v(0.5em)
#project-sentence-visual()

= Selbstversuch <touying:hidden>

// Ziel der Folie:
// - Sprachumfang klar und knapp machen
// - bewusst kleine Sprache nicht als Schwäche verkaufen
// Sprechpunkte:
// - Sprache ist absichtlich klein gehalten
// - nur ein Datentyp: Int
// - Syntax lehnt sich an Rust an
// - Funktionen, Variablen, if, while reichen für die Pipeline
// - Ziel war Nachvollziehbarkeit, nicht Vollständigkeit
// Beispiele / mögliche Formulierungen:
// - klein genug zum Bauen, groß genug zum Zeigen
// Kürzbar:
// - Rust-Nähe
== Sprachdesign

#bullet-list((
  [Nur ein Datentyp: `Int`],
  [Syntax grob an Rust angelehnt],
  [Funktionen, Variablen, Bedingungen, Schleifen],
))

#v(0.34em)
#feature-table()

// Ziel der Folie:
// - ein Beispiel setzen, das danach wieder aufgegriffen wird
// - möglichst wenig, aber lesbarer Code
// Sprechpunkte:
// - dieses kleine Beispiel kann man gut durch alle Schritte verfolgen
// - addieren, ausgeben, fertig
// - wichtig: nicht vom Code erschlagen, nur Grundidee sehen
// - danach immer fragen: was wird aus diesem Beispiel im nächsten Schritt
// Beispiele / mögliche Formulierungen:
// - das ist mein Ausgangsmaterial für die Pipeline
// Kürzbar:
// - Erklärung einzelner Syntaxteile
== Beispielprogramm

#bullet-list((
  [Kleines Beispiel als roter Faden],
  [Wir kompilieren es gleich im Kopf durch],
  [Von Syntax zu Tokens, AST und WebAssembly],
))

#v(0.46em)
#code-example(width: 66%)[
```rust
// Startpunkt des Programms
fn main() {
    // Ausdruck ausrechnen und ausgeben
    print(7 + 5);
}
```
]
#v(0.22em)

// Ziel der Folie:
// - ersten Verarbeitungsschritt sehr einfach erklären
// - "Text wird sortiert" statt abstrakter Theorie
// Sprechpunkte:
// - Lexer schaut erst einmal nur auf Zeichen
// - daraus werden sinnvolle Stücke
// - zum Beispiel Schlüsselwörter, Namen, Zahlen, Operatoren
// - noch kein tiefes Verständnis, eher Sortieren
// - aber unverzichtbar für alles Weitere
// Beispiele / mögliche Formulierungen:
// - aus `print(7 + 5)` werden einzelne Bausteine
// - ähnlich wie Wörter und Satzzeichen auseinandernehmen
// Kürzbar:
// - Aufzählung einzelner Token-Arten
== Lexer

#bullet-list((
  [Aus Text wird eine Liste von Tokens],
  [Noch keine Bedeutung, nur Bausteine],
  [Beispiel: Name, Klammern, Zahlen, Plus],
))

#v(0.46em)
#code-example(width: 72%)[
```text
// Noch keine Struktur, nur Einzelteile
[Ident("print"), LParen, Int(7),
 Plus, Int(5), RParen]
```
]
#v(0.22em)

// Ziel der Folie:
// - aus Token eine Struktur machen
// - AST erklären, ohne den Begriff zu überladen
// Sprechpunkte:
// - Parser nimmt diese Bausteine und ordnet sie sinnvoll
// - jetzt wird erkannt, was zusammengehört
// - daraus entsteht eine Baumstruktur
// - wichtig: Programm wird jetzt nicht mehr nur gelesen, sondern verstanden
// - Begriff AST einmal nennen, dann einfach bleiben
// Beispiele / mögliche Formulierungen:
// - `7 + 5` ist ein Ausdruck
// - `print(...)` ist ein Funktionsaufruf
// Kürzbar:
// - Fachbegriff AST
== Parser

#bullet-list((
  [Aus Tokens wird Struktur],
  [Jetzt erkennt der Compiler Zusammenhänge],
  [Ergebnis: ein echter Baum],
))

#v(0.3em)
#ast-diagram()

// Ziel der Folie:
// - Zwischenschritt IR verständlich machen
// - nicht technisch, sondern organisatorisch erklären
// Sprechpunkte:
// - aus der Baumstruktur geht es noch nicht direkt nach WebAssembly
// - dazwischen steht eine interne Zwischenform
// - Grund: Übersetzung wird übersichtlicher
// - man trennt Sprachlogik und Zielausgabe etwas sauberer
// - das ist in Compilern ein üblicher Gedanke
// Beispiele / mögliche Formulierungen:
// - wie ein Zwischennotizzettel für den Compiler
// Kürzbar:
// - Hinweis auf große Compiler
== Codegen Teil 1

#bullet-list((
  [Baum wird in einfache Arbeitsschritte zerlegt],
  [Zuerst 7, dann 5, dann addieren],
  [Danach Ergebnis an `print` geben],
))

#v(0.46em)
#code-example(width: 62%)[
```text
// Interne Schrittfolge für den Ausdruck
push 7
push 5
add
call print
```
]
#v(0.22em)

// Ziel der Folie:
// - eigentlichen WebAssembly-Schritt klar machen
// - Nutzen für Leitfrage direkt betonen
// Sprechpunkte:
// - jetzt wird aus der Zwischenform wirklich WebAssembly
// - genau hier hilft das Zielformat
// - ich muss keinen nativen Code für konkrete Prozessoren schreiben
// - stattdessen ein einheitliches, standardisiertes Ziel
// - das senkt den Aufwand vor allem im Backend
// Beispiele / mögliche Formulierungen:
// - nicht Windows hier, Linux dort, CPU hier, CPU dort
// Kürzbar:
// - Begriff Backend
== Codegen Teil 2

#bullet-list((
  [Diese Schritte werden zu WebAssembly],
  [Ein Ziel statt vieler Plattformen],
  [Darum wird das Backend einfacher],
))

#v(0.46em)
#code-example(width: 66%)[
```wat
;; Das Ergebnis der kleinen Pipeline
(func (export "main")
  i64.const 7
  i64.const 5
  i64.add
  call 0
)
```
]

// Ziel der Folie:
// - zeigen, dass es nicht nur Theorie blieb
// - Wasmtime knapp und verständlich einordnen
// Sprechpunkte:
// - erzeugter Code wurde wirklich ausgeführt
// - dafür habe ich Wasmtime verwendet
// - Wasmtime ist eine Laufzeit für WebAssembly
// - über eine Host-Funktion konnte mein Programm auch etwas ausgeben
// - theoretisch wäre Browser-Ausführung ebenfalls möglich
// Beispiele / mögliche Formulierungen:
// - also nicht nur "übersetzt", sondern tatsächlich gestartet
// Kürzbar:
// - Host-Funktion technisch erklären
== Ausführung mit Wasmtime

#bullet-list((
  [WebAssembly wird wirklich ausgeführt],
  [Wasmtime übernimmt die Laufzeit],
  [Theoretisch auch im Browser möglich],
))

#v(0.3em)
#code-example(width: 58%)[
```bash
cargo run -- add.eres
Ausgabe: 12
```
]

= Bewertung <touying:hidden>

// Ziel der Folie:
// - Werkzeugentscheidung begründen
// - Rust nicht zu groß machen, aber sinnvoll verankern
// Sprechpunkte:
// - warum gerade Rust
// - persönliche Stärke: Sprache liegt mir
// - technisch: performant, kompiliert, flexibel
// - wichtig für dieses Projekt: starkes WebAssembly-Ökosystem
// - das passt gut zur Leitfrage, weil Werkzeugwahl den Selbstversuch erleichtert
// Beispiele / mögliche Formulierungen:
// - mit Rust kommt man vom Systemnahen bis zu WebAssembly
// - viele Tools rund um WASM sind dort stark
// Kürzbar:
// - persönlicher Teil
== Entscheidungen und Werkzeuge

#bullet-list((
  [Implementiert in Rust],
  [Gut geeignet für kompilierten Code],
  [Starkes WebAssembly-Ökosystem],
))

#v(0.38em)
#tools-visual()

// Ziel der Folie:
// - Leitfrage differenziert beantworten
// - weder übertreiben noch kleinreden
// Sprechpunkte:
// - WebAssembly nimmt viel Plattformarbeit ab
// - man muss nicht jedes Zielsystem selbst bedienen
// - dadurch wird der Backend-Teil realistischer
// - aber die Denkarbeit verschwindet nicht
// - Lexer, Parser, Fehlerbehandlung und Sprachdesign bleiben anspruchsvoll
// Beispiele / mögliche Formulierungen:
// - einfacher heißt nicht einfach
// - realistischer für Einzelpersonen, aber kein Wochenendprojekt
// Kürzbar:
// - Beispiele einzelner schwerer Stellen
== Bewertung der Leitfrage

#bullet-list((
  [WebAssembly senkt vor allem die Backend-Hürde],
  [Frontend und Sprachlogik bleiben schwierig],
  [Also: realistischer, aber nicht trivial],
))

#v(0.38em)
#evaluation-visual()

// Ziel der Folie:
// - sauber abschließen
// - Raum für Fragen lassen
// Sprechpunkte:
// - ja, man kann als Einzelperson so ein Projekt bauen
// - mein Projekt ist klein, aber funktionsfähig
// - WebAssembly war dabei ein echter Vorteil
// - gleichzeitig sieht man, wo die Grenzen bleiben
// - dann offen in Fragen übergehen
// Beispiele / mögliche Formulierungen:
// - eigene Sprache bauen: ja
// - perfekte Allzwecksprache bauen: natürlich nicht
// Kürzbar:
// - letzter Vergleich
== Fazit und Fragen

#bullet-list((
  [Eigene Sprache bauen: ja],
  [Mit WebAssembly deutlich realistischer],
  [Aber weiterhin technisch anspruchsvoll],
  [Fragen?],
))

#v(0.42em)
#final-visual()
