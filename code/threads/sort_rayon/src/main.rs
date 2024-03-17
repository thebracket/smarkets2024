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
