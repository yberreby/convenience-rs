# convenience

Useful stuff that isn't in std.

Currently there isn't much to see there apart from a few file I/O functions.

This is somewhat inspired by what I saw of Go's rich standard library, but I
only took a quick look at it so take this with a grain of salt.

If you need to do something substantially more complex than what if possible
with this crate's API, chances are you really want to use the [`standard
library`](https://doc.rust-lang.org/std/) directly instead of this crate.

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

This is strictly less flexible than the version using only `std`: you can't
allocate just the right amount of memory if you know the size of your file
upfront, you can't read a non-UTF-8 file, and you can't reuse a buffer. However,
most of the time you don't need to do any of this, you _just want to get your
code working_. Rust's `std` is a low-level foundation, and rightly so, but
sometimes you want something that is more rigid but also simpler.

## Features

Implemented:

- `read_file(path)`
- `write_file(path, contents)`

To do:

- (maybe) `random_string(len)`

## What goes here

Any reasonably common and simple task that you could expect to be in `std`, but
isn't.
