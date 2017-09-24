extends: default.liquid
title: See it for yourself.
is_section: true
hide_title: false
---

### Getting the code

Currently you will need to clone this repository to get the source, it hasn't been submitted to <a href="https://crates.io">crates.io</a> yet.

```bash
git clone https://github.com/tmzt/isymtope
cd isymtope

cargo build
cargo test
```

Currently the examples are run as cargo tests.

### Running the examples

If you want to build the examples yourself, you can run:

```bash
cargo build
cargo test
```

You will need to install Rust if you haven't already.

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then you will find the html pages with the demos in the `output` folder of your checked out repository.
