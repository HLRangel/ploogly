# Ploogly

Ploogly is a static site generator written in Rust. It uses a custom template language and a data model called "bases" to manage content. It can build a site from a project directory, serve it locally, and manage variables and macros.

## Features

- Custom template language with commands like `set`, `var`, `include`, `template`, `gen_doc`, `iter_dir`, `iter_base`, etc.
- Data storage in "bases" (JSON files) for structured content.
- Local development server with live rebuild (press R to rebuild).
- Markdown to HTML conversion.
- Macros and variable imports.
- Command-line interface.

## Installation

### Prerequisites

- Rust toolchain (stable)
- Git (optional, for commit ID)

### Build from source

