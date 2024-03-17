# Fearless Atomics

Rust wouldn't be very useful if it implemented threads really well and then didn't let you change anything with them. You *could* structure it so that each thread runs entirely indepenently and returns a value (in fact, if that fits your problem - you should). But sometimes you can't avoid the need to share.

But I just told you that you can't share borrows?

In this case, we're using an integer---and CPUs have primitives called "atomics". An atomic is a number designed to be safely shared. Instead of the usual 3-phase update (fetch, increment, store) it guarantees that other cores won't interrupt the process - so you don't end up with corrupt data.

Here's the same counter in Rust with atomics:

```rust
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicU32;

fn main() {
    let counter = AtomicU32::new(0);
    std::thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
        let t2 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
    });
    println!("{}", counter.load(Relaxed));
}
```

This will give the same answer every time. The syntax for atomics is a little more explicit, and there are only so many operations available---but they are *really* fast (CPU accelerated).

So *why* does this work, and a regular integer doesn't? Is it because Atomics are super-special (they are, but that's not the answer).

There are two traits (properties/interfaces) that are applied for you when applicable to all types in Rust---your own and built-in types:

* A type with `Send` can be sent between threads. Most types are `Send`.
* A type with `Sync` can be *read* safely across threads, immutably.

Ok, so `Sync` means it can be *read* across threads. But we're not just *reading*? The second half of this is *interior mutability*. A type that has interior mutability is immutable from *outside*---the type itself can be `Sync` and borrowed as much as you want.

Internally, it protects its contents from concurrent writes. Atomics do this with CPU intrinsics. Mutexes---the next slide---do this for anything.

So you are *immutably borrowing* the atomic---you just have a pointer to where it is. Then the atomic itself ensures that access obeys Rust's very strict rules---so it can safely declare itself `Sync`.