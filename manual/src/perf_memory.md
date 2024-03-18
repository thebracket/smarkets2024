# Memory Fragmentation and Reallocation

For long-running processes that grow (and maybe shrink) collection capacities you can run into two problems:

* *Memory Reallocation*. We covered part of this in our talk about using `with_capacity` --- you want to avoid vectors reallocating over and over.
* *Memory Fragmentation*. This is linked to the first one. Whenever you allocate, the allocator has to search available memory for a large enough chunk into which it can store your new allocation. Over time, it starts to look like `defrag` on old DOS systems: the gaps between chunks shrink, and allocation takes longer.

You can use an arena or slotmap to minimize this: they reuse the same memory over and over, avoiding reallocations (and therefore fragmentation). We talked about that earlier.

You can also change your allocator! Your compilation time will skyrocket, but for long-running servers it can be worth it. I like to use `jemalloc` (on everything except Windows!)

Add this to your `Cargo.toml`:

```toml
# Support JemAlloc on supported platforms
[target.'cfg(any(target_arch = "x86", target_arch = "x86_64"))'.dependencies]
jemallocator = "0.5"
```

And in your `main.rs` file add the following:

```rust
// Use JemAllocator only on supported platforms
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use jemallocator::Jemalloc;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
```

This will replace your entire allocator for the whole program with an allocator designed to minimize fragmentation, and make reallocation less painful. It also has various diagnostics if you feel like diving into them (we won't have time today).