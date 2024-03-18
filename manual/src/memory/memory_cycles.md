# Cyclic References


Sometimes, you can get into a bit of a mess with `Arc` and `Rc`. Reference counting is notoriously bad at "cyclic references". That is, two `Rcs` that hold a pointer to one another.

```rust
use std::rc::Rc;

// Special type for single-threaded runtime mutability.
// A non-locking Mutex.
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let mut head = Rc::new(RefCell::new(Node { next: None }));
    let mut tail = Rc::new(RefCell::new(Node { next: None }));


    head.borrow_mut().next = Some(tail.clone());
    tail.borrow_mut().next = Some(head.clone());

    println!("Ending Program");
}
```

Notice that nothing is dropped! That's because the linked list appears to be infinite - it loops. Since each node is looking at the other, it'll *never* be dropped. We've made a memory leak!

> Rust does *not* include memory leaks in its memory safety options.

## So What Can We Do About It?

### Option 1: Don't Do This

The preferred option is to not do this. If you really need a linked list (and they are rarely the right choice), `std::collections::LinkedList` has one for you.

This my sound hand-waivey, but 9 times out of 10 not doing this is the preferred option. Usually ownership is either hierarchical (in which case you can have the top owning its children), or you can use an existing collection type.

### Option 2: Use a Weak Reference

```rust
use std::rc::Rc;
use std::rc::Weak;

// Special type for single-threaded runtime mutability.
// A non-locking Mutex.
use std::cell::RefCell;

struct Node {
    next: Option<Weak<RefCell<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let mut head = Rc::new(RefCell::new(Node { next: None }));
    let mut tail = Rc::new(RefCell::new(Node { next: None }));

    let weak_head = Rc::downgrade(&head);
    let weak_tail = Rc::downgrade(&tail);

    head.borrow_mut().next = Some(weak_head);
    tail.borrow_mut().next = Some(weak_tail);

    println!("Ending Program");
}
```

When you create a *weak* reference, it doesn't prevent parents from being dropped. So you do need to keep the original somewhere. But now you can safely have self-referential links, and call `upgrade` on the weak pointer if you want to access it; it returns a result, so if the original no longer exists you can act on that information.

It's still preferable to not have to do this!