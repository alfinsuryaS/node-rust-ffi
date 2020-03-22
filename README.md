# Rust FFI (Foreign Function Interface)
Writing safe and fast native modules with Rust and running natively in NodeJS. 

## FFI Overview

#### What is FFI:
FFI refers to the ability for code written in one language (the “host language,” such as Rust, Ruby), to access and invoke functions written in another language.

#### When Not To Use FFI:
As amazing as FFI is, it’s not always the right tool for the job. May be a better approach in any of these scenarios:
- You need to implement your own low-level or highly-optimized code. 
- You need to perform some delicate callbacks from the guest language into the host language.
- The library makes heavy use of compile-time or preprocessor features


## Dependencies
#### Rust:
- rust -> 0.2.3
- rand -> "0.7.2"
- libc -> "0.2"

#### Node:
- ffi -> "2.3.0"
- nodemon

## Running
```bash
$ npm start
```

## Build
```bash
$ cd native/
$ cargo build --release
```

## Test
```bash
$ cd native/
$ cargo test
```
