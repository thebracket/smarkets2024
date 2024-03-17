fn receiver(rx: std::sync::mpsc::Receiver<i32>) {
    while let Ok(received) = rx.recv() {
        println!("Got: {}", received);

        // Without this, the program will run forever
        if received == 10 {
            break;
        }
    }
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        receiver(rx)
    });

    for i in 1..=10 {
        tx.send(i).unwrap();
    }

    handle.join().unwrap();
}
