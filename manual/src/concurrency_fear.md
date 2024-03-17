# Fearful Concurrency

> Hands-up if threads have ever given you a headache. Preferably not including today!

Concurrency has a reputation for being *hard*. Rust makes it easier, and eliminates many of the most common bugs.

Let's start with some Python:

```python
#!/usr/bin/python3
import threading

counter = 0

def one():
    return 1

def adder():
    global counter
    for _ in range(1000000):
        counter = counter + one()

threads = []
for i in range(10):
    thread = threading.Thread(target=adder)
    thread.start()
    threads.append(thread)

for thread in threads:
    thread.join()

print(counter)
```

Note that Python 3.9+ has a little bit of race-condition protection in, but we fooled it by adding the `one()` function. Prior to 3.9, you didn't need that. What does this program return?

```
python3 racey.py
3638217
python3 racey.py
4307520
```

And so on --- the program rarely gives the same answer twice! This is called a *race condition*. It happens because adding to a variable is a multi-step process:

1. Retrieve the current value.
2. Add one to it.
3. Store the current value.

Any other thread may interrupt while this is happening, causing you to miss part of the task.

This is a **big** problem. Uber found several thousand race conditions in their systems! It's also one of the primary deterrents to writing concurrent code. It's just too easy to mess up, with no warning or errors.

You *can* write the same bug in Rust, if you try really hard:

```rust
use std::thread::scope;

fn main() {
    static mut COUNTER: usize = 0;
    scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                for _ in 0..1_000_000 {
                    unsafe {
                        COUNTER += 1;
                    }
                }
            });
        }
    });
    unsafe {
        println!("Counter: {COUNTER}");
    }
}
```

> PLEASE don't do this

Rust *will* let you have this bug, too! But you need `unsafe` everywhere to do it. If you try without unsafe, it simply won't compile.

Let's try this without `unsafe` tags:

```rust
use std::thread::scope;

fn main() {
    static mut COUNTER: usize = 0;
    scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                for _ in 0..1_000_000 {
                    COUNTER += 1;
                }
            });
        }
    });
    println!("Counter: {COUNTER}");
}
```

It won't compile with the error message "use of a mutable static requires an unsafe tag". Ok, so we've used a mutable static. Can we use a regular mutable reference?

```rust
use std::thread::scope;

fn main() {
    let mut counter: usize = 0;
    scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                for _ in 0..1_000_000 {
                    counter += 1;
                }
            });
        }
    });
    println!("Counter: {counter}");
}
```

This won't compile either, with the error message that you can't mutably borrow `counter` more than once. That's the key here: the borrow checker prevents race conditions.