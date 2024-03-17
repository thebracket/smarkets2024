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