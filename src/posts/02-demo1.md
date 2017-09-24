extends: default.liquid
title: Demo 1
subtitle: The famous counter, staple of all reactive frameworks.
is_section: true
hide_title: false
---


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

Here it is running:

<iframe src="/isymtope/assets/demos/test_output3.html" scrollbars="no" width="600" height="100>" border="0"></iframe>

<a href="/isymtope/assets/demos/test_output3.html" target="_blank">Open demo in new tab</a>.
