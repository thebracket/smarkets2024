# System Threads

A thread is a full *operating-system thread*. It appears as a schedulable item in the OS, has its own stack (but shares program memory).

![](../images/crab-lifting.webp)

Threads are relatively heavy-weight:

* Creating a thread is expensive:
    * The OS has to create the schedulable thread.
    * The OS has to allocate a stack for the thread.
* Threads context-switch regularly, typically every 16ms.
    * Upside: you don't need to write fancy code to use them.
    * Downside: they can scale equally with your CPU cores, sometimes even if they are waiting for something.
* You can't create more than a few thousand threads. Linux shows 60k as the limit on my machine.
* Switching between threads is also relatively expensive:
    * Threads take it in turns, per-core.
    * The current thread is suspended (registers saved, execution pointer saved, etc.)
    * The new thread is restored.
    * The new thread executes until the OS decides its another items turn.
    * Repeat.

> Threads are *great* for sustained CPU-heavy execution. They are rescheduled infrequently, and are designed for crunching.

## An Old Adage

> I had a problem, so I used 10 threads. Now I have 10 brpolems.

Threading is considered tricky - and rightly so. You're juggling multiple tasks that share the same address space. Rust makes it a *lot easier* to not suffer race conditions, or forget to synchronize your data.