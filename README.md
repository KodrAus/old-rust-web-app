# A Rust Web app [![Build Status](https://travis-ci.org/KodrAus/rust-webapp.svg?branch=master)](https://travis-ci.org/KodrAus/rust-webapp)

This is an example web application written in the [Rust](https://www.rust-lang.org) programming language.

It's currently a work in progress.

## Getting started

If you're new to Rust, the best way to get started is download the [rustup](https://rustup.rs/) installer.
This lets you easily install and update multiple Rust toolchains.

This application is targeting the `nightly` channel.

### Run the app

```
cd api
cargo run
```

Then open a browser to `http://localhost:3000`.

### Run tests

```
cd api
cargo test
```

### Build documentation

```
cd api
cargo doc
```

Then open `target/doc/webapp_demo/index.html` in a browser.