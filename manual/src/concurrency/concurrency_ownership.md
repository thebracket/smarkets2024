# Ownership

So where are we going with moving and borrowing? Rust is all about understanding *ownership*. A variable is *owned* somewhere, and when it falls out of scope (is no longer referenced) it is *dropped*. We'll talk about this in memory management, later this afternoon.

There's a reason behind the ownership system:

* The Rust compiler *always* knows where a variable actually is.
* The *borrow checker* strictluy enforces a compile-time rule that:
    * EITHER
        * You can have as many immutable (read-only) references as you want.
    * OR (exclusive)
        * You can have one mutable writer to a variable.

You *cannot* have more than one mutable writer to a variable, or a mutable writer while a reader exists. It won't compile.

That may sound like a strange restriction, but it allows Rust to offer Fearless Concurrency. Let's work through what that means.