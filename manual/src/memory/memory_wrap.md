# Wrap Up

Rust isn't a garbage collected language, but that doesn't mean you have to struggle with allocation, deallocation and cleaning up. You can even add a form of garbage collection (reference counting) easily should you need it.

Rust makes it very hard to leak memory (unless you use the built-in commands for it!). Unless you *need it*, you don't normally need to think about the low-level allocation side of things---but you do need to think about ownership:

* If its *mine*, stick to the stack or a `Box`. Single ownership.
* If you *send it to someone*, use the move system.
* If you want another function to use it, but not change it - use `&` borrows.
* If you want another function to be able to change it, use `&mut`.
* If you are going to share across threads, you *must* use `Rc`, `Arc` and a locking mechanism such as `Mutex`, `RwLock`, etc.

So you *do* have to think a bit more about ownership - but the Rust compiler won't let you really break things.

One last tip: try to avoid holding onto references. They are really useful, but a reference shouldn't be forever. Rust's lifetime system makes this complicated, and most of the time you can resolve it with either a shared structure (if it is really shared) or an ownership hierarchy.