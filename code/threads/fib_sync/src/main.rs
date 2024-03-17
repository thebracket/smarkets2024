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
        .iter()
        .map(|n| fibonacci(*n))
        .collect();
    let elapsed = now.elapsed();
    println!("Completed in {}", elapsed.as_secs_f32());
    println!("{results:?}");
}