# Scoped Threads in Rust

There are two major methods for creating threads in Rust. Let's start with *scoped threads*. Similar to Rayon, threads spawned inside a scope are *guaranteed* to finish when the scope ends - so borrowing values from outside of the scope is easier, and there's no need to remember to wait for your threads.

You don't need any dependencies.

Here's a very basic example using two scoped threads:

```rust
use std::thread::scope;

fn main() {
    scope(|scope| {
        for i in 0..10 {
            scope.spawn(move || {
                println!("Thread number {}", i);
            });
        }
    });
}
```

> Note the mysterious `move`, which isn't really moving anything. We'll talk about that in a bit.

This program spawns 10 threads, and they each print a greeting. The order in which the greetings appear is up to your operating system.

> Scoped threads are great when you want to "fan out" to child threads, and be sure they are all done when you resume.