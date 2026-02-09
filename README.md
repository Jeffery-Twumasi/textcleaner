# TextCleaner

TextCleaner is a command-line tool written in Rust that helps clean and process text files by automating common text-editing tasks.

This project was developed as part of a **Digital Business** course and focuses on automation, developer workflows, and software distribution.

---

## Problem Statement

Working with raw text files often involves repetitive manual tasks such as removing empty lines or filtering specific content. These tasks can be time-consuming and error-prone when done manually.

TextCleaner automates these operations through a simple and efficient command-line interface.

---

## Features

- Remove empty lines from text files
- Filter lines based on a keyword
- Simple and fast CLI interface
- Written in Rust for performance and safety
- Distributed as precompiled Linux binaries
- Easy to integrate into scripts and workflows

---

## Usage

Run the program from a Linux terminal.

Display help information:

```bash
./textcleaner --help
Remove empty lines from a file:

bash
Copy code
./textcleaner remove-empty input.txt
Filter lines containing a specific word:

bash
Copy code
./textcleaner only keyword input.txt
Download
Precompiled Linux binaries are available on the project website:

👉 https://jeffery-twumasi.github.io/textcleaner/

Available platforms:

Linux (aarch64)

Linux (x86_64)

Note: These are Linux command-line binaries and must be executed from a Linux terminal.

Developer Workflow
Rust project managed using Cargo

Continuous Integration via GitHub Actions

Automated cross-compilation for multiple architectures

Static binaries hosted on GitHub Pages

Project Structure
text
Copy code
.
├── src/
│   └── main.rs
├── docs/
│   ├── index.html
│   ├── textcleaner-aarch64
│   └── textcleaner-x86_64
├── .github/
│   └── workflows/
│       └── main.yml
├── Cargo.toml
└── README.md
Author
Created by Jeffery Twumasi
