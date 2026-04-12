#import "@preview/touying:0.7.1": *
#import themes.simple: *

#let bg = rgb("#0b1220")
#let bg-soft = rgb("#111c2f")
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

#set page(
  fill: bg,
  margin: (x: 1.3cm, y: 1.0cm),
)

#set text(
  font: "Noto Sans CJK JP",
  fill: text-main,
  size: 22pt,
)

#set par(justify: false)

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
]

#show strong: set text(fill: accent)

#let muted(body) = text(fill: text-muted, size: 13pt)[#body]

#slide[
  #v(0.38fr)
  #text(size: 16pt, fill: accent, tracking: 0.09em, weight: "medium")[PRÄSENTATION]
  #v(0.45em)
  #text(size: 34pt, weight: "bold")[WebAssembly als Abkürzung zum eigenen Compiler?]
  #v(0.5em)
  #text(size: 17pt, fill: text-muted)[Präsentation zur Facharbeit]
  #v(1.3em)
  #text(size: 14pt, fill: text-muted)[Erik Tschöpe]
]

#slide[
  #v(0.45fr)
  #text(size: 14pt, fill: accent, tracking: 0.08em, weight: "medium")[STRUKTUR]
  #v(0.35em)
  #text(size: 28pt, weight: "bold")[Gliederung]
  #v(1.1em)
  #muted[
    #outline(title: none, indent: 1.4em)
  ]
]

= Einstieg

== Warum überhaupt ein eigener Compiler?

== Die Leitfrage

= Grundlagen

== Was macht ein Compiler?

== Frontend und Backend

== Warum WebAssembly?

= Selbstversuch

== Ziel des Projekts

== Eigene Sprache im Überblick

== Lexer

== Parser

== Codegen nach WebAssembly

== Ausführung mit Wasmtime

= Bewertung

== Was WebAssembly vereinfacht

== Wo die Komplexität bleibt

== Fazit

= Abschluss

== Fragen?
