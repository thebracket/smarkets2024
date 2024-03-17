# Parallel Sorting


Rayon also includes parallel versions of the regular Rust sort operations:

```rust
use rayon::prelude::*;

fn main() {
    const CAPACITY: usize = 1_000_000;
    let mut random = Vec::with_capacity(CAPACITY);
    for _ in 0..CAPACITY {
        random.push(rand::random::<u64>());
    }

    let now = std::time::Instant::now();
    let mut v = random.to_vec();
    v.sort();
    let elapsed = now.elapsed();
    println!("Regular Sort of {CAPACITY} Numbers Completed in  {}", elapsed.as_secs_f32());


    let now = std::time::Instant::now();
    let mut v = random.to_vec();
    v.par_sort();
    let elapsed = now.elapsed();
    println!("Parallel Sort of {CAPACITY} Numbers Completed in {}", elapsed.as_secs_f32());
}

```

On my work computer, I get the following results:

```
Regular Sort of 1000000 Numbers Completed in  0.055435345
Parallel Sort of 1000000 Numbers Completed in 0.0088947
```

> Note that the parallel sort can be *slower* than a regular sort for small data-sets!
