## What Else can it Do?

Rayon also includes mechanisms to:

### Join

`join` lets you spawn two threaded tasks and receive the results when they both finish. Not to be confused with the regular `join` command---the naming is unfortunate.

```rust
fn a() -> u32 {
    let mut n = 0;
    for _ in 0..1_000_000 {
        n += 1;
    }
    n
}

fn b() -> u32 {
    let mut n = 0;
    for _ in 0..1_000_000 {
        n += 1;
    }
    n

}

fn main() {
    let (a,b) = rayon::join(a, b);
    println!("a: {a}, b: {b}");
}
```

## Thread Scopes

You can also use Rayon to spawn your own tasks. This is just like Rust's regular thread scoping, but spawns tasks inside the Rayon tasks-list.

```rust
fn do_something(n: i32) {
    for i in 0..n*10 {
        println!("{n} - do_something: {}", i);
    }
}

fn main() {
    rayon::scope(|scope| {
        for i in 0..10 {
            scope.spawn(move |_| {
                do_something(i);
            });
        }
    });
}
```

It is guaranteed that all tasks will complete before the scope terminates.