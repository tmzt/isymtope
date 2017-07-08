extends: default.liquid
title: Introducing isymtope
is_section: true
hide_title: true
---

## Introducing, <em>isymtope</em>

### One template language for client and server

Design your app once and only once.

### Redux-ified

Specify your stores and action right in the template, we will generate the Javascript for you, automatically.

### API's included.

Define your API's in our template language or import definitions from grpc.io, protobuf, swagger and more. We will generate Redux for each API call, both REST and RPC.

You can also define APIs for your database bindings using <a href="https://diesel.rs" target="_blank">diesel.rs</a>.

### Make free calls!

Calls to your API's are free when rendering server-side (when the implementation is in the same process). Calls are always treated as asynchronous and exposed to your app as Redux actions.

You write the exact same code to call your services.

### No (server-side) Javascript

This framework is pure Rust and can even by compile your entire site into a native binary taking advantage of Rust's zero-cost abstractions. You don't need Node.js or any Javascript framework running on your server.
