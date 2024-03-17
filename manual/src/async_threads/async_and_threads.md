# Why Not Both?

![](../images/crabs-together.webp)

You *can* mix and match threads and async! This is especially helpful if your workload is mostly CPU-heavy, but you'd like an async interface to the rest of your service stack.

There are two common patterns here:

## Async and Threaded Separately

* Run a single thread with an async engine.
* That engine sends/receives messages (instructions).
* The engine sends instructions into *channels* to tell the threaded architecture what to do.

> This is a great fit for a mostly threaded system that needs to maintain high-performance communication with other systems.

## All Tokio, All the Time

* Run Tokio in "multi thread" flavour.
* Tokio makes one thread per CPU core, with a task list for each thread.
* Task lists can "work steal" from one another - reducing the risk of bogging down.
* Tasks that are going to take a while use `spawn_blocking` to run inside Tokio's thread pool.

> This is a great fit for a service architecture focused on performance, that sometimes needs to perform heavy calculation in threads.