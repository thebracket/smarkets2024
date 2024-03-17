use tokio::{net::TcpListener, spawn, io::{AsyncReadExt, AsyncWriteExt}};

async fn tcp_server() -> anyhow::Result<()> {
    println!("Listening on 127.0.0.1:3001");
    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    loop {
        let (socket, _address) = listener.accept().await?;
        spawn(server_receive(socket));
    }

}

async fn server_receive(mut socket: tokio::net::TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await?;
    println!("Server received: {n} bytes");
    socket.write_all(&buffer[0..n]).await?;
    Ok(())
}

async fn tcp_client(n: i32) -> anyhow::Result<()> {
    let mut socket = tokio::net::TcpStream::connect("127.0.0.1:3001").await?;
    let message = format!("Hello, world! from client {n}");
    socket.write_all(message.as_bytes()).await?;
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await?;
    println!("Echoed message: {}", std::str::from_utf8(&buffer[0..n])?);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Start the TCP server
    spawn(tcp_server());
    let mut set = tokio::task::JoinSet::new();
    for i in 0 .. 1000 {
        set.spawn(tcp_client(i));
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
