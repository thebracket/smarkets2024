use std::thread::scope;

fn main() {
    scope(|scope| {
        for i in 0..10 {
            scope.spawn(move || {
                println!("Thread number {}", i);
            });
        }
    });
}
