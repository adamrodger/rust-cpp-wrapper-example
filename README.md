Rust Wrapper Around Existing C++ Library Example
================================================

This example project demonstrates writing a Rust wrapper around an existing C++ library which will be dynamically loaded
instead of statically linked. This is useful in situations where you have an existing C++ library to which you don't have
access to (or can't modify) the source code, but you do have the built artifacts (e.g. a `*.so` or `*.dll` file).

If you have access to modify and build both sides of the interop link in the same build process then there are probably
better alternatives, such as the `cxx` crate. This example assumes that you can't build/modify the C++ side.

Structure
---------

The C++ library is under `counter-cpp/`. This is an example library which maintains a persistent counter that you can create,
increment, decrement, read the current value and get a history of every value the counter has contained. This allows the
demonstration of some key concepts:

- Sharing a memory pointer between C++ and Rust
- Handling exceptions
- Freeing memory across the FFI boundary

In `counter-rs-sys/` we write a C-like wrapper interface around `counter-cpp` which is compiled and statically linked during
`cargo build` using the `cc` crate. This allows us to write a small amount of C++ glue code against which we can generate
Rust bindings using the `bindgen` crate.

The `counter-rs` crate uses `counter-rs-sys` and writes a safe idiomatic Rust wrapper around the unsafe FFI interop code.
This would be the library that we'd expect people to use when they were writing Rust code against the C++ library.

Building
--------

### Manually

Pre-requisites:

- A recent C++ compiler installed and in a location where `cc` can find it. This example uses LLVM.
- A recent Rust toolchain

First build `counter-cpp` and output the result to the `artifacts/` folder:

```bash
mkdir -p artifacts
c++ -shared -o artifacts/libcounter.so counter-cpp/Counter.cc
```

This mirrors the situation in which someone else has built the library you're going to link against rather than it being
built at the same time as your Rust code.

Now you can build the Rust code as usual with Cargo:

```bash
cargo build
```

### Docker

Check the `Dockerfile` for an example which builds the C++ library in one layer and the Rust code in another layer with
`libcounter.so` copied over, modelling how this may work in a real situation. Run with:

```bash
docker build .
```

Testing
-------

In order to run the tests you must tell the dynamic loader where to locate the library (under `artifacts/`). For example,
on Linux you can do this manually:

```bash
LD_LIBRARY_PATH=$(pwd)/artifacts cargo test
```

For production usage you would either install the dynamic libraries somewhere the OS is already configured to find them
(e.g.`/usr/local/lib`) or add the installed location to the search path (e.g. adding an entry under `/etc/ld.so.conf.d/`).
