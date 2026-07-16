# AGENTS.md

> Token‑efficient map for AI agents working on Ploogly.

## Short‑path aliases

- `main.rs` → `src/main.rs`
- `build.rs` → `src/build.rs`
- `new.rs` → `src/new.rs`
- `produce.rs` → `src/produce.rs`
- `file.rs` → `src/file.rs`
- `md2html.rs` → `src/md2html.rs`
- `interpreter_facilities.rs` → `src/interpreter_facilities.rs`
- `var_imports.rs` → `src/var_imports.rs`
- `misc.rs` → `src/misc.rs`
- `bases/mod.rs` → `src/bases/mod.rs`
- `bases/data.rs` → `src/bases/data.rs`
- `commands/mod.rs` → `src/commands/mod.rs`
- `commands/if_n_def.rs` → `src/commands/if_n_def.rs`
- `commands/include.rs` → `src/commands/include.rs`
- `commands/redir.rs` → `src/commands/redir.rs`
- `commands/set.rs` → `src/commands/set.rs`
- `commands/unset.rs` → `src/commands/unset.rs`
- `commands/var.rs` → `src/commands/var.rs`
- `commands/template.rs` → `src/commands/template.rs`
- `commands/truncate.rs` → `src/commands/truncate.rs`
- `commands/iter_dir.rs` → `src/commands/iter_dir.rs`
- `commands/add_document.rs` → `src/commands/add_document.rs`
- `commands/produce_base.rs` → `src/commands/produce_base.rs`
- `commands/load_base.rs` → `src/commands/load_base.rs`
- `commands/cutbase_extension.rs` → `src/commands/cutbase_extension.rs`
- `commands/sortbase_by_key.rs` → `src/commands/sortbase_by_key.rs`
- `commands/reverse_base_order.rs` → `src/commands/reverse_base_order.rs`
- `commands/iter_base.rs` → `src/commands/iter_base.rs`
- `commands/gen_doc.rs` → `src/commands/gen_doc.rs`
- `commands/call.rs` → `src/commands/call.rs`
- `commands/create_macro.rs` → `src/commands/create_macro.rs`
- `commands/ltrim.rs` → `src/commands/ltrim.rs`
- `commands/rtrim.rs` → `src/commands/rtrim.rs`
- `serve/listen.rs` → `src/serve/listen.rs`
- `serve/mod.rs` → `src/serve/mod.rs`

## Key concepts

- **Base** : collection of tracked files (see `bases/data.rs`). Each entry has an id, path, and abstract or produced data.
- **Produce** : converts Markdown/HTML into output, storing metadata and final content.
- **Commands** : the template language interpreter. Each command receives the byte slice, variable map, and anonymous stack.
- **Serving** : `serve/listen.rs` starts actix‑web, serves `./out/site`, and accepts keyboard rebuild/quit.

## How to use

Always refer to files by the short‑path alias. For example, to inspect the build logic, see `build.rs`. This saves tokens and keeps responses focused.

Detailed documentation is available in [`./extra/docs/index.md`](./extra/docs/index.md).
