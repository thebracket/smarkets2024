# Shared Ownership

Sometimes, ownership is tricky. You may pass data in divergent paths, and it's not at all clear *exactly* where you ownership ends. Something may even be logically owned by several tasks---but you want to only have one of them.

Rust isn't a garbage collected language, but you can *opt in* to using reference counted types for this purpose. Reference counted types (`Rc`) are designed to:

* Include a "reference counter" that lists how many times the object is owned.
* When you `clone` an `Rc` you don't actually clone it - the reference count is increased.
* When an `Rc` is dropped, the reference count decreases---it's only dropped when nobody is using it anymore.
* `Rc` is immutable and *not* `Send` (use `Arc` for that - the next page). But you can wrap an `Rc` or `Arc` around a sync type to allow for multiple parts of your program to edit it.

```rust
use std::rc::Rc;

struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn move_me(s: Rc<MyStruct>) {
    println!("Function run");
}

fn main() {
    let shared = Rc::new(MyStruct);
    move_me(shared.clone());
    println!("Program ended");
}
```

Even though we *clone* the structure, it only drops once. It's shared. You can have as many clones as you want---it will only be dropped when its done. Use `Rc` (and `Arc`) for shared data, or where it isn't clear where ownership goes.
