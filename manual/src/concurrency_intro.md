# Safe Concurrency

Rust makes a really big deal about claiming *fearless concurrency*. Fearless comes in two flavours:

* It's not hard to use threads or async code. (We just did some!)
* Rust tries really hard to make it difficult to shoot yourself in the foot.

Before we dive into how this works, let's start with a couple of fundamental Rust concepts.