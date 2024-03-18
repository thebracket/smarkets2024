# Profile Guided Optimization
 
> Please refer to https://github.com/Kobzol/cargo-pgo for full instructions.

PGO is often useful for binaries that typically follow the same execution path. It works by instrumenting a binary to record branch decisions, and then using that data to optimize the binary. It can lead to great performance improvements. It can *also* lead to making things worse if the test data wasn't typical!

The easy way to perform PGO is:

```bash
cargo install cargo-pgo
rustup component add llvm-tools-preview
```

Once this is in place, you can invoke your program with:

```bash
cargo pgo run
```

Let it run its course, and then you can produce an optimized binary with:

```bash
cargo pgo optimize
```

It is that simple. However:

* Your resulting `profdata` file can be quite large.
* If you didn't feed your program typical/likely test data, you won't see any gains---it may even get worse.

> If you are building a library, making a test suite of likely cases as unit tests and then running `cargo pgo test` can be used to generate an optimization profile.