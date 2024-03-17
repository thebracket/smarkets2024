# Async

![](../images/crab-feathers.webp)


Async can run in one or more threads. The unit-of-work in an async environment is the "future" - a task that promises to accomplish some work. If you've used NodeJS or async in Python, you're familiar with this.

An async runtime maintains a list of futures that are either active, or waiting on something else. Multitasking is *cooperative*: a task yields control when it waits for something, or explicitly yields control. Tasks are scheduled by the *async runtime*.

Async tasks are *very* lightweight: you can have a *lot* of them (hundreds of thousands):

* Switching tasks just requires updating the future's status and saving the return point.
* Creating futures is lightweight. They allocate a few bytes for their local storage (depending upon how much you are storing).
* You can have hundreds of thousands, millions even of futures.

Async is *great* for heavy input-output operations, and for servers that spend a lot of time waiting for things (disks, databases, network, etc). Async is *not good* for CPU-heavy tasks, because if you don't explicitly yield control---other tasks won't get to go.

### Runtimes

Rust chose to be agnostic about the async runtime. You can pick from several - ranging from `smol` (tiny, embedded friendly) all the way up to `tokio` (one thread per core, shared task lists, work-stealing to minimize pausing, the Enterprise standard). There are specialized ones like `glommio` that integrate Linux's `io_uring` for blazing input-output performance.
