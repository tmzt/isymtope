extends: default.liquid
title: Demo 2
subtitle: The famous counter, staple of all reactive frameworks.
is_section: true
hide_title: false
---


```rust
use html;

store {
    let lines = "";

    lines {
        action add => lines + "Hello!<br />";
    }
}

component greeter {
    p { "Message: <br />" }
    p (class="actions") {
        a (href="#add") click || { dispatch decrement; } { "Say Hello" }
    }
}

greeter (x="lines") {}
```

<iframe src="/isymtope/assets/demos/test_output4.html" scrollbars="no" width="600" height="100>" border="0"></iframe>

<a href="/isymtope/assets/demos/test_output4.html" target="_blank">Open demo in new tab</a>.
