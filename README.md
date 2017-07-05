# ismytope
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
        a (href="#decrement") { "Decrement" }
        { " | " }
        a (href="#increment") { "Increment" }
    }
}

counter (x="counter") {}
```
