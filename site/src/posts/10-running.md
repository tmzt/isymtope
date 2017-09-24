extends: default.liquid
title: See it for yourself.
is_section: true
hide_title: false
---

### Getting the code

You will need to clone this repository to get the source, it hasn't been submitted to <a href="https://crates.io">crates.io</a> yet.

The API to be exposed from _isymtope_ as a library has not be determined, through it should be possible to integrate into an application.


```bash
git clone https://github.com/tmzt/isymtope
cd isymtope

cargo build
cargo test
```

Currently the examples are run as cargo tests.

### Running the examples

You will need to install Rust if you haven't already.

```bash
curl https://sh.rustup.rs -sSf | sh
```

You may need to logout of your session and log back in for the `cargo` command to be in your path.

If you want to build the examples yourself, you can run:

```bash
cargo build
cargo test
```

Then you will find the html pages with the demos in the `site/src/assets/demo` folder of your checked out repository.

You can rebuild the docs with `cobalt build` from the root of the repository, and browse them from `docs/index.html`.