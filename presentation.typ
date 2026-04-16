#import "@preview/touying:0.7.1": *
#import themes.simple: *
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

#let bg = rgb("#0b1220")
#let panel = rgb("#162338")
#let panel-soft = rgb("#132033")
#let text-main = rgb("#e6edf7")
#let text-muted = rgb("#9fb1c9")
#let accent = rgb("#6ea8ff")
#let warm = rgb("#ffb454")
#let green = rgb("#8ec07c")

// Layout-Tuning:
// - Boxbreite: width: ...
// - Boxhöhe: height: ...
// - Innenabstand: inset: ...
// - Abstand zwischen Boxen: gutter: ... / spacing: ...

#show: simple-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: [Präsentationstitel],
    subtitle: [Präsentation zur Facharbeit],
    author: [Name],
    date: [Datum],
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

#let bullet-list(items, width: 90%, spacing: 0.56em) = {
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
  box(width: width)[
    #stack(spacing: spacing, ..rendered)
  ]
}

#let card(title, body, width: 100%, height: auto, tone: accent, inset: 12pt) = rect(
  fill: panel,
  stroke: 0.8pt + rgb("#253754"),
  radius: 14pt,
  inset: inset,
  width: width,
  height: height,
)[
  #text(size: 12pt, fill: tone, weight: "bold", tracking: 0.05em)[#title]
  #v(0.22em)
  #body
]

#let outline-card(title, lines, height: 4.8cm) = card(title, height: height)[
  #for line in lines [
    #text(size: 13.2pt)[#line]
    #linebreak()
  ]
]

#let pill(label, tone: accent, fill-color: panel-soft) = rect(
  fill: fill-color,
  stroke: 0.8pt + rgb("#29405f"),
  radius: 999pt,
  inset: (x: 15pt, y: 9pt),
)[
  #text(size: 14pt, fill: tone, weight: "bold")[#label]
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

#let simple-flow(labels, width: 92%) = box(width: width)[
  #grid(
    columns: (1fr, auto, 1fr, auto, 1fr),
    gutter: 12pt,
    align: center + horizon,
    card([SCHRITT 1], height: 2.7cm)[
      #align(center + horizon)[#text(size: 18pt, weight: "bold")[#labels.at(0)]]
    ],
    text(fill: text-muted, size: 20pt)[→],
    card([SCHRITT 2], height: 2.7cm, tone: warm)[
      #align(center + horizon)[#text(size: 18pt, weight: "bold")[#labels.at(1)]]
    ],
    text(fill: text-muted, size: 20pt)[→],
    card([SCHRITT 3], height: 2.7cm, tone: green)[
      #align(center + horizon)[#text(size: 18pt, weight: "bold")[#labels.at(2)]]
    ],
  )
]

#let comparison-table(left-title, right-title, rows) = box(width: 92%)[
  #set text(size: 15pt)
  #table(
    columns: (1fr, 1fr),
    stroke: none,
    inset: 8pt,
    fill: (x, y) => if y == 0 { rgb("#20314a") } else if calc.odd(y) { panel } else { panel-soft },
    align: left,
    [*#left-title*], [*#right-title*],
    ..rows,
  )
]

#let diagram-placeholder() = box(
  width: 86%,
  height: 5.3cm,
  inset: 10pt,
  fill: panel,
  stroke: 0.6pt + rgb("#24344d"),
  radius: 10pt,
)[
  #align(center + horizon)[
    #set text(fill: text-main, size: 13pt)
    #diagram(
      cell-size: 13mm,
      spacing: 1.6em,
      node-fill: rgb("#27456d"),
      node-stroke: 1.4pt + accent,
      edge-stroke: 1.4pt + accent,
      node((0, 0), [Hauptidee], name: <a>, width: 3.0cm, height: 1.0cm, corner-radius: 8pt),
      node((-1.4, 1.4), [Aspekt A], name: <b>, width: 2.4cm, height: 0.95cm, corner-radius: 8pt),
      node((1.4, 1.4), [Aspekt B], name: <c>, width: 2.4cm, height: 0.95cm, corner-radius: 8pt),
      edge(<a>, <b>, "->"),
      edge(<a>, <c>, "->"),
    )
  ]
]

// Ziel der Folie:
// - ruhiger Start
// - Thema und Person ersetzen, sobald die Facharbeit vorliegt
// Sprechpunkte:
// - hier später Einstiegssatz und Kontext ergänzen
// - keine Detailflut auf der Titelfolie
#empty-slide[
  #meta[PRÄSENTATION]
  #v(0.24em)
  #text(size: 45pt, weight: "bold")[Präsentationstitel]
  #v(0.28em)
  #lead[Kurzer Untertitel oder zentrale Leitfrage.]
  #v(0.74em)
  #simple-flow(([Ausgangspunkt], [Kernidee], [Fazit]))
  #v(0.56em)
  #text(size: 14pt, fill: text-muted)[Name]
]

// Ziel der Folie:
// - grobe Struktur setzen
// - noch keine inhaltlichen Details
// Sprechpunkte:
// - die vier Blöcke später passend zur Facharbeit benennen
// - Reihenfolge soll den Vortrag führen, nicht die Facharbeit kopieren
#empty-slide[
  #meta[STRUKTUR]
  #v(0.1em)
  #text(size: 30pt, weight: "bold")[Gliederung]
  #v(0.22em)
  #rect(width: 100%, height: 0.08cm, radius: 999pt, fill: rgb("#1a2940"))
  #v(0.26em)
  #grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    gutter: 14pt,
    outline-card([01 Einstieg], ([Kontext], [Problem], [Leitfrage])),
    outline-card([02 Grundlagen], ([Begriff 1], [Begriff 2], [Einordnung])),
    outline-card([03 Hauptteil], ([Methode], [Ergebnisse], [Beispiel])),
    outline-card([04 Bewertung], ([Einordnung], [Fazit], [Fragen])),
  )
]

= Einstieg <touying:hidden>

// Ziel der Folie:
// - Publikum abholen
// - Thema ohne Vorwissen öffnen
// Sprechpunkte:
// - kurze Frage oder Beobachtung aus dem Alltag
// - an vorhandenes Wissen anschließen
== Einstieg

#bullet-list((
  [Publikumsfrage oder Alltagsbezug],
  [Warum ist das Thema relevant?],
  [Was soll der Vortrag klären?],
))

#v(0.38em)
#simple-flow(([Beobachtung], [Problem], [Leitfrage]))

// Ziel der Folie:
// - Leitfrage klar formulieren
// - Arbeitsziel verständlich machen
// Sprechpunkte:
// - Leitfrage wörtlich oder sinngemäß nennen
// - Abgrenzung: was wird nicht untersucht?
== Leitfrage

#bullet-list((
  [Zentrale Frage der Facharbeit],
  [Warum diese Frage sinnvoll ist],
  [Was am Ende bewertet wird],
))

#v(0.38em)
#card([LEITFRAGE], width: 86%, height: 3.1cm, tone: warm)[
  #align(center + horizon)[
    #text(size: 22pt, weight: "bold")[Hier später die konkrete Leitfrage einsetzen.]
  ]
]

= Grundlagen <touying:hidden>

// Ziel der Folie:
// - wichtigste Begriffe einfach erklären
// - nicht mit Theorie überladen
// Sprechpunkte:
// - nur Begriffe erklären, die das Publikum wirklich braucht
// - Beispiele statt Definitionen bevorzugen
== Grundlagen

#bullet-list((
  [Begriff oder Konzept 1],
  [Begriff oder Konzept 2],
  [Warum diese Grundlagen für den Hauptteil wichtig sind],
))

#v(0.38em)
#diagram-placeholder()

// Ziel der Folie:
// - einen Vergleich sichtbar machen
// - Gemeinsamkeiten und Unterschiede bündeln
// Sprechpunkte:
// - Tabelle nicht vorlesen
// - nur die wichtigste Zeile hervorheben
== Vergleich

#bullet-list((
  [Zwei Perspektiven gegenüberstellen],
  [Unterschiede sichtbar machen],
  [Zum Hauptteil überleiten],
))

#v(0.34em)
#comparison-table(
  [Perspektive A],
  [Perspektive B],
  (
    [Merkmal 1], [Einordnung],
    [Merkmal 2], [Einordnung],
    [Merkmal 3], [Einordnung],
  ),
)

= Hauptteil <touying:hidden>

// Ziel der Folie:
// - Vorgehen der Facharbeit erklären
// - roter Faden statt Detailmenge
// Sprechpunkte:
// - wie wurde gearbeitet?
// - welche Quellen, Methode, Analyse oder Versuch?
== Vorgehen

#bullet-list((
  [Wie wurde untersucht?],
  [Welche Materialien oder Quellen wurden genutzt?],
  [Welche Zwischenschritte gab es?],
))

#v(0.38em)
#simple-flow(([Material], [Analyse], [Ergebnis]))

// Ziel der Folie:
// - ein konkretes Beispiel zeigen
// - Vortrag anschaulich machen
// Sprechpunkte:
// - Beispiel kurz einführen
// - dann an diesem Beispiel erklären
== Beispiel

#bullet-list((
  [Konkretes Beispiel aus der Facharbeit],
  [Was zeigt dieses Beispiel?],
  [Warum ist es für die Leitfrage wichtig?],
))

#v(0.46em)
#code-example(width: 66%)[
```text
// Platzhalter für Beispiel, Zitat, Daten oder Ablauf
Schritt 1 -> Schritt 2 -> Ergebnis
```
]

// Ziel der Folie:
// - zentrale Ergebnisse knapp zeigen
// - nicht alles aus der Facharbeit wiederholen
// Sprechpunkte:
// - 2 bis 3 wichtigste Ergebnisse auswählen
// - erklären, warum sie relevant sind
== Ergebnisse

#bullet-list((
  [Ergebnis 1],
  [Ergebnis 2],
  [Ergebnis 3],
))

#v(0.38em)
#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 14pt,
  card([ERGEBNIS 1], height: 3.4cm, tone: green)[#text(size: 16pt)[Kurze Aussage]],
  card([ERGEBNIS 2], height: 3.4cm, tone: warm)[#text(size: 16pt)[Kurze Aussage]],
  card([ERGEBNIS 3], height: 3.4cm)[#text(size: 16pt)[Kurze Aussage]],
)

= Bewertung <touying:hidden>

// Ziel der Folie:
// - Ergebnisse einordnen
// - Stärken und Grenzen klar trennen
// Sprechpunkte:
// - Was beantwortet die Facharbeit gut?
// - Wo bleiben Unsicherheiten oder Grenzen?
== Bewertung

#bullet-list((
  [Was spricht für die Antwort?],
  [Was schränkt die Antwort ein?],
  [Wie fällt die Bewertung insgesamt aus?],
))

#v(0.38em)
#grid(
  columns: (1fr, 1fr),
  gutter: 14pt,
  card([STÄRKEN], height: 4.4cm, tone: green)[
    #stack(
      spacing: 8pt,
      pill([Punkt 1], tone: green),
      pill([Punkt 2], tone: green),
      pill([Punkt 3], tone: green),
    )
  ],
  card([GRENZEN], height: 4.4cm, tone: warm)[
    #stack(
      spacing: 8pt,
      pill([Punkt 1], tone: warm),
      pill([Punkt 2], tone: warm),
      pill([Punkt 3], tone: warm),
    )
  ],
)

// Ziel der Folie:
// - klarer Abschluss
// - Fragen ermöglichen
// Sprechpunkte:
// - Leitfrage beantworten
// - einen Schlusssatz formulieren
// - dann bewusst Pause für Fragen
== Fazit und Fragen

#bullet-list((
  [Antwort auf die Leitfrage],
  [Wichtigste Erkenntnis],
  [Offene Fragen oder Ausblick],
  [Fragen?],
))

#v(0.42em)
#card([MITNEHMEN], width: 82%, height: 3.8cm, tone: accent)[
  #align(center + horizon)[
    #text(size: 28pt, fill: accent, weight: "bold")[Kernaussage]
    #v(0.24em)
    #text(size: 14pt, fill: text-muted)[Ein prägnanter letzter Satz.]
  ]
]
