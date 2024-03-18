# Did you Remember Release Mode?

I almost hate to mention this, but perusing `r/rust` on Reddit shows that a *lot* of people forget to enable compiler optimizations.

> When you build or run outside of debugging, use the `--release` flag!

Take the `fib_sync` project. In `debug` mode (`cargo run`), it runs in 0.94 seconds on my workstation. The same program with `cargo run --release` finishes in 0.31 seconds.

## What does --release do?

Release mode:

* Enables compiler optimizations.
* **Disables** some runtime overflow-checking.

So it's always worth *testing* in debug mode, but you want your production binaries to be compiled for release!