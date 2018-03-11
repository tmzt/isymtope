
![Current build status](https://travis-ci.org/tmzt/isymtope.svg?branch=master)

Isymtope
========

Experimental hybrid redux+IncrementalDOM client and server-side rendering in Rust. (Pronounced like isomorphic + asymtope)

Here's the classic counter example:

```
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

Running the example

[TODO: update these instructions]
