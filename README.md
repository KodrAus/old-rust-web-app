# A Rust Web app [![Build Status](https://travis-ci.org/KodrAus/rust-webapp.svg?branch=master)](https://travis-ci.org/KodrAus/rust-webapp)

This application is a bit out of date now. For something more current, and more focused on application development, see [here](https://github.com/KodrAus/rust-web-app).

This is an example web application written in the [Rust](https://www.rust-lang.org) programming language.

## Getting started

This repo is part of a Rust introduction workshop, which you can follow [here](https://www.gitbook.com/book/kodraus/rust-webapp/details).

The `/api` folder includes an implementation of the API built in the guide. The `/api-futures` folder includes an implementation of a simpler API built using the new, unreleased asynchronous io `futures` + `tokio` stack.

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
