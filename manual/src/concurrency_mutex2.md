# Spot The Difference

Now let's look at `code/mutex_oops.py`. Can you spot the difference?

```python
#!/usr/bin/python3
from threading import Thread, Lock

counter = 0
mutex = Lock()

def one():
    return 1

def adder():
    global counter
    for _ in range(1000000):
            counter = counter + one()

threads = []
for i in range(10):
    thread = Thread(target=adder)
    thread.start()
    threads.append(thread)

for thread in threads:
    thread.join()

print(counter)
```

> In case you missed it, `with mutex:` is gone from the second example.

The problem with mutexes in most languages is that you have to remember to use them. It's really easy to forget one lock, and *boom* - you have a race condition.

Let's look at a Rust mutex setup:

```rust
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    std::thread::scope(|scope| {
        let my_counter = counter.clone();
        let t1 = scope.spawn(move || {
            for _ in 0 .. 1000000 {
                let mut lock = my_counter.lock().unwrap();
                *lock += 1;
            }
        });

        let my_counter = counter.clone();
        let t2 = scope.spawn(move || {
            for _ in 0 .. 1000000 {
                let mut lock = my_counter.lock().unwrap();
                *lock += 1;
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    let lock = counter.lock().unwrap();
    println!("{}", *lock);
}
```

> We'll talk about `Arc` later. For now, think of it as an easy way to share data across threads.

The basics should look very familiar: we create a mutex and lock it (and `unwrap` the result, because Rust mutexes can tell you if a thread crashed while it held the mutex). But notice the significant difference:

```rust
let counter = Arc::new(Mutex::new(0));
```

We've *wrapped the mutex around the data*. You *cannot* forget to lock the mutex, because locking is the only way to get to the data!