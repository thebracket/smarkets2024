# The Simplest Benchmark

The quick'n'dirty approach is to add a little benchmark inline. There's nothing wrong with doing this!

```rust
fn main() {
    let now = std::time::Instant::now();
    let mut n = 0;
    for i in 0 .. 1_000_000 {
        n += 1;
    }
    let elapsed = now.elapsed();
    println!("Execution ran in: {} seconds", elapsed.as_secs_f32());
}
```

Use this technique when you are developing and want some instant feedback on a quick test. It's very simple --- and that's the point. You can get up and running really quickly with it.