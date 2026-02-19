# Facharbeit – Eigene Programmiersprache mit WebAssembly

Dieses Repository enthält den vollständigen Quellcode und die Dokumentation meiner Facharbeit zum Thema Entwicklung einer eigenen Programmiersprache und deren Compiler mit Fokus auf moderne Compilerarchitekturen und WebAssembly als Zielplattform.

Die Arbeit kombiniert theoretische Grundlagen mit einer praktischen Implementierung eines Compilers, geschrieben in Rust, sowie der vollständigen schriftlichen Ausarbeitung, gesetzt mit Typst.

---

## Überblick

Ziel dieser Facharbeit ist es, den Aufbau und die Funktionsweise eines Compilers zu verstehen und praktisch umzusetzen. Dazu wurde eine eigene Programmiersprache entwickelt, einschließlich:

* Lexer
* Parser
* Intermediate Representation (IR)
* Codegenerator für WebAssembly

Zusätzlich behandelt die schriftliche Ausarbeitung die theoretischen Grundlagen von:

* Compilerarchitektur
* WebAssembly
* Parsing und Syntaxanalyse
* Codegenerierung
* Plattformunabhängiger Ausführung von Programmen

---

## Repository-Struktur

```
.
├── src/                  # Rust Compiler-Implementierung
├── examples/             # Beispielprogramme in der eigenen Sprache
├── infos/                # Zusätzliche Informationen und Materialien
├── fonts/                # Schriftarten für das Typst-Dokument
├── vendor/               # Externe Abhängigkeiten
│
├── facharbeit.typ        # Hauptdatei der Facharbeit (Typst)
├── bibliography.yaml     # Quellenverzeichnis
├── facharbeit.pdf        # Kompilierte Facharbeit
│
├── *.eres                # Beispielprogramme der eigenen Sprache
├── Cargo.toml            # Rust-Projektdefinition
```

---

## Technologien

Dieses Projekt verwendet folgende Technologien:

* Rust – Implementierung des Compilers
* WebAssembly – Zielplattform für generierten Code
* Typst – Satzsystem für die Facharbeit
* YAML – Bibliographieformat
* Git – Versionskontrolle

---

## Compiler

Der Compiler ist in Rust implementiert und folgt der klassischen Compilerpipeline:

1. Lexikalische Analyse (Tokenisierung)
2. Syntaxanalyse (Parsing)
3. Intermediate Representation
4. Codegenerierung (WebAssembly)

Beispielprogramm:

```
factorial.eres
add.eres
```

---

## Facharbeit kompilieren

Voraussetzung: Typst installiert

Installation:

```
cargo install typst-cli
```

Facharbeit kompilieren:

```
typst compile facharbeit.typ
```

Ergebnis:

```
facharbeit.pdf
```

---

## Motivation

Die Entwicklung einer eigenen Programmiersprache und eines Compilers ermöglicht ein tiefes Verständnis von:

* Funktionsweise moderner Programmiersprachen
* Übersetzung von Quellcode in ausführbaren Code
* Plattformunabhängiger Softwareausführung
* Internen Strukturen von Compilern

WebAssembly wurde gewählt, da es eine moderne, portable und effiziente Zielplattform darstellt.

---

## Autor

Erik Tschöpe
Facharbeit – Informatik
Gymnasiale Oberstufe

---

## Lizenz

Dieses Projekt dient ausschließlich Bildungszwecken im Rahmen einer schulischen Facharbeit.
