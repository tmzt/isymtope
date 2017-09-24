extends: default.liquid
title: Introducing isymtope
is_section: true
hide_title: true
---

## Introducing, <em>isymtope</em>


### Web Application Script Language

- One syntax for client and server

- Built-in Server Side Rendering, no complicated hacks, and no Node.js required.

- Test mode allows you to compile instantly, using a server-side VM implementation.

- Native compilation mode will create static executables, which acts as a Web server and manages cached page source automatically.

- Powered by Rust - the memory safe language created by Mozilla and the Rust team.

### Rehydrate with no Flash of Unstyled Content

When the page is rendered on the server, it is linked to the generated views by way of unique keys on each element. This allows the client-side code to continue where the server left off, and does not require rerendering the page on the client side, or removing and adding any elements.

You can even serve a completly static site and enhance it with additional interactivity, all from the same template language.

### Javascript is optional, we serve a real HTML page

There is no Javascript required for viewing the pages built with _isymtope_. Not only are real pages delivered to the user, we can even support dispatching actions view old-fashioned form POST.

### Redux-ified

- Define variables using `let` that will be automatically converted into Reducers.

- Define actions for those reducers in one of three syntaxes:

  - Method pipeline syntax

    Looks like a combination of Javascript and Rust, features a subset Rust's powerful collection of iterators.

  - Filter pipeline syntax

    Uses a combination of SQL-inspired and Unix-pipeline semantics. Supports updating streaming data, reducing to a unique stream based on an id, and other tasks commonly associated with Functional Reactive Programming on the web.

  - Expression syntax

    Along with the other two, this syntax allows for simple updates to state values, as well as merging objects.

Specify your stores and action right in the template, we will generate the Javascript for you, automatically.

Default values are also rendered into the page on the server, and support for replaying actions on the server is coming.

### Build API's alongside your application

Define your API's in our template language or import definitions from grpc.io, protobuf, swagger and more. We will generate Redux reducers and actions for each API call, both REST and RPC.

You can also define APIs for your database bindings using <a href="https://diesel.rs" target="_blank">diesel.rs</a>.

Call the same API's from client and server, with the same action dispatch syntax and asynhronous semantics.

### Built with Rust

There is no javascript code running on the server, and Node.js is not required.

### Modular

_isymtope_ is designed to be modular and can target different frameworks for rendering and dispatching actions. It can also be used to generate different output formats, such as targeting Web Assembly, or even as a static site generated.

In the future, other scripting and template formats may be supported, and will target the same backend code generation.

We also intend to support different Rust web servers and frameworks.

### Demos

To see _isymtope_ in action, scroll the following demos as well the (TodoMVC)[#TodoMVC] demo below which will open in a new tab.

We are working on an interactive playground, as well as CLI tools for launching the _isymtope_ compiler.

