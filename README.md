# alib

The core infrastructure library for MAT PhD Enterprises.

`alib` provides shared functionality written in Rust for a growing ecosystem of tools focused on software orchestration, configuration management, budgeting, publishing, and sustainable inference.

## Philosophy

`alib` treats software as organizational infrastructure.

Rather than a collection of isolated applications, `alib` provides a common foundation for tools that help individuals and organizations coordinate resources, knowledge, and action.

## Included Tools

### ahelper

A module manager for installing, running, and maintaining reusable software modules.

## Other Tools Coming Soon

### dotlink

A configuration and dotfile management system.

### tbudget

A lightweight budgeting and resource allocation system.

### oquarto

Opinionated tooling for working with Quarto projects.

## Installation

```bash
git clone https://github.com/subtletea-research/alib.git
cd alib
cargo build
```

## Running

```bash
cargo run --bin ahelper -- help
```

## Architecture

```
alib
├── cmd
├── ahelper
├── dotlink
├── tbudget
└── oquarto
```

Every alib command implements a common [`Command` trait](/src/cmd.rs).

## Status

Early development (v0.1).

The API is expected to change rapidly as the ecosystem evolves.

## Contact

Please contact me, the developer Matt Turner (matt at mat.phd), with suggestions, comments, questions, etc.


## Thanks

Thank you for visiting this repo.
