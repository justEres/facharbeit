#import "@preview/touying:0.7.1": *
#import themes.simple: *
#import "vendor/codly/codly.typ": *

#let bg = rgb("#0b1220")
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
    date: [14. April 2026],
  ),
)

#set page(
  fill: bg,
  margin: (x: 1.15cm, y: 0.8cm),
)

#set text(
  font: "Noto Sans CJK JP",
  fill: text-main,
  size: 18.5pt,
)

#set par(justify: false, leading: 0.82em)

#show heading.where(level: 1): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #text(size: 14pt, fill: text-muted, tracking: 0.07em, weight: "medium")[BLOCK]
  #v(0.18em)
  #text(size: 31pt, weight: "bold", fill: text-main)[#it.body]
]

#show heading.where(level: 2): it => block(
  above: 0pt,
  below: 0pt,
  fill: none,
)[
  #text(size: 14pt, fill: text-muted)[#utils.display-current-heading(level: 1)]
  #v(0.1em)
  #text(size: 27pt, weight: "bold", fill: text-main)[#it.body]
  #v(0.28em)
  #rect(width: 1.1cm, height: 0.08cm, radius: 999pt, fill: accent)
  #v(0.38em)
]

#let meta(body) = text(size: 14pt, fill: text-muted, tracking: 0.07em, weight: "medium")[#body]
#let lead(body) = text(size: 16pt, fill: text-muted)[#body]

#let bullet-list(items) = {
  let rendered = ()
  for item in items {
    rendered.push([
      #grid(
        columns: (0.45cm, 1fr),
        gutter: 0.35cm,
        align(center + horizon)[#text(fill: accent, size: 16pt)[•]],
        text(size: 19pt, fill: text-main)[#item],
      )
    ])
  }
  stack(spacing: 0.5em, ..rendered)
}

#let outline-card(title, lines) = rect(
  fill: panel,
  stroke: (paint: rgb("#253754"), thickness: 0.8pt),
  radius: 14pt,
  inset: 12pt,
  width: 100%,
  height: 4.5cm,
)[
  #text(size: 12pt, fill: accent, weight: "bold")[#title]
  #v(0.28em)
  #for line in lines [
    #text(size: 12.5pt)[#line]
    #linebreak()
  ]
]

#let code-example(body) = [
  #show: codly-init.with()
  #show raw.where(block: true): set text(
    font: "Noto Sans Mono CJK JP",
    size: 9.3pt,
    fill: text-main,
  )
  #show raw.line: set text(
    font: "Noto Sans Mono CJK JP",
    size: 9.3pt,
    fill: text-main,
  )
  #codly(
    fill: panel,
    radius: 10pt,
    stroke: 0.6pt + rgb("#24344d"),
    inset: 8pt,
    zebra-fill: none,
    number-format: none,
    languages: (
      rust: (name: "Rust", color: rgb("#f08d49")),
      bash: (name: "Shell", color: rgb("#8fd18a")),
      wat: (name: "WAT", color: rgb("#c48cff")),
      text: (name: "Text", color: accent),
    ),
  )
  #box(width: 78%)[
    #body
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
  #text(size: 35pt, weight: "bold")[WebAssembly als Abkürzung zum eigenen Compiler?]
  #v(0.34em)
  #lead[Eigene Sprache, eigener Compiler, aber möglichst ohne Vorwissen erklärt.]
  #v(0.8em)
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
  #v(0.25em)
  #text(size: 24pt, weight: "bold")[Gliederung]
  #v(0.45em)
  #grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    gutter: 12pt,
    outline-card([01 Einstieg], ([Publikumsfrage], [Motivation], [Leitfrage])),
    outline-card([02 Grundlagen], ([Compiler], [Interpreter], [WebAssembly])),
    outline-card([03 Selbstversuch], ([Sprache], [Pipeline], [Ausführung])),
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
  [Wenig Syntax, leicht lesbar],
  [Läuft gleich durch alle Schritte],
))

#v(0.3em)
#code-example[
```rust
fn main() {
    print(7 + 5);
}
```
]

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
  [Text -> kleine Bausteine],
  [Zahlen, Namen, Zeichen werden getrennt],
  [Grundlage für alle nächsten Schritte],
))

#v(0.3em)
#code-example[
```text
print ( 7 + 5 )
Ident  ( Int Plus Int )
```
]

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
  [Tokens -> Programmstruktur],
  [Was gehört zusammen?],
  [Ergebnis: ein Baum des Programms],
))

#v(0.3em)
#code-example[
```text
print
└─ plus
   ├─ 7
   └─ 5
```
]

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
  [AST -> interne Zwischenform],
  [Macht die Übersetzung übersichtlicher],
  [Noch nicht WebAssembly, aber schon näher dran],
))

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
  [Interne Form -> WebAssembly],
  [Ein Ziel statt vieler Plattformen],
  [Kein eigener CPU-spezifischer Code nötig],
))

#v(0.3em)
#code-example[
```wat
(func (export "main")
  i64.const 7
  i64.const 5
  i64.add
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
#code-example[
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
