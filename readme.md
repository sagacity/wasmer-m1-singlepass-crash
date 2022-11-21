This is a reproduction scenario of a crash I am seeing using the 'Singlepass' compiler on Apple M1. It does not appear on Cranelift and does not seem to appear on Linux.

How to build:
* First, build the 'wasm' project: `cargo build --target wasm32-unknown-unknown -p wasm`
* Then, build and run the 'host' project: `cargo run -p host`

The wasm project uses `rmp_serde` to decode some msgpack data. This results in a heap out of bounds error. Using `serde_json` to deserialize some JSON works fine in the same setup.

As noted, this only seems to crash on Apple M1. Additionally, on the same machine running a host cross-compiled to x86_64 using `cargo run --target x86_64-apple-darwin -p host` crashes with this assertion:
```
  left: `XMM0`,
 right: `XMM0`', /Users/roy/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-compiler-singlepass-3.0.0/src/emitter_x64.rs:1666:40
```

So, I'm unsure if this is an issue in wasmer, but it seems weird that it only crashes with the Singlepass compiler on an Apple M1 and therefore I am assuming there is something broken in the wasmer code generation.

The issue occurs in wasmer 2.3 and the recently released 3.0.0.
