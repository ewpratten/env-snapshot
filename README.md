# env-snapshot
[![Crates.io](https://img.shields.io/crates/v/env-snapshot)](https://crates.io/crates/env-snapshot) 
[![Docs.rs](https://docs.rs/env-snapshot/badge.svg)](https://docs.rs/env-snapshot) 
[![Build](https://github.com/Ewpratten/env-snapshot/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/env-snapshot/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/env-snapshot/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/env-snapshot/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/env-snapshot/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/env-snapshot/actions/workflows/audit.yml)


`env-snapshot` allows you to dump an arbitrary environment to a small-size binary file format, then at any time, on any machine, reload that environment and spawn a process with it.

This is a debugging tool.

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install env-snapshot
```
