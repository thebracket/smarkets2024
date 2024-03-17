async fn looper(n: i32) {
    for i in 0..5 {
        println!("[{n}]: looper: {}", i);
        tokio::task::yield_now().await;
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::join!(looper(1), looper(2), looper(3));
}