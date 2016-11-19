# A Rust Web app [![Build Status](https://travis-ci.org/KodrAus/rust-webapp.svg?branch=master)](https://travis-ci.org/KodrAus/rust-webapp)

This is an example web application written in the [Rust](https://www.rust-lang.org) programming language.

It's currently a work in progress.

## Getting started

If you're new to Rust, the best way to get started is download the [rustup](https://rustup.rs/) installer.
This lets you easily install and update multiple Rust toolchains.

This application is targeting the `nightly` channel.

To keep things simple, we're targeting a specific `nightly` build too, which is easy to install with `rustup` by running the following command:

```
rustup toolchain install nightly-2016-11-06
```

We can then set the default toolchain for our web project so it uses the right one.

```
cd <root of the cloned repository>
rustup override set nightly-2016-11-06
```

You should see output like the following (depending on your OS):

```
info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
info: override toolchain for '/home/ashley/src/rust-webapp' set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.14.0-nightly (cae6ab1c4 2016-11-05)
```

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