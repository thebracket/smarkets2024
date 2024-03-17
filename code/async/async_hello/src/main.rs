async fn say_hello() {
    println!("Hello, world!");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    say_hello().await;
}
