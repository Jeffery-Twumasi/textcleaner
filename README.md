# TextCleaner

TextCleaner is a command-line tool written in Rust that helps clean and process text files by automating common text-editing tasks.

This project was developed as part of a Digital Business course and focuses on automation, developer workflows, and software distribution.

---

## Problem Statement

Working with raw text files often involves repetitive manual tasks such as removing empty lines or filtering specific content. These tasks can be time-consuming and error-prone when done manually.

TextCleaner automates these operations through a simple and efficient command-line interface.

---

## Features

- Remove empty lines from text files
- Filter lines containing a specific keyword
- Simple command-line interface
- Written in Rust
- Precompiled Linux binaries
- Distributed via GitHub Pages

---

## Usage

Run the program from a Linux terminal.

### Display help information

```bash
./textcleaner --help
```

### Remove empty lines from a file

```bash
./textcleaner remove-empty input.txt
```

### Filter lines containing a specific word

```bash
./textcleaner only keyword input.txt
```

---

## Download

Precompiled Linux binaries are available on the project website:

https://jeffery-twumasi.github.io/textcleaner/

### Available platforms

- Linux (aarch64 / ARM64)
- Linux (x86_64)

---

## Continuous Integration

- Automated builds using GitHub Actions
- Cross-compilation for multiple architectures

---

## Project Structure

```text
src/
docs/
.github/
Cargo.toml
README.md
```

---

## Author

Jeffery Twumasi
