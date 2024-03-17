# Parallel Fibonacci

Rayon adds "parallel iterators" to make this easy to paralleize. We can parallelize this by adding a dependency to rayon:

```bash
cargo add rayon
```

And replacing one line of code:

> The code for this is in `code/threads/fib_rayon`

```rust
// Import the "prelude" (commonly used functions) from Rayon
use rayon::prelude::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let targets: Vec<u64> = (0 .. 40).collect();
    let now = std::time::Instant::now();
    let results: Vec<u64> = targets
        .par_iter() // <--- Use `par_iter` instead of `iter`
        .map(|n| fibonacci(*n))
        .collect();
    let elapsed = now.elapsed();
    println!("Completed in {}", elapsed.as_secs_f32());
    println!("{results:?}");
}
```

Rayon creates a 1-thread per CPU thread-pool by default, and divides tasks automatically (with work-stealing so no thread goes idle). My `debug` performance improved to 0.46 seconds, and my `release` performance is up to 0.122 seconds.

Note that you'll see limited improvement in the Rust Playground---it doesn't offer many CPUs!

So with two lines of code, we've parallelized the whole calculation. The performance increase is pretty good---not the best you could achieve, but it's hard to beat using two lines of code!

> Rayon is a great choice when you have a readily parallized algorithm. It can often do it for you!

Most of the commonly used iterator functions are [available](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#provided-methods).