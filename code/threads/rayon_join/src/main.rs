fn a() -> u32 {
    let mut n = 0;
    for _ in 0..1_000_000 {
        n += 1;
    }
    n
}

fn b() -> u32 {
    let mut n = 0;
    for _ in 0..1_000_000 {
        n += 1;
    }
    n

}

fn main() {
    let (a,b) = rayon::join(a, b);
    println!("a: {a}, b: {b}");
}
