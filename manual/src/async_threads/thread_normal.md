# Regular Threads in Rust

The building block for threads is `std::thread`. Here's the "hello world" of regular threads:

```rust
fn main() {
    let handle1 = std::thread::spawn(|| {
        println!("Thread number 1");
    });
    let handle2 = std::thread::spawn(|| {
        println!("Thread number 2");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

Threads keep running until either you `join` them, or your program stops. If you take the `join` calls away, there's no guaranty that your program will do anything at all!

## Retrieving Data from Threads

Join handles also let you retrieve results from threads:

```rust
fn double_it(n: i32) -> i32 {
    n * 2
}

fn main() {
    let handle = std::thread::spawn(|| {
        double_it(5)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}
```

## Sending Data Between Threads with Channels

Channels are Go's secret weapon, so Rust implemented them too.

```rust
fn receiver(rx: std::sync::mpsc::Receiver<i32>) {
    while let Ok(received) = rx.recv() {
        println!("Got: {}", received);

        // Without this, the program will run forever
        if received == 10 {
            break;
        }
    }
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        receiver(rx)
    });

    for i in 1..=10 {
        tx.send(i).unwrap();
    }

    handle.join().unwrap();
}
```

The *great* part is that you can use any type. So you can send enumerations to issue commands, even send function pointers (that's part of Rayon's magic trick!).