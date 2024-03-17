# Threads and Async

Use **Rayon threads** when you want:

* Simple code
* Easy parallelization of tasks
* Your problem is embarrassingly parallel and fits into the Rayon iterator model

Use **threads** when you want:

* Maximum CPU throughput
* You can keep the number of threads manageable
    * 1 per CPU core is ideal
    * Maximum is around 60,000---operating system dependent.

Use **async** when you want:

* Maximum input/output speed
* Accept that the network, database, file I/O will cause latency
* Convert that latency into throughput by letting other tasks run while you wait

And in an ideal world: mix and match as needed, with either `spawn_blocking` or channels to give commands to threads. You can even use `rayon` inside async functions.