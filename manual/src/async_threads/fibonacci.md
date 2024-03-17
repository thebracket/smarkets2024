# Fibonacci Numbers

Fibonacci numbers form a sequence where each number is the sum of the two preceding ones, starting from 0 and 1.

Fibonacci numbers are also a good performance test, because generating them the "obvious" recursive way can be pretty slow. When you're testing performance with threads, it's a good idea to have a workload that will keep your CPU warm!

> The code for this is in `code/threads/fib_sync`

Here's a single-threaded Fibonacci program:

```rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    // Shorthand: put 0 through 39 into a vector (list).
    let targets: Vec<u64> = (0 .. 40).collect();

    // Start the timer
    let now = std::time::Instant::now();
    let results: Vec<u64> = targets
        // Iterate through each target
        .iter()
        // Map transforms each entry into the results of the function call
        .map(|n| fibonacci(*n))
        // Collect gathers the results into a collection
        .collect();

    // Collect how much time has passed
    let elapsed = now.elapsed();

    // Print the result
    println!("Completed in {} seconds.", elapsed.as_secs_f32());
    println!("{results:?}");
}
```

Calculating the first 40 Fibonacci numbers in `debug` mode took 0.93 seconds on my laptop. In optimized mode (`cargo run --release`) it's a much more reasonable 0.31 seconds.

The online Rust Playground averages about 2.39 seconds for this task.

Not terrible---but we can do better!