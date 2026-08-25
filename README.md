# Ploogly

Ploogly is an experimental static site generator written in Rust; while it is not yet mature, it can be beaten into shape to facilitate creation and deployment of personal projects. It uses a custom template language to manage content. It can build a site from a project directory, serve it locally, and manage variables and macros (WIP).

## Installation

### Prerequisites

- Rust toolchain (stable)
- Git (optional, for commit ID)

### Build from source

Build with `cargo build --release`. The binary will be in `target/release/ploogly`.

## Usage

### Create a new project

Create a new project under directory `my-proj` with `ploogly new my-proj`.

### Build an existing project

Run `ploogly build` inside a project directory. Your site's files will appear in `out/site`.

### Run a test web server

Run a web server to test your site with on port 5000 by running `ploogly serve 5000` within your project's directory.

## Examples

The RTC site is the largest website that currently uses Ploogly to generate content. If you've made a site with Ploogly, make sure to create a pull request so it can be included in this README!

## License

This project is licensed under the terms of the MIT license. There is NO WARRANTY, not even an IMPLIED WARRANTY or agreement, to the extent permitted by the law. See LICENSE file for details.
