# You Don't Have to Implement Drop

Just a quick note: if you don't implement `Drop`, child entries are still dropped---you have the *option* to override it, not a
requirement. Not implementing it won't make you suddenly leak memory!

For example:

```rust
struct MyDropStruct;

impl Drop for MyDropStruct {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

struct MyNormalStruct {
    dropper: MyDropStruct,
}

fn main() {
    let s = MyNormalStruct { dropper: MyDropStruct };
}
```

Even though `MyNormalStruct` doesn't implement drop, it's contents are still deleted properly.