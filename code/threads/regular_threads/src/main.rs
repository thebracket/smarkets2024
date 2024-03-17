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
