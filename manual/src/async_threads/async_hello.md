# Working with Async

Async in Rust requires a runtime (it provides task scheduling, IO interaction and scheduling). So let's make a project with `tokio` as the runtime:

```bash
cargo add tokio -F full # The feature flag "full" enables all the features
```

Now we'll make a simple "Hello, World" in async Rust:

```rust
async fn say_hello() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}
```

Things to note:

* We've decorated `main()` with `#[tokio::main]`. This macro wraps the actual invocation of Tokio for you. It's really creating a Tokio runtime and calling a function called `block_on`.
* The `async` keyword on functions. This is syntax sugar that changes your function to return a `Future` type.
* So `say_hello()` doesn't *do* anything other than create a `Future` variable.
* The `Future` doesn't *run* until you `await` it.

`.await` does several things:

* It adds your future to the list of tasks awaiting execution.
* It pauses your current function, noting the return location.
* The task queue continues to the next task.

> If you don't `await` (or otherwise execute) a `Future`---*nothing happens*.

## Single or Multi-Threaded

Async can work on a single thread---it just maintains the task queue. Here's the same program in single-threaded mode:

```rust
async fn say_hello() {
    println!("Hello, world!");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    say_hello().await;
}
```

Staying in single-thread mode, let's use `join!` to spawn a few tasks at once and wait for them:

```rust
async fn looper(n: i32) {
    for i in 0..5 {
        println!("[{n}]: looper: {}", i);
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::join!(looper(1), looper(2), looper(3));
}
```

This results in serialized execution: tasks 1 through 3 run in order. This illustrates the biggest problem with async code: *blocking*. If your task keeps running, it doesn't allow other tasks to execute. Now let's explicitly tell teach task to yield control:

```rust
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
```

Now the tasks run in-turn. You are explicitly telling each task to give up control of the CPU and let the next task run. You *aren't* paying for threads (in scheduling time or overhead), but you *are* cooperatively multitasking.

You can also use `spawn_blocking` if you need to run a task in a thread. Tokio maintains a thread pool, so you have some of the advantages of Rayon (you pay for thread creation up front):

```rust
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
```

> Note that we had to remove `current_thread` to allow multiple threads to run. Otherwise Tokio assumes you *really* mean it when you say you want to use one thread!

## So Why Would You Do This?

Async really shines for input/output. Let's take a look at this program:

```rust
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
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mut set = tokio::task::JoinSet::new();
    for i in 0 .. 1000 {
        set.spawn(tcp_client(i));
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

Walking through it:

* We're sticking to single-threaded mode. No threads here.
* We spawn the `tcp_server`
    * In turn, it listens on 127.0.0.1:3001 (TCP).
    * When a connection arrives, it spawns a new task - `server_receive`.
        * In turn, `server_receive` prints how many bytes it received.
        * It then replies with the same message.
* `tcp_client` connects to the server, and sends it a message. It prints the reply.
* We use `JoinSet`, a Tokio helper for spawning lots of tasks at once.

So in 41 lines of code, we've made a TCP server and a TCP client. It doesn't do much - but it is very fast.

Now let's build it with `cargo build --release`. We'll measure it with:

```bash
/usr/bin/time -v ../../target/release/async_tcp # -l on Macs
```

The maximum resident set size was 4,640 bytes! And it peaked at 3% CPU usage. So very little code, and *very* lean and mean network code. Plus, it all ran in a single thread.

Switching off the single-thread mode ups CPU usage to 20% - as it could use many cores, but remained very small (4640 bytes resident set).

> Conclusion: Rust Async offers extremely lean, mean, high-performance IO.