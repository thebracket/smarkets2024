# Smart Pointers

When you first heard that Rust has "manual memory management", you probably feared writing something like this:

```rust
use std::alloc::{Layout, alloc, dealloc};

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout
}

impl <T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");

        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);

            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout
            }
        }
    }

    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }

    fn get(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }
}

impl <T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory from SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

fn main() {
    let mut my_num = SmartPointer::<i32>::new();
    my_num.set(12);
    println!("my_num = {}", my_num.get());
}
```

This program is pretty neat: it allocates whatever type you give it to the heap, and uses `Drop` to deallocate it when it leaves scope. Never forget to deallocate again.

## You Don't Have to Do this!!!

> But you can if you *need* to for some reason.

Instead of making you worry about direct allocations and removals, Rust has a built-in type for allocating to the heap: `Box`. This code does the same thing:

```rust
fn main() {
    let mut my_num = Box::new(0);
    *my_num = 12;
    println!("{my_num}");
}
```

`Box` is a *unique pointer* in C++ - it has a single owner, allocates to the heap, and will be automatically deleted as soon as it leaves scope. `Box` is everywhere; a `Vec` is a `Box` of a given capacity, a `String` is a vector of characters, etc. So if you need to store data larger than your stack, put it in a `Box`.