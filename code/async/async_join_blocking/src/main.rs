async fn looper(n: i32) {
    tokio::task::spawn_blocking(move || {
        for i in 0..10 {
            println!("[{n}]: looper: {}", i);
        }
    }).await.unwrap();
}

#[tokio::main()]
async fn main() {
    tokio::join!(looper(1), looper(2), looper(3));
}