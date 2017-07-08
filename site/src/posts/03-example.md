extends: default.liquid
title: Example
is_section: true
---

## Example

The famous counter, staple of all reactive frameworks.

```rust
use html;

store {
    let counter = 0;

    counter {
        action increment => value + 1;
        action decrement => value - 1;
    }
}

component counter {
    h4 { "Counter: " { counter } }
    p (class="actions") {
        a (href="#decrement") || { dispatch decrement; } { "Decrement" }
        { " | " }
        a (href="#increment") || { dispatch increment; } { "Increment" }
    }
}

counter (x="counter") {}
```

*(Of course, the syntax is still subject to change.)*

### Running the example

If you want to build the example yourself, you can run:

```bash
cargo build
cargo test
```

Of course you will need Rust first, if you don't have it.

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then you will find the html pages, including the counter demo, generated in `output/test_output3.html`.

Remember, this is only a test.

## Demo time.

Here it is:

<iframe src="/isymtope/gen/output/test_output3.html" scrollbars="no" width="600" height="100>" border="0"></iframe>

<a href="/isymtope/gen/output/test_output3.html" target="_blank">Open demo in new tab</a>.
