# Vector Allocation

You can `push` into vectors all the time---they don't have a set capacity (the limit is your platform's pointer size). BUT, just like Python lists they reserve a set capacity---and double it when they run out.

Let's add a million items to a vector:

```rust
fn main() {
    let now = std::time::Instant::now();
    let mut v = Vec::new();
    for i in 0..1_000_000 {
        v.push(i);
    }
    println!("{} s", now.elapsed().as_secs_f32());
}
```

We know we're adding a million items, so let's tell the Vector up-front that it should have capacity for a million items!

```rust
fn main() {
    let now = std::time::Instant::now();
    let mut v = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        v.push(i);
    }
    println!("{} s", now.elapsed().as_secs_f32());
}
```

On my workstation, the first takes 0.02 seconds---and the second 0.01 seconds. Not a huge difference, but if you were allocating large structures it can make a *world* of difference.