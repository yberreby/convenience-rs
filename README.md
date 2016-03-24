# convenience - Convenient Rust

Useful stuff that isn't in std (and shouldn't be) because it's less flexible.
Somewhat inspired by Go's rich standard library, but I don't know much Go.

## Motivation

I was tired of writing this:

```rust
let mut s = String::new();
let mut f = try!(File::open("/path/to/file"));
try!(f.read_to_string(&mut s));
```

When I really wanted to do this:

```rust
let s = try!(read_file("/path/to/file"));
```

Clearer, shorter, less room for bugs. This is strictly less flexible than the
version using only `std`: you can't allocate just the right amount of memory if
you know the size of your file upfront, you can't read a non-UTF-8 file, and you
can't reuse a buffer. However, most of the time you don't need to do any of
this, you _just want to get your code working_. Rust's `std` is a low-level
foundation, and rightly so, but sometimes you want something that is more rigid
but also simpler.

## Features

Implemented:

- `read_file(path)`

To do:

- `random_string(len)`
- `write_file(path, contents, permissions)`
