# Compiling for Maximum Performance

If you don't mind slowing down compiles (hint: use this for your final builds!), there are a number of things you can add to `Cargo.toml` to squeeze out that last bit of performance.

**Step 1: Don't forget --release**

Your release build needs to be `cargo build --release`, not `cargo build`!

**Step 2: Enable Link Time Optimization**

LTO allows the optimizer to cross crate boundaries, and cull unneeded code. It also allows code in libraries to be "inlined", and can give a substantial performance boost. It also *hurts* compile times, which is why it isn't the default. Edit your `Cargo.toml`:

```toml
[profile.release]
lto = fat
```

**Step 3: Disable Multi-Core Compilation**

Rust uses all your CPUs for compilation. This makes for faster compile times, but can cause optimizations to be missed. So for *published* builds, add the following:

```toml
[profile.release]
lto = fat
codegen-units = 1
```

**Step 4: Optimize for Your CPU**

Rust will build a binary that runs on any compatible architecture. You can instruct the compiler to target a specific chip. If you have the luxury of being able to build on the same type of box you are targeting, you can use a shortcut:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

This will aggressively enable optimizations for your specific CPU. SSE2/AVX instructions that aren't available at the baseline are available, and the compiler will use them wherever it can.

You can also specify a specific chipset, but you'll have to visit LLVM's website to find the appropriate one for your architecture. These are updated quite often!