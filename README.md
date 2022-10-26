PepegSitter
===========

[![Crates.io][crates-badge]][crates-url]
[![License][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![docs.rs][docsrs-badge]][docsrs-url]

[crates-badge]: https://img.shields.io/crates/v/pepegsitter.svg
[crates-url]: https://crates.io/crates/pepegsitter
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/dav1dde/pepegsitter/blob/master/LICENSE
[actions-badge]: https://github.com/Dav1dde/pepegsitter/workflows/CI/badge.svg
[actions-url]: https://github.com/Dav1dde/pepegsitter/actions?query=workflow%3ACI+branch%3Amaster
[docsrs-badge]: https://img.shields.io/docsrs/pepegsitter
[docsrs-url]: https://docs.rs/pepegsitter


<p align="center">
    <img width="auto" height="300px" src="https://user-images.githubusercontent.com/255721/198038819-bb22cd9d-f8d3-4d71-84ac-5d4f86b8b4e8.png" />
</p>

A collection of tree sitter parsers packaged into a single crate with `tree-sitter-highlight` support.
The `build.rs` is also tuned to statically link parsers that depend on `libstdc++`.

Every parser is included as a git submodule and has it's own Cargo Feature.

## Usage

Enable the parsers you need through cargo features, by default all features are enabled.

```rust
// Get the TreeSitter language
let _ = pepegsitter::bash::language();
// Get a new `HighlightConfiguration` for the language.
let _ = pepegsitter::bash::highlight();

// Queries are accessible module constants.
let _ = pepegsitter::bash::HIGHLIGHT_QUERY;
```

## Parsers

Currently includes the following parsers:

* Bash
* C
* C++
* CSS
* D
* Go
* Haskell
* Html
* Java
* JavaScript
* Json
* Lua
* Python
* Rust
* Toml
* Typescript / TSX
* Yaml
