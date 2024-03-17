use std::thread::scope;

fn main() {
    static mut COUNTER: usize = 0;
    scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                for _ in 0..1_000_000 {
                    unsafe {
                        COUNTER += 1;
                    }
                }
            });
        }
    });
    unsafe {
        println!("Counter: {COUNTER}");
    }
}
