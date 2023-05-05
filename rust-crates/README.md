# Links

- [blessed.rs](https://blessed.rs/crates) -> An unofficial guide to the Rust ecosystem.

# Concurrency/Futures

- [futures-util](https://crates.io/crates/futures-util) -> Common utilities and extension traits for the futures-rs library.
- [rayon](https://crates.io/crates/rayon) -> Simple work-stealing parallelism for Rust.
- [tokio](https://crates.io/crates/tokio) -> An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
- [crossbeam](https://crates.io/crates/crossbeam) -> This crate provides a set of tools for concurrent programming.
- [flume](https://crates.io/crates/flume) -> A blazingly fast multi-producer channel.
- [async-trait](https://crates.io/crates/async-trait) -> Type erasure for async trait methods.

# Hash

- [md5](https://crates.io/crates/md5) -> The package provides the MD5 hash function.
- [sha2](https://crates.io/crates/sha2) -> Pure Rust implementation of the SHA-2 hash function family including SHA-224, SHA-256, SHA-384, and SHA-512.
- [bcrypt](https://crates.io/crates/bcrypt) -> Easily hash and verify passwords using bcrypt.

# Rust Additions

- [libc](https://crates.io/crates/libc) -> Raw FFI bindings to platform libraries like libc.
- [memmap2](https://crates.io/crates/memmap2) -> Cross-platform Rust API for memory-mapped file IO.
- [byteflags](https://crates.io/crates/bitflags) -> A macro to generate structures which behave like bitflags.
- [bytemuck](https://crates.io/crates/bytemuck) -> A crate for mucking around with piles of bytes.
- [byteorder](https://crates.io/crates/byteorder) -> Library for reading/writing numbers in big-endian and little-endian.
- [num](https://crates.io/crates/num) -> A collection of numeric types and traits for Rust, including bigint, complex, rational, range iterators, generic integers, and more.
- [itertools](https://crates.io/crates/itertools) -> Extra iterator adaptors, iterator methods, free functions, and macros.
- [tinyvec](https://crates.io/crates/tinyvec) -> `tinyvec` provides 100% safe vec-like data structures.
- [smallvec](https://crates.io/crates/smallvec) -> 'Small vector' optimization: store up to a small number of items on the stack.
- [arrayvec](https://crates.io/crates/arrayvec) -> A vector with fixed capacity, backed by an array (it can be stored on the stack too). Implements fixed capacity ArrayVec and ArrayString.
- [indexmap](https://crates.io/crates/indexmap) -> A hash table with consistent order and fast iteration.
- [color-eyre](https://crates.io/crates/color-eyre) -> An error report handler for panics and eyre::Reports for colorful, consistent, and well formatted error reports for all kinds of errors.
- [anyhow](https://crates.io/crates/anyhow) -> Flexible concrete Error type built on std::error::Error.
- [thiserror](https://crates.io/crates/thiserror) -> This library provides a convenient derive macro for the standard library's `std::error::Error` trait.
- [derive_more](https://crates.io/crates/derive_more) -> Adds #[derive(x)] macros for more traits.
- [derive_builder](https://crates.io/crates/derive_builder) -> Rust macro to automatically implement the builder pattern for arbitrary structs.
- [bitvec](https://crates.io/crates/bitvec) -> Addresses memory by bits, for packed collections and bitfields.
- [regex](https://crates.io/crates/regex) -> An implementation of regular expressions for Rust. This implementation uses finite automata and guarantees linear time matching on all inputs.
- [lazy-regex](https://crates.io/crates/lazy-regex) -> lazy static regular expressions checked at compile time.
- [fancy-regex](https://crates.io/crates/fancy-regex) -> An implementation of regexes, supporting a relatively rich set of features, including backreferences and look-around.
- [confy](https://crates.io/crates/confy) -> Boilerplate-free configuration management.
- [nuttype](https://crates.io/crates/nutype) -> The newtype with guarantees.
- [toml](https://crates.io/crates/toml) -> A native Rust encoder and decoder of TOML-formatted files and streams. Provides implementations of the standard Serialize/Deserialize traits for TOML data to facilitate deserializing and serializing Rust structures.
- [bstr](https://crates.io/crates/bstr) -> A string type that is not required to be valid UTF-8.
- [flate2](https://crates.io/crates/flate2) -> DEFLATE compression and decompression exposed as Read/BufRead/Write streams. Supports miniz_oxide and multiple zlib implementations. Supports zlib, gzip, and raw deflate streams.
- [arboard](https://crates.io/crates/arboard) -> Image and text handling for the OS clipboard.
- [lopdf](https://crates.io/crates/lopdf) -> A Rust library for PDF document manipulation.
- [console](https://crates.io/crates/console) -> A terminal and console abstraction for Rust.

# System

- [tempfile](https://crates.io/crates/tempfile) -> A library for managing temporary files and directories.
- [uuid](https://crates.io/crates/uuid) -> A library to generate and parse UUIDs.
- [humantime](https://crates.io/crates/humantime) -> A parser and formatter for std::time::{Duration, Sys
- temTime}.
- [notify](https://crates.io/crates/notify) -> Cross-platform filesystem notification library.
- [notify-rust](https://crates.io/crates/notify-rust) -> Show desktop notifications (linux, bsd, mac). Pure Rust dbus client and server.
- [regex](https://crates.io/crates/regex) -> An implementation of regular expressions for Rust. This implementation uses finite automata and guarantees linear time matching on all inputs.
- [dotenvy](https://crates.io/crates/dotenvy) -> A well-maintained fork of the dotenv crate.
~~- [dotenv](https://crates.io/crates/dotenv) -> A `dotenv` implementation for Rust.~~
- [time](https://crates.io/crates/time) -> Date and time library. Fully interoperable with the standard library. Mostly compatible with `#![no_std]`.
- [chrono](https://crates.io/crates/chrono) -> It aims to be a feature-complete superset of the time library.
- [dirs](https://crates.io/crates/dirs) -> A tiny low-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows, macOS and Redox.
- [directories](https://crates.io/crates/directories) -> A tiny mid-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows and macOS.
- [enigo](https://crates.io/crates/enigo) -> Enigo lets you control your mouse and keyboard in an abstract way on different operating systems (currently only Linux, macOS, Win – Redox and *BSD planned).
- [globwalk](https://crates.io/crates/globwalk) -> Glob-matched recursive file system walking.
- [glob](https://crates.io/crates/glob) -> Support for matching file paths against Unix shell style patterns.
- [walkdir](https://crates.io/crates/walkdir) -> Recursively walk a directory.
- [rand](https://crates.io/crates/rand) -> Random number generators and other randomness functionality.
- [getrandom](https://crates.io/crates/getrandom) -> A small cross-platform library for retrieving random data from system source.
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation) -> This crate provides Grapheme Cluster, Word and Sentence boundaries according to Unicode Standard Annex #29 rules.
- [config](https://crates.io/crates/config) -> Layered configuration system for Rust applications.
- [figment](https://crates.io/crates/figment) -> A configuration library so con-free, it's unreal.
- [tar](https://crates.io/crates/tar) -> A Rust implementation of a TAR file reader and writer. This library does not currently handle compression, but it is abstract over all I/O readers and writers. Additionally, great lengths are taken to ensure that the entire contents are never required to be entirely resident in memory all at once.
- [flate2](https://crates.io/crates/flate2) -> DEFLATE compression and decompression exposed as Read/BufRead/Write streams. Supports miniz_oxide and multiple zlib implementations. Supports zlib, gzip, and raw deflate streams.
- [signal-hook](https://crates.io/crates/signal-hook) -> Unix signal handling.

# Databases

- [sqlx](https://crates.io/crates/sqlx) -> The Rust SQL Toolkit. An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL, MySQL, and SQLite.
- [sqlite](https://crates.io/crates/sqlite) -> The package provides an interface to SQLite.
- [diesel](https://crates.io/crates/diesel) -> A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL.

# Web

- [jsonwebtoken](https://crates.io/crates/jsonwebtoken) -> Create and decode JWTs in a strongly typed way.
- [sentry](https://crates.io/crates/sentry) -> Sentry (getsentry.com) client for rust ;)
- [futures](https://crates.io/crates/futures) -> An implementation of futures and streams featuring zero allocations, composability, and iterator-like interfaces.
- [axum](https://crates.io/crates/axum) -> Web framework that focuses on ergonomics and modularity.
- [axum-auth](https://crates.io/crates/axum-auth) -> High-level http auth extractors for axum.
- [actix-web](https://crates.io/crates/actix-web) -> Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
- [hyper](https://crates.io/crates/hyper) -> A fast and correct HTTP library.
- [tower](https://crates.io/crates/tower) -> Tower is a library of modular and reusable components for building robust clients and servers.
- [tower-http](https://crates.io/crates/tower-http) -> Tower middleware and utilities for HTTP clients and servers.
- [reqwest](https://crates.io/crates/reqwest) -> higher level HTTP client library.
- [ureq](https://crates.io/crates/ureq) -> Simple, safe HTTP client.
- [wasm-bindgen](https://crates.io/crates/wasm-bindgen) -> Easy support for interacting between JS and Rust.
- [lettre](https://crates.io/crates/lettre) -> Email client.
- [lambda-web](https://crates.io/crates/lambda-web) -> Run Rust web frameworks on AWS Lambda.
- [lambda_http](https://crates.io/crates/lambda_http) -> Application Load Balancer and API Gateway event types for AWS Lambda.
- [lambda_runtime](https://crates.io/crates/lambda_runtime) -> AWS Lambda Runtime.
- [poem-openapi](https://crates.io/crates/poem-openapi) -> OpenAPI support for Poem.
- [fantoccini](https://crates.io/crates/fantoccini) -> High-level API for programmatically interacting with web pages through WebDriver.
- [mockito](https://crates.io/crates/mockito) -> HTTP mocking for Rust.
- [mockall](https://crates.io/crates/mockall) -> A powerful mock object library for Rust.
- [mockall_double](https://crates.io/crates/mockall_double) -> Test double adapter for Mockall.
- [scraper](https://crates.io/crates/scraper) -> HTML parsing and querying with CSS selectors.

# TUI

- [tui](https://crates.io/crates/tui) -> A library to build rich terminal user interfaces or dashboards.

# GUI

- [egui](https://crates.io/crates/egui) -> An easy-to-use immediate mode GUI that runs on both web and native.
- [eframe](https://crates.io/crates/eframe) -> egui framework - write GUI apps that compiles to web and/or natively.
- [rfd](https://crates.io/crates/rfd) -> Rusty File Dialogs is a cross platform Rust library for using native file open/save dialogs. It provides both asynchronous and synchronous APIs.

# Appearance

- [owo-colors](https://crates.io/crates/owo-colors) -> Zero-allocation terminal colors that'll make people go owo.
- [termcolor](https://crates.io/crates/termcolor) -> A simple cross platform library for writing colored text to a terminal.
- [colored](https://crates.io/crates/colored) -> The most simple way to add colors in your terminal.
- [colorful](https://crates.io/crates/colorful) -> Make your terminal output colorful.
- [comfy-table](https://crates.io/crates/comfy-table) -> An easy to use library for building beautiful tables with automatic content wrapping.
- [prettytable-rs](https://crates.io/crates/prettytable-rs) -> A library for printing pretty formatted tables in terminal.
- [termion](https://crates.io/crates/termion) -> A bindless library for manipulating terminals.

# Serialization

- [serde](https://crates.io/crates/serde) -> Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
- [serde_json](https://crates.io/crates/serde_json) -> A JSON serialization file format.
- [bincode](https://crates.io/crates/bincode) -> A binary serialization / deserialization strategy for transforming structs into bytes and vice versa!
- [bytesize](https://crates.io/crates/bytesize) -> An utility for human-readable bytes representations.
- [humansize](https://crates.io/crates/humansize) -> A configurable crate to easily represent sizes in a human-readable format.
- [fast_qr](https://crates.io/crates/fast_qr) -> Generates optimized QRCode.
- [qrcode-generator](https://crates.io/crates/qrcode-generator) -> Generate QR Code matrices and images in RAW, PNG and SVG formats.
- [qrcodegen](https://crates.io/crates/qrcodegen) -> High-quality QR Code generator library.
- [bardecoder](https://crates.io/crates/bardecoder) -> Detect and decode QR Codes.

# User interaction

- [notify-rust](https://crates.io/crates/notify-rust) -> Show desktop notifications (linux, bsd, mac). Pure Rust dbus client and server.
- [lexopt](https://crates.io/crates/lexopt) -> Minimalist pedantic command line parser.
- [pico-args](https://crates.io/crates/pico-args) -> An ultra simple CLI arguments parser.
- [argh](https://crates.io/crates/argh) -> Derive-based argument parser optimized for code size.
- [clap](https://crates.io/crates/clap) -> A simple to use, efficient, and full-featured Command Line Argument Parser.
- [dialoguer](https://crates.io/crates/dialoguer) -> A command line prompting library.

# Log

[A list of all recommended log tools](https://docs.rs/log/latest/log/)

- [log](https://crates.io/crates/log) -> A lightweight logging facade for Rust.
- [tracing](https://crates.io/crates/tracing) -> Application-level tracing for Rust, good for async implementations.
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber) -> Utilities for implementing and composing `tracing` subscribers.
- [env_logger](https://crates.io/crates/env_logger) -> A logging implementation for `log` which is configured via an environment variable.
- [simple_logger](https://crates.io/crates/simple_logger) -> A logger that prints all messages with a readable output format.
- [simplelog](https://crates.io/crates/simplelog) -> A simple and easy-to-use logging facility for Rust's log crate.
- [pretty_env_logger](https://crates.io/crates/pretty_env_logger) -> A visually pretty env_logger.
- [stderrlog](https://crates.io/crates/stderrlog) -> Logger that logs to stderr based on verbosity specified.
- [flexi_logger](https://crates.io/crates/flexi_logger) -> An easy-to-configure and flexible logger that writes logs to stderr or stdout and/or to files. It allows custom logline formats, and it allows changing the log specification at runtime. It also allows defining additional log streams, e.g. for alert or security messages.
- [log4rs](https://crates.io/crates/log4rs) -> A highly configurable multi-output logging implementation for the `log` facade.