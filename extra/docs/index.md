# Ploogly documentation

## Overview

Ploogly is a static site generator written in Rust. It processes Markdown and HTML templates, tracks files through a “base” system, and serves the output via the actix‑web development server.

The template language uses `{{command args}} {{inner}}` blocks. Variables come from `project.ssg` and from frontmatter in Markdown files.

## Project structure

| Short alias          | Full path                  | Purpose |
|----------------------|----------------------------|---------|
| `main.rs`            | `src/main.rs`              | CLI entry point (`new`, `build`, `serve`, `licenses`) |
| `build.rs`           | `src/build.rs`             | Full‑build orchestration |
| `new.rs`             | `src/new.rs`               | Project scaffolding |
| `produce.rs`         | `src/produce.rs`           | Template parser / interpreter |
| `file.rs`            | `src/file.rs`              | File inclusion and dispatching |
| `md2html.rs`         | `src/md2html.rs`           | Markdown → HTML conversion |
| `interpreter_facilities.rs` | `src/interpreter_facilities.rs` | Byte‑slice navigation, tokenisation, bracket matching |
| `var_imports.rs`     | `src/var_imports.rs`       | Parses `project.ssg` and frontmatter |
| `misc.rs`            | `src/misc.rs`              | Miscellaneous helpers |
| `bases/data.rs`      | `src/bases/data.rs`        | Base abstraction and operations |
| `serve/listen.rs`    | `src/serve/listen.rs`      | Development server (actix‑web) |
| `commands/*`         | `src/commands/`            | Template language commands |

## Template commands

Commands are invoked inside `{{command args}} ... }}`.

| Command | Signature | Description |
|---------|-----------|-------------|
| `var`   | `var name` | Insert variable value; newlines become `<br>` |
| `set`   | `set name value` | Assign variable |
| `unset` | `unset name` | Remove variable |
| `include` | `include path` | Process and include external template |
| `template` | `template path [var# val ...]` | Include template with local variables |
| `ifdef` | `ifdef var` | Show inner block if variable exists |
| `ifndef` | `ifndef var` | Show inner block if variable does NOT exist |
| `truncate` | `truncate text length` | Truncate string to given length, add `...` |
| `redir` | `redir url` | Create a redirect page |
| `iter_base` | `iter_base base` | Iterate base entries; inner block gets entry data |
| `iter_dir` | `iter_dir dir` | Iterate directory files; inner block runs per file |
| `load_base` | `load_base basename` | Load base as JSON string |
| `add_document` | `add_document docpath basename` | Add document to base |
| `produce_base` | `produce_base basename` | Produce all entries in base |
| `cutbase_extension` | `cutbase_extension ext base` | Remove entries not ending with `.ext` |
| `sortbase_by_key` | `sortbase_by_key key base` | Sort entries by context key |
| `reverse_base_order` | `reverse_base_order base` | Reverse entry order |
| `gen_doc_from_template` | `gen_doc_from_template output template` | Generate document from template |
| `ltrim` | `ltrim string count` | Left‑trim `count` characters |
| `rtrim` | `rtrim string count` | Right‑trim `count` characters |
| `call` | `call ...` | Print evaluated arguments (debug) |
| `macro` | `macro name params` | Define macro (debug only) |