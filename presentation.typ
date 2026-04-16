#import "@preview/touying:0.7.1": *
#import themes.simple: *
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

#let bg = rgb("#081109")
#let panel = rgb("#102116")
#let panel-soft = rgb("#162b1c")
#let panel-deep = rgb("#0c1a11")
#let text-main = rgb("#fbf4cc")
#let text-muted = rgb("#c5d6b3")
#let accent = rgb("#79ae6f")
#let accent-strong = rgb("#9fcb98")
#let warning = rgb("#d9b86f")
#let risk = rgb("#d9836f")

#show: simple-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: [Cannabiskonsum im Jugendalter],
    subtitle: [Präsentation zur Facharbeit],
    author: [Mattis Becker],
    date: [2026],
  ),
)

#set page(
  fill: bg,
  margin: (left: 0.82cm, right: 0.82cm, top: 0.96cm, bottom: 0.36cm),
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
  #text(size: 14pt, fill: text-muted, tracking: 0.08em, weight: "medium")[BLOCK]
  #v(0.16em)
  #text(font: "Times New Roman", size: 34pt, weight: "bold", fill: text-main)[#it.body]
]

#show heading.where(level: 2): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #text(size: 14pt, fill: text-muted)[#utils.display-current-heading(level: 1)]
  #v(0.08em)
  #text(font: "Times New Roman", size: 31pt, weight: "bold", fill: text-main)[#it.body]
  #v(0.18em)
  #rect(width: 1.05cm, height: 0.08cm, radius: 999pt, fill: accent)
  #v(0.28em)
]

#let meta(body) = text(size: 14pt, fill: text-muted, tracking: 0.08em, weight: "medium")[#body]
#let lead(body) = text(size: 19pt, fill: text-muted)[#body]
#let icon(name, size: 1.05cm) = box(width: size, height: size)[
  #image("assets/icons/" + name + ".svg", width: size)
]

#let header-icon(name, size: 1.55cm) = place(top + right, dx: -0.25cm, dy: 0.02cm)[
  #icon(name, size: size)
]

#let bullet-list(items, width: 91%, spacing: 0.9em) = {
  let rendered = ()
  for item in items {
    rendered.push([
      #grid(
        columns: (0.45cm, 1fr),
        gutter: 0.34cm,
        align(center + horizon)[#text(fill: accent-strong, size: 18pt)[•]],
        text(size: 24pt, fill: text-main)[#item],
      )
    ])
  }
  box(width: width)[
    #stack(spacing: spacing, ..rendered)
  ]
}

// title-gap steuert den Abstand zwischen Box-Label und Inhalt.
#let card(title, body, width: 100%, height: auto, tone: accent, fill-color: panel, inset: 12pt, title-gap: 0em) = rect(
  fill: fill-color,
  stroke: 0.85pt + rgb("#25482f"),
  radius: 16pt,
  inset: inset,
  width: width,
  height: height,
)[
  #text(size: 12pt, fill: tone, weight: "bold", tracking: 0.06em)[#title]
  #v(title-gap)
  #body
]

#let icon-note(name, label, body, tone: accent-strong, width: 100%, height: 3.8cm) = card(label, width: width, height: height, tone: tone)[
  #grid(
    columns: (1fr, auto),
    gutter: 14pt,
    align: horizon,
    body,
    icon(name, size: 1.45cm),
  )
]

#let outline-card(title, lines, height: 4.95cm) = card(title, height: height)[
  #for line in lines [
    #text(size: 13.2pt)[#line]
    #linebreak()
  ]
]

#let pill(label, tone: accent-strong, fill-color: panel-soft) = rect(
  fill: fill-color,
  stroke: 0.75pt + rgb("#2f5838"),
  radius: 999pt,
  inset: (x: 14pt, y: 8pt),
)[
  #text(size: 14pt, fill: tone, weight: "bold")[#label]
]

#let title-flow(items) = box(width: 92%)[
  #grid(
    columns: (1fr, auto, 1fr, auto, 1fr, auto, 1fr),
    gutter: 10pt,
    align: center + horizon,
    pill(items.at(0), tone: accent-strong),
    text(fill: text-muted, size: 18pt)[→],
    pill(items.at(1), tone: warning),
    text(fill: text-muted, size: 18pt)[→],
    pill(items.at(2), tone: accent),
    text(fill: text-muted, size: 18pt)[→],
    pill(items.at(3), tone: risk),
  )
]

// triad(..., title-gap: 0.2em) setzt den Abstand global.
// Optional kann jedes Item als ([LABEL], [Text], farbe, 0.1em) einen eigenen Abstand setzen.
#let triad(a, b, c, title-gap: 0em) = {
  let gap(item) = if item.len() > 3 { item.at(3) } else { title-gap }

  grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 13pt,
    card(a.at(0), height: 3.6cm, tone: a.at(2), title-gap: gap(a))[
      #text(size: 17pt, weight: "bold")[#a.at(1)]
    ],
    card(b.at(0), height: 3.6cm, tone: b.at(2), title-gap: gap(b))[
      #text(size: 17pt, weight: "bold")[#b.at(1)]
    ],
    card(c.at(0), height: 3.6cm, tone: c.at(2), title-gap: gap(c))[
      #text(size: 17pt, weight: "bold")[#c.at(1)]
    ],
  )
}

#let process-diagram() = box(
  width: 90%,
  height: 6.5cm,
  inset: 10pt,
  fill: panel,
  stroke: 0.75pt + rgb("#25482f"),
  radius: 16pt,
)[
  #align(center + horizon)[
    #set text(fill: text-main, size: 13pt)
    #diagram(
      cell-size: 13mm,
      spacing: 1.7em,
      node-fill: rgb("#1e3b25"),
      node-stroke: 1.35pt + accent,
      edge-stroke: 1.35pt + accent,
      node((0, 0), [THC], name: <thc>, width: 1.9cm, height: 0.9cm, corner-radius: 8pt),
      node((1.7, 0), [CB1-Rezeptor], name: <cb1>, width: 3.0cm, height: 1.9cm, corner-radius: 8pt),
      node((3.8, 0), [Signal wird gebremst], name: <signal>, width: 3.4cm, height: 1.7cm, corner-radius: 8pt),
      node((2.0, 1.55), [Gedächtnis], name: <mem>, width: 2.7cm, height: 0.85cm, corner-radius: 8pt),
      node((3.7, 1.55), [Aufmerksamkeit], name: <focus>, width: 4.1cm, height: 0.85cm, corner-radius: 8pt),
      node((5.7, 1.55), [Koordination], name: <motor>, width: 3.2cm, height: 0.85cm, corner-radius: 8pt),
      edge(<thc>, <cb1>, "->"),
      edge(<cb1>, <signal>, "->"),
      edge(<signal>, <mem>, "->"),
      edge(<signal>, <focus>, "->"),
      edge(<signal>, <motor>, "->"),
    )
  ]
]

#let brain-window() = box(width: 88%)[
  #grid(
    columns: (1fr, 1fr),
    gutter: 14pt,
    card([PRÄFRONTALER CORTEX], height: 4.85cm, tone: accent-strong)[
      #text(size: 17pt, weight: "bold")[Entscheiden, planen, Impulse kontrollieren]
      #v(0.25em)
      #text(size: 13pt, fill: text-muted)[Reift bis in die Mitte der zwanziger Jahre.]
    ],
    card([HIPPOCAMPUS], height: 4.85cm, tone: warning)[
      #text(size: 17pt, weight: "bold")[Lernen und Gedächtnis]
      #v(0.25em)
      #text(size: 13pt, fill: text-muted)[Viele CB1-Rezeptoren, deshalb besonders relevant.]
    ],
  )
]

#let compare-table() = box(width: 92%)[
  #set text(size: 15pt)
  #table(
    columns: (1fr, 1fr),
    stroke: none,
    inset: 8pt,
    fill: (x, y) => if y == 0 { rgb("#203b25") } else if calc.odd(y) { panel } else { panel-soft },
    align: left,
    text(fill: accent-strong, weight: "bold")[Studienlage],
    text(fill: accent-strong, weight: "bold")[Erfahrungsberichte],
    [erhöhte Risiken], [keine schweren Folgen berichtet],
    [große Stichproben], [nur drei Personen],
    [statistische Zusammenhänge], [individuelle Erfahrung],
  )
]

#let method-map() = box(width: 88%)[
  #grid(
    columns: (1fr, auto, 1fr, auto, 1fr),
    gutter: 12pt,
    align: center + horizon,
    card([GRUNDLAGEN], height: 4.75cm, tone: accent-strong)[
      #text(size: 16.2pt, weight: "bold")[Wie wirkt Cannabis im Körper?]
    ],
    text(fill: text-muted, size: 20pt)[+],
    card([STUDIEN], height: 4.75cm, tone: warning)[
      #text(size: 16.2pt, weight: "bold")[Welche Risiken zeigen größere Untersuchungen?]
    ],
    text(fill: text-muted, size: 20pt)[+],
    card([BERICHTE], height: 4.75cm, tone: accent)[
      #text(size: 16.2pt, weight: "bold")[Wie erleben einzelne Personen ihren Konsum?]
    ],
  )
]

#let evidence-scale() = box(width: 88%)[
  #grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 13pt,
    card([STUDIEN], height: 3.6cm, tone: accent-strong)[
      #text(size: 17pt, weight: "bold")[zeigen Muster in Gruppen]
    ],
    card([ERFAHRUNG], height: 3.6cm, tone: warning)[
      #text(size: 17pt, weight: "bold")[zeigt einen einzelnen Verlauf]
    ],
    card([FAZIT], height: 3.6cm, tone: risk)[
      #text(size: 17pt, weight: "bold")[muss beides sauber trennen]
    ],
  )
]

#let quote-card(body, source: []) = card([ERFAHRUNGSBERICHT], width: 84%, height: 3.8cm, tone: warning)[
  #v(0.9em)
  #align(center + horizon)[
    #text(font: "Times New Roman", size: 24pt, fill: text-main)[„#body“]
    #v(0.24em)
    #text(size: 13pt, fill: text-muted)[#source]
  ]
]

// Ziel der Folie:
// - ruhiger Start
// - Thema klar, aber nicht dramatisierend setzen
// - direkt zeigen: es geht nicht um Moral, sondern um Folgen im Jugendalter
// Sprechpunkte:
// - Titel der Facharbeit nennen
// - Kontext: Sport und Gesundheit, Schuljahr 2025/2026
// - Fokus heute: was passiert biologisch, psychisch, sozial?
// - nicht: politische Grundsatzdebatte über Legalisierung
// - nicht: pauschal alle Konsumenten bewerten
// - roter Faden: Wirkung verstehen -> Risiken einordnen -> Erfahrungsberichte prüfen
#empty-slide[
  #meta[PRÄSENTATION]
  #v(0.24em)
  #grid(
    columns: (1fr, auto),
    gutter: 18pt,
    align: horizon,
    text(font: "Times New Roman", size: 46pt, weight: "bold")[Cannabiskonsum im Jugendalter],
    icon("cannabis", size: 2.35cm),
  )
  #v(0.28em)
  #lead[Welche neurobiologischen, psychischen und sozialen Folgen kann er haben?]
  #v(0.74em)
  #title-flow(([Legalisierung], [Gehirn], [Risiken], [Einordnung]))
  #v(0.56em)
  #text(size: 14pt, fill: text-muted)[Mattis Becker]
]

// Ziel der Folie:
// - Vortrag klar strukturieren
// - zeigen, dass nicht die Facharbeit 1:1 vorgelesen wird
// Sprechpunkte:
// - vier große Blöcke
// - erst Einstieg über Legalisierung und Leitfrage
// - dann Grundlagen: Cannabis, THC/CBD, Endocannabinoid-System
// - dann kurz- und langfristige Folgen
// - zuletzt Erfahrungsberichte und Fazit
// - Hinweis: Begriffe werden einfach erklärt, kein Vorwissen nötig
#empty-slide[
  #meta[STRUKTUR]
  #v(0.1em)
  #text(font: "Times New Roman", size: 32pt, weight: "bold")[Gliederung]
  #v(0.22em)
  #rect(width: 100%, height: 0.08cm, radius: 999pt, fill: rgb("#1b3822"))
  #v(0.26em)
  #grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    gutter: 14pt,
    outline-card([01 Einstieg], ([Legalisierung], [Problem], [Leitfrage])),
    outline-card([02 Grundlagen], ([Cannabis], [THC / CBD], [Gehirn])),
    outline-card([03 Folgen], ([kurzfristig], [langfristig], [Sucht])),
    outline-card([04 Einordnung], ([Erfahrungen], [Grenzen], [Fazit])),
  )
]

= Einstieg <touying:hidden>

// Ziel der Folie:
// - Publikum alltagsnah abholen
// - politische Aktualität zeigen
// - Frage nach Jugendschutz öffnen
// Sprechpunkte:
// - 1. April 2024: Cannabisgesetz in Deutschland
// - Konsum, Besitz, Eigenanbau unter bestimmten Regeln legalisiert
// - Debatte danach: Freiheit vs. Jugendschutz
// - entscheidend für Vortrag: Erwachsene dürfen mehr, Jugendliche bleiben besonders schutzbedürftig
// - Frage an Publikum möglich: was verbindet ihr eher mit Cannabis, Entspannung oder Risiko?
// - Übergang: die Arbeit fragt nicht nur politisch, sondern biologisch und sozial
== Warum das Thema aktuell ist
#header-icon("scroll-text", size: 1.6cm)

#bullet-list((
  [Cannabisgesetz seit 1. April 2024],
  [Debatte um Jugendschutz],
  [Frage: Was passiert bei Jugendlichen?],
))

#v(0.38em)
#title-flow(([CanG 2024], [mehr Sichtbarkeit], [Jugendliche], [Risiken]))

// Ziel der Folie:
// - Leitfrage der Facharbeit setzen
// - Untersuchungsbereiche sichtbar machen
// Sprechpunkte:
// - genaue Leitfrage: Welche neurobiologischen, psychischen und sozialen Folgen kann Cannabiskonsum im Jugendalter haben?
// - "kann" ist wichtig: es geht um Risiken, nicht um zwangsläufige Folgen
// - drei Ebenen: Körper/Gehirn, Psyche, soziales Leben
// - Erfahrungsberichte ergänzen Studienlage
// - Ziel: differenzierte Antwort statt Schwarz-Weiß
== Die Leitfrage

#bullet-list((
  [Was passiert im Gehirn?],
  [Welche psychischen Risiken gibt es?],
  [Welche sozialen Folgen können auftreten?],
))

#v(0.38em)
#card([LEITFRAGE], width: 86%, height: 3.15cm, tone: warning, title-gap: -1.4em)[
  #align(center + horizon)[
    #text(font: "Times New Roman", size: 23pt, weight: "bold")[Welche Folgen kann Cannabiskonsum im Jugendalter haben?]
  ]
]

// Ziel der Folie:
// - erklären, wie die Facharbeit zu ihrer Antwort kommt
// - Studien und Erfahrungsberichte von Anfang an sauber trennen
// Sprechpunkte:
// - nicht nur eine Meinung, sondern mehrere Bausteine
// - erst biologische Grundlagen: warum kann Cannabis überhaupt wirken?
// - dann Forschungslage: welche Folgen werden in Studien beobachtet?
// - dann eigene Erfahrungsberichte: drei konkrete Lebensläufe als Ergänzung
// - wichtig: Erfahrungsberichte machen das Thema greifbarer
// - aber: sie ersetzen keine große Studie
// - Übergang: zuerst die Grundlagen, damit alle die späteren Risiken verstehen
== Wie die Arbeit vorgeht

#bullet-list((
  [Grundlagen verstehen],
  [Studienlage einordnen],
  [Erfahrungsberichte vergleichen],
))

#v(0.38em)
#method-map()

= Grundlagen <touying:hidden>

// Ziel der Folie:
// - Cannabis als Pflanze und Stoffgemisch erklären
// - THC und CBD unterscheiden
// Sprechpunkte:
// - Cannabis sativa, Hanfgewächs
// - enthält viele Pflanzenstoffe: Cannabinoide, Terpene
// - für Vortrag vor allem THC und CBD relevant
// - THC: psychoaktiv, löst Rausch aus, greift stark in Wahrnehmung/Gedächtnis ein
// - CBD: nicht klassisch berauschend, kann indirekt wirken, wird oft als abschwächend beschrieben
// - wichtig: Cannabis ist nicht ein einzelner Stoff, sondern ein Stoffgemisch
== Was ist Cannabis?
#header-icon("cannabis", size: 1.6cm)

#bullet-list((
  [Cannabis sativa als Hanfpflanze],
  [Viele Cannabinoide und Terpene],
  [Im Fokus: THC und CBD],
))

#v(0.38em)
#triad(
  ([PFLANZE], [Cannabis sativa], accent-strong),
  ([RAUSCH], [THC], warning),
  ([GEGENSPIELER], [CBD], accent),
)

// Ziel der Folie:
// - Endocannabinoid-System ohne Fachüberladung erklären
// - THC-Wirkung als "falsches Signal" verständlich machen
// Sprechpunkte:
// - Körper hat eigenes Endocannabinoid-System
// - reguliert Stimmung, Schmerz, Appetit, Stress, Gedächtnis
// - CB1-Rezeptoren besonders im zentralen Nervensystem
// - CB2 eher auf Immunzellen
// - normalerweise: System bremst neuronale Aktivität nach Bedarf
// - THC bindet an CB1 und imitiert diesen Mechanismus
// - Problem: Hemmung passiert nicht unbedingt passend zum tatsächlichen Bedarf
== Wie wirkt THC?
#header-icon("brain", size: 1.6cm)

#bullet-list((
  [THC bindet an CB1-Rezeptoren],
  [Signalübertragung wird verändert],
  [Gedächtnis, Aufmerksamkeit und Motorik betroffen],
))

#v(0.34em)
#process-diagram()

// Ziel der Folie:
// - erklären, warum Jugendalter besonders relevant ist
// - präfrontaler Cortex und Hippocampus einfach einordnen
// Sprechpunkte:
// - Gehirn ist bis Mitte zwanzig nicht vollständig ausgereift
// - präfrontaler Cortex: Entscheidungen, Impulskontrolle, Planung
// - Hippocampus: Lernen und Gedächtnis
// - Cannabis kann Entwicklungsprozesse stören
// - Arbeit nennt Veränderungen wie verstärkte Ausdünnung im präfrontalen Cortex
// - besonders relevant bei frühem Beginn und höherer Dosis
== Warum Jugendliche besonders empfindlich sind
#header-icon("brain", size: 1.6cm)

#bullet-list((
  [Gehirn reift bis Mitte zwanzig],
  [Präfrontaler Cortex noch in Entwicklung],
  [Früher Konsum kann stärker eingreifen],
))

#v(0.38em)
#brain-window()

= Folgen <touying:hidden>

// Ziel der Folie:
// - Kurzfristige Folgen als drei Bereiche bündeln
// - Publikum nicht mit Studien überladen
// Sprechpunkte:
// - kurzfristig heißt: während oder kurz nach dem Konsum
// - kognitiv: Gedächtnis, Aufmerksamkeit, Konzentration
// - motorisch: Reaktion, Gleichgewicht, Koordination
// - psychisch: Angst, Panik, Paranoia möglich
// - sozial: Schule, Mitarbeit, Alltag können leiden
// - wichtig: Intensität hängt von Person, Dosis, Potenz und Mischkonsum ab
== Kurzfristige Folgen
#header-icon("clock", size: 1.6cm)

#bullet-list((
  [Konzentration und Gedächtnis],
  [Reaktion und Koordination],
  [Angst, Panik oder Rückzug möglich],
))

#v(0.38em)
#triad(
  ([KOGNITIV], [langsamer denken], accent-strong),
  ([MOTORISCH], [unsicherer handeln], warning),
  ([PSYCHISCH], [Angst möglich], risk),
)

// Ziel der Folie:
// - konkrete Studienergebnisse verständlich machen
// - Zahlen/Studien nur als Stütze, nicht als Hauptinhalt
// Sprechpunkte:
// - ABCD-Studie: Jugendliche mit Cannabis-Konsum schneiden bei Gedächtnisaufgaben schlechter ab
// - höhere Konsummenge korreliert mit schlechteren Leistungen
// - motorische Studien: Arm-/Beingeschwindigkeit und Gleichgewicht können beeinträchtigt sein
// - Reaktionszeit kann mehrere Stunden schlechter sein
// - bei Jugendlichen möglicherweise stärker, weil Gehirn empfindlicher ist
// - aber: manche motorischen Studien nicht direkt an Jugendlichen, ethisch schwierig
== Lernen und Reaktion

#bullet-list((
  [Gedächtnistests schlechter],
  [Reaktionszeit verlangsamt],
  [Koordination kann leiden],
))

#v(0.38em)
#grid(
  columns: (1fr, 1fr),
  gutter: 14pt,
  card([ABCD-STUDIE], height: 4.2cm, tone: accent-strong)[
    #text(size: 18pt, weight: "bold")[Cannabis-Konsum korrelierte mit schwächerem Gedächtnis.]
  ],
  card([MOTORIK], height: 4.2cm, tone: warning)[
    #text(size: 18pt, weight: "bold")[Geschwindigkeit, Gleichgewicht und Reaktion können sinken.]
  ],
)

// Ziel der Folie:
// - psychische und soziale Kurzzeitfolgen zusammenführen
// - zeigen: es geht nicht nur um Biologie
// Sprechpunkte:
// - Angststörungen als häufig beobachteter psychischer Effekt in Notaufnahmen
// - Facharbeit nennt u.a. schwere Angstzustände, Panikattacken, Paranoia, depressive Stimmung
// - Mischkonsum ist als Störfaktor wichtig
// - schulischer Alltag: Konzentration, Aufgabenbearbeitung, mündliche Mitarbeit
// - soziale Wirkung kann indirekt sein: weniger Teilnahme, weniger Aktivität, Konflikte
== Psyche und Alltag


#bullet-list((
  [Angst und Panik möglich],
  [Mischkonsum verstärkt Unsicherheit],
  [Schule und Alltag können direkt betroffen sein],
))

#v(1.38em)
#title-flow(([Rausch], [Konzentration sinkt], [Mitarbeit sinkt], [Konflikte]))

// Ziel der Folie:
// - langfristige Folgen als Risikolandschaft darstellen
// - "Risiko" statt "Automatismus" betonen
// Sprechpunkte:
// - langfristig geht es um wiederholten oder frühen Konsum
// - neurobiologisch: Entwicklung im präfrontalen Cortex, Aufmerksamkeit, Gedächtnis
// - psychisch: Angst, Depressionen, psychotische Episoden bei erhöhtem Risiko
// - Sucht: früher Einstieg erhöht Wahrscheinlichkeit für Abhängigkeit
// - sozial: Schule, Abschlüsse, Berufsweg, soziale Integration
== Langfristige Folgen
#header-icon("clock", size: 1.6cm)

#bullet-list((
  [Entwicklung des Gehirns],
  [Psychische Stabilität],
  [Sucht und soziale Teilhabe],
))

#v(0.38em)
#triad(
  ([GEHIRN], [Reifung gestört?], accent-strong),
  ([PSYCHE], [Risiko erhöht], warning),
  ([SOZIAL], [Schule / Beruf], risk),
)

// Ziel der Folie:
// - Suchtentwicklung verständlich erklären
// - Toleranzentwicklung als Körperanpassung zeigen
// Sprechpunkte:
// - regelmäßiger Konsum kann CB1-Rezeptoren herunterregulieren
// - weniger empfindliche Rezeptoren -> gleiche Wirkung braucht mehr Konsum
// - das nennt man Toleranzentwicklung
// - Absetzen kann Reizbarkeit, Schlafprobleme, Unruhe verursachen
// - frühes Einstiegsalter erhöht Risiko für spätere Abhängigkeit
// - nicht jeder wird abhängig, aber Risiko steigt
== Suchtentwicklung

#bullet-list((
  [Körper passt sich an],
  [Toleranz kann entstehen],
  [Früher Einstieg erhöht Risiko],
))

#v(0.38em)
#title-flow(([regelmäßig], [CB1 sinkt], [mehr nötig], [Entzug möglich]))

= Erfahrungsberichte <touying:hidden>

// Ziel der Folie:
// - Methode der eigenen Erfahrungsberichte erklären
// - Grenzen direkt transparent machen
// Sprechpunkte:
// - drei Personen, alle erster Konsum etwa mit 16
// - Fragen in Kategorien: Konsummuster, Schule, Psyche, Soziales, Körper, Kontrolle, Rückblick
// - meist Ja/Nein-Fragen, dadurch leichter vergleichbar
// - Ziel: Studienlage einordnen, nicht repräsentativ beweisen
// - wichtige Einschränkung: kleine Stichprobe, subjektive Erinnerung
== Eigene Erfahrungsberichte
#header-icon("users-round", size: 1.6cm)

#bullet-list((
  [Drei befragte Personen],
  [Erster Konsum jeweils mit 16],
  [Kategorien statt freies Gespräch],
))

#v(0.38em)
#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 14pt,
  card([PERSON 1], height: 3.4cm, tone: accent-strong)[#text(size: 18pt, weight: "bold")[21 Jahre]],
  card([PERSON 2], height: 3.4cm, tone: warning)[#text(size: 18pt, weight: "bold")[20 Jahre]],
  card([PERSON 3], height: 3.4cm, tone: accent)[#text(size: 18pt, weight: "bold")[18 Jahre]],
)

// Ziel der Folie:
// - zentrale Interviewergebnisse knapp darstellen
// - Kontrast zur Studienlage vorbereiten
// Sprechpunkte:
// - alle berichten: keine Schulabbrüche, keine klaren Leistungseinbrüche
// - zwei sagen sogar, Lernen sei eher einfacher geworden; aber Kausalität unklar
// - keine nachhaltigen psychischen Folgen berichtet
// - soziale Konflikte mit Eltern/Lehrern bei allen
// - bei einer Person kurzzeitig Kontrollverlust und zentrale Rolle im Alltag
// - Körperliche Folgen kaum berichtet, eine Person nennt Ausdauer in Verbindung mit Rauchen
== Was die Befragten berichten

#bullet-list((
  [Schule blieb stabil],
  [Konflikte mit Eltern oder Lehrern],
  [Kontrollverlust nur bei einer Person],
))

#v(0.38em)
#quote-card([Würd’s keinem empfehlen, aber schlecht fand ich es auch nicht], source: [rückblickende Einschätzung])

// Ziel der Folie:
// - Studienlage und Erfahrungsberichte nebeneinanderstellen
// - zentrale Differenz verständlich machen
// Sprechpunkte:
// - Studien zeigen statistisch erhöhte Risiken
// - Erfahrungsberichte zeigen keine schweren Folgen bei diesen drei Personen
// - beides widerspricht sich nicht zwingend
// - Statistik heißt: Wahrscheinlichkeit steigt, nicht jeder einzelne Fall tritt ein
// - Interviews sind nicht repräsentativ
// - möglich: Schutzfaktoren, soziale Einbindung, reflektierter Umgang, geringere Frequenz
== Studien vs. Erfahrung

#bullet-list((
  [Risiko ist nicht gleich Schicksal],
  [Einzelfälle ersetzen keine Studien],
  [Beides zusammen macht die Antwort differenzierter],
))

#v(0.34em)
#compare-table()

// Ziel der Folie:
// - methodische Grenze vor dem Fazit klar machen
// - verhindern, dass die Erfahrungsberichte als Gegenbeweis missverstanden werden
// Sprechpunkte:
// - eine Studie kann Wahrscheinlichkeiten und Muster zeigen
// - ein Erfahrungsbericht kann zeigen, wie unterschiedlich Verläufe sein können
// - Korrelation bedeutet nicht automatisch Ursache
// - trotzdem ist Korrelation bei Gesundheitsthemen relevant
// - bei Cannabis kommen viele Faktoren dazu: Dosis, THC-Gehalt, Alter, Mischkonsum, Umfeld
// - deshalb: kein Schwarz-Weiß-Fazit
// - aber: Jugendalter bleibt aus biologischer Sicht die riskantere Phase
== Grenzen der Einordnung

#bullet-list((
  [Studien zeigen Wahrscheinlichkeiten],
  [Erfahrungen zeigen einzelne Verläufe],
  [Korrelation ist nicht automatisch Ursache],
))

#v(0.38em)
#evidence-scale()

= Fazit <touying:hidden>

// Ziel der Folie:
// - Leitfrage beantworten
// - keine Übertreibung, aber klare Risikobewertung
// Sprechpunkte:
// - Cannabis greift über Endocannabinoid-System in Gehirnprozesse ein
// - Jugendalter besonders sensibel, weil Gehirn noch reift
// - belegte Risiken: Konzentration, Gedächtnis, Angst, Sucht, soziale Folgen
// - Erfahrungsberichte zeigen: schwere Folgen treten nicht automatisch auf
// - trotzdem: frühes Einstiegsalter erhöht Wahrscheinlichkeiten
// - Fazit der Facharbeit: Konsum besser bis Erwachsenenalter vermeiden
== Antwort auf die Leitfrage

#bullet-list((
  [Klare Risiken sind belegt],
  [Folgen treten nicht automatisch ein],
  [Je früher der Einstieg, desto riskanter],
))

#v(0.38em)
#grid(
  columns: (1fr, 1fr),
  gutter: 14pt,
  card([WICHTIG], height: 4.2cm, tone: accent-strong)[
    #text(size: 19pt, weight: "bold")[Jugendliche Gehirne sind besonders empfindlich.]
  ],
  card([GRENZE], height: 4.2cm, tone: warning)[
    #text(size: 19pt, weight: "bold")[Einzelne Erfahrungen beweisen keine allgemeine Entwarnung.]
  ],
)

// Ziel der Folie:
// - sauber abschließen
// - prägnanter letzter Satz
// Sprechpunkte:
// - noch einmal: keine Panikmache, aber Risikobewusstsein
// - Appell aus der Facharbeit: mit Konsum bis Erwachsenenalter warten
// - dann bewusst Pause
// - Fragen annehmen
== Fazit und Fragen

#bullet-list((
  [Nicht automatisch katastrophal],
  [Aber klar riskanter im Jugendalter],
  [Warten ist die sicherere Entscheidung],
  [Fragen?],
))

#v(0.42em)
#card([MITNEHMEN], width: 88%, height: 3.8cm, tone: accent-strong, title-gap: -1.8em)[
  #align(center + horizon)[
    #text(font: "Times New Roman", size: 27pt, fill: accent-strong, weight: "bold")[Risiko steigt, auch wenn Folgen nicht garantiert sind.]
  ]
]

// Ziel der Folie:
// - Quellen aus der Facharbeit sichtbar machen
// - nicht ausführlich besprechen, nur als Nachweis am Ende
// Sprechpunkte:
// - Quellen stammen aus dem Literaturverzeichnis der Facharbeit
// - im Vortrag nur bei Bedarf zeigen
// - Erfahrungsberichte sind in der Facharbeit zusätzlich dokumentiert
== Quellen

#set text(size: 6.7pt, fill: text-muted)
#set par(leading: 0.72em)
#grid(
  columns: (1fr, 1fr),
  gutter: 14pt,
  [
    ALBAUGH, Matthew D. et al. Association of Cannabis Use During Adolescence With Neurodevelopment. JAMA Psychiatry. 2021. DOI: 10.1001/jamapsychiatry.2021.1258\
    CHAN, Olsen et al. Cannabis Use During Adolescence and Young Adulthood and Academic Achievement. JAMA Pediatrics. 2024. DOI: 10.1001/jamapediatrics.2024.3674\
    GHOSH, Dipayan et al. Monoecious Cannabis sativa L. Journal of Applied Research on Medicinal and Aromatic Plants. 2023. DOI: 10.1016/j.jarmap.2023.100476\
    HIRVONEN, J. et al. Reversible and regionally selective downregulation of brain cannabinoid CB1 receptors. Molecular Psychiatry. 2012. DOI: 10.1038/mp.2011.82\
    HITCHCOCK, Leah N. et al. Acute Effects of Cannabis Concentrate on Motor Control and Speed. Frontiers in Psychiatry. 2020. DOI: 10.3389/fpsyt.2020.623672\
    HOURFANE, Sohaib et al. A Comprehensive Review on Cannabis sativa. Plants. 2023. DOI: 10.3390/plants12061245\
    IVERSEN, Leslie. Cannabis and the brain. Brain. 2003. DOI: 10.1093/brain/awg143\
    JORDAN, Susanne. Der Cannabiskonsum von Jugendlichen als Herausforderung für die pädagogische Arbeit. 2007. BZgA.\
  ],
  [
    KEUNG, Man Yee et al. Cannabis-Induced Anxiety Disorder in the Emergency Department. Cureus. 2023. DOI: 10.7759/cureus.38158\
    KIBURI, Sarah Kanana et al. Cannabis use in adolescence and risk of psychosis. Substance Abuse. 2021. DOI: 10.1080/08897077.2021.1876200\
    PAIGE, Katie J. and Craig R. COLDER. Long-Term Effects of Early Adolescent Marijuana Use. Journal of Studies on Alcohol and Drugs. 2020. DOI: 10.15288/jsad.2020.81.164\
    PERTWEE, R. G. The diverse CB1 and CB2 receptor pharmacology of three plant cannabinoids. British Journal of Pharmacology. 2008. DOI: 10.1038/sj.bjp.0707442\
    RAMAEKERS, Johannes G. et al. High-Potency Marijuana Impairs Executive Function and Inhibitory Motor Control. Neuropsychopharmacology. 2006. DOI: 10.1038/sj.npp.1301068\
    WADE, Natasha E. et al. Cannabis use and neurocognitive performance at 13-14 Years-Old. Addictive Behaviors. 2024. DOI: 10.1016/j.addbeh.2023.107930\
    WALKER, J. Michael and Susan M. HUANG. Cannabinoid analgesia. Pharmacology & Therapeutics. 2002. DOI: 10.1016/s0163-7258(02)00252-8\
    Bundesgesetzblatt Teil I. Gesetz zum kontrollierten Umgang mit Cannabis und zur Änderung weiterer Vorschriften. Zugriff: 24.01.2026.\
  ],
)
