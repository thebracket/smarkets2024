# Threaded Shared Ownership

An `Rc` uses a simple unsigned integer to count references. This isn't thread-safe! The `Arc` --- Atomic Reference Count --- type provides thread safety. It uses an *atomic* for the reference count; so it is slightly slower (but still really fast)---and completely safe to use between threads. You saw an `Arc` in our threads demo.

Let's combine interior mutabilty and an `Arc` to make a system that represents some complex processing:

```rust
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct ImportantData {
    subset_1: Mutex<i32>,
    subset_2: Mutex<i32>,
}

impl ImportantData {
    fn new() -> Self {
        Self {
            subset_1: Mutex::new(5),
            subset_2: Mutex::new(3),
        }
    }
}

impl Drop for ImportantData {
    fn drop(&mut self) {
        println!("Destroying important data");
    }
}

fn main() {
    // Some important data has entered the pipeline for processing
    let new_data = Arc::new(ImportantData::new());
    std::thread::scope(|scope| {
        scope.spawn(|| {
            println!("Pipeline 1 working on subset_1");
            let mut lock = new_data.subset_1.lock().unwrap();
            *lock = 20;
        });

        scope.spawn(|| {
            println!("Pipeline 2 working on subset_1");
            let mut lock = new_data.subset_2.lock().unwrap();
            *lock = 30;
        });
    });

    println!("{new_data:?}");
}
```

This pulls together a lot of what we've learned:

* `ImportantData` is `Sync` because all of the fields are protected by mutexes - but there are more than one, so you can work on different parts of it concurrently without pausing.
* `new_data` is an `Arc`. It is heap allocated, shared, and reference counted.
* We've passed our data into multiple threads, which independently work on it.
* The Rust compiler ensures that we *can't* accidentally have a race condition.
* The `Arc` guarantees that memory won't be leaked when it's done.