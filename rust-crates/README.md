# Concurrency

- [rayon](https://crates.io/crates/rayon) -> Simple work-stealing parallelism for Rust.
- [tokio](https://crates.io/crates/tokio) -> An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
- [crossbeam](https://crates.io/crates/crossbeam) -> This crate provides a set of tools for concurrent programming.
- [flume](https://crates.io/crates/flume) -> A blazingly fast multi-producer channel.

# Hash

- [md5](https://crates.io/crates/md5) -> The package provides the MD5 hash function.

# Rust Additions

- [derive_more](https://crates.io/crates/derive_more) -> Adds #[derive(x)] macros for more traits.

# System

- [dotenv](https://crates.io/crates/dotenv) -> A `dotenv` implementation for Rust.
- [time](https://crates.io/crates/time) -> Date and time library. Fully interoperable with the standard library. Mostly compatible with `#![no_std]`.
- [chrono](https://crates.io/crates/chrono) -> It aims to be a feature-complete superset of the time library.
- [dirs](https://crates.io/crates/dirs) -> A tiny low-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows, macOS and Redox.
- [directories](https://crates.io/crates/directories) -> A tiny mid-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows and macOS.
- [enigo](https://crates.io/crates/enigo) -> Enigo lets you control your mouse and keyboard in an abstract way on different operating systems (currently only Linux, macOS, Win – Redox and *BSD planned).
- [walkdir](https://crates.io/crates/walkdir) -> Recursively walk a directory.
- [rand](https://crates.io/crates/rand) -> Random number generators and other randomness functionality.
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation) -> This crate provides Grapheme Cluster, Word and Sentence boundaries according to Unicode Standard Annex #29 rules.

# Databases

- [sqlx](https://crates.io/crates/sqlx) -> The Rust SQL Toolkit. An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL, MySQL, and SQLite.
- [sqlite](https://crates.io/crates/sqlite) -> The package provides an interface to SQLite.
- [diesel](https://crates.io/crates/diesel) -> A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL.

# Web

- [jsonwebtoken](https://crates.io/crates/jsonwebtoken) -> Create and decode JWTs in a strongly typed way.
- [sentry](https://crates.io/crates/sentry) -> Sentry (getsentry.com) client for rust ;)
- [axum](https://crates.io/crates/axum) -> Web framework that focuses on ergonomics and modularity.
- [axum-auth](https://crates.io/crates/axum-auth) -> High-level http auth extractors for axum.
- [actix-web](https://crates.io/crates/actix-web) -> Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
- [hyper](https://crates.io/crates/hyper) -> A fast and correct HTTP library.
- [tower](https://crates.io/crates/tower) -> Tower is a library of modular and reusable components for building robust clients and servers.
- [tower-http](https://crates.io/crates/tower-http) -> Tower middleware and utilities for HTTP clients and servers.
- [reqwest](https://crates.io/crates/reqwest) -> higher level HTTP client library.
- [wasm-bindgen](https://crates.io/crates/wasm-bindgen) -> Easy support for interacting between JS and Rust.

# GUI

- [egui](https://crates.io/crates/egui) -> An easy-to-use immediate mode GUI that runs on both web and native.
- [eframe](https://crates.io/crates/eframe) -> egui framework - write GUI apps that compiles to web and/or natively.

# Appearance

- [termcolor](https://crates.io/crates/termcolor) -> A simple cross platform library for writing colored text to a terminal.
- [colorful](https://crates.io/crates/colorful) -> Make your terminal output colorful.
- [prettytable-rs](https://crates.io/crates/prettytable-rs) -> A library for printing pretty formatted tables in terminal.
- [termion](https://crates.io/crates/termion) -> A bindless library for manipulating terminals.

# Serialization

- [serde](https://crates.io/crates/serde) -> Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
- [serde_json](https://crates.io/crates/serde_json) -> A JSON serialization file format.
- [bincode](https://crates.io/crates/bincode) -> A binary serialization / deserialization strategy for transforming structs into bytes and vice versa!

# User interaction

- [clap](https://crates.io/crates/clap) -> A simple to use, efficient, and full-featured Command Line Argument Parser.
- [dialoguer](https://crates.io/crates/dialoguer) -> A command line prompting library.

# Log

[A list of all recommended log tools](https://docs.rs/log/latest/log/)

- [tracing](https://crates.io/crates/tracing) -> Application-level tracing for Rust, good for async implementations.
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber) -> Utilities for implementing and composing `tracing` subscribers.
- [env_logger](https://crates.io/crates/env_logger) -> A logging implementation for `log` which is configured via an environment variable.
- [simple_logger](https://crates.io/crates/simple_logger) -> A logger that prints all messages with a readable output format.
- [simplelog](https://crates.io/crates/simplelog) -> A simple and easy-to-use logging facility for Rust's log crate.
- [pretty_env_logger](https://crates.io/crates/pretty_env_logger) -> A visually pretty env_logger.
- [stderrlog](https://crates.io/crates/stderrlog) -> Logger that logs to stderr based on verbosity specified.
- [flexi_logger](https://crates.io/crates/flexi_logger) -> An easy-to-configure and flexible logger that writes logs to stderr or stdout and/or to files. It allows custom logline formats, and it allows changing the log specification at runtime. It also allows defining additional log streams, e.g. for alert or security messages.