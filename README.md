This repo is for testing an idea of using derive macros to generate the `trace_object` function in `mmtk-core`.

It uses the `proc-macro2` crate, so it should work using stable Rust.

It should just run:
```
cargo run
```

Use cargo-expand to see how macros expand:

```
cargo install cargo-expand
cargo expand
```
