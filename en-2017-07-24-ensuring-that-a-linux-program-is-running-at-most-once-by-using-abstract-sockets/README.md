Source code for my [Ensuring That a Linux Program Is Running at Most Once by Using Abstract Sockets](https://blog.petrzemek.net/2017/07/24/ensuring-that-a-linux-program-is-running-at-most-once-by-using-abstract-sockets/) English blog post.

Works only on Linux. Requires at least Rust 1.19 due to the use of the [`exprintln`](https://doc.rust-lang.org/std/macro.eprintln.html) macro.

To build and run the program, use
```
$ cargo run
```
