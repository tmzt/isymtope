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

Running the example

```
cargo test
```

You will find the output for this example in `output/test_output3.html`.

Note, the javascript actions for the links are currently hard-coded so you will have to modify the resulting html or the Rust source.
