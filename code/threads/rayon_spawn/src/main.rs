fn do_something(n: i32) {
    for i in 0..n*10 {
        println!("{n} - do_something: {}", i);
    }
}

fn main() {
    rayon::scope(|scope| {
        for i in 0..10 {
            scope.spawn(move |_| {
                do_something(i);
            });
        }
    });
}
