# Threading with Rayon

Let's start by looking at Rayon. Rayon is sometimes called "easy mode" by Rust programmers. That's not a bad thing: sometimes you want easy---especially if its blazing fast, and resource efficient at the same time!

Rayon is a crate that is widely used in the Rust ecosystem (and directly led to some core Rust features). Rayon:

* Builds a thread pool (one thread per available CPU core by default) on start-up. You pay for thread creation once.
* Implements *parallel iterators* to let you take normal Rust code and easily run it in parallel---*if* your problem is readily parallelizable.
* Provides lots of helpers such as parallel sorting.
* Implements a scoped-threading task system, similar to the standard library---but remaining inside the global thread pool so you don't oversubscribe your tasks.

Rayon isn't the answer to everything: it's bad at making a long-running thread that you want to keep working while your program does other things. But its a great place to start, and to get a feel for how Rust can make threading feel easy.
