# The Incredible Power of Drop

C++ invented a pattern called `RAII` - "Resource Acquisition is Initialization". You can tell it's a C++ term, because it neither accurately describes what it does, nor rolls off the tongue!

RAII is built upon *destructors*. You have those in Python, too:

```python
# Python program to illustrate destructor
class Employee:
 
    # Initializing
    def __init__(self):
        print('Employee created.')
 
    # Deleting (Calling destructor)
    def __del__(self):
        print('Destructor called, Employee deleted.')
 
obj = Employee()
del obj
```

The problem here is that the destructor fires when you call `del`---but if you let the garbage collector do the destruction, it's going to fire at some point in the future when a) you're done with the object, and b) Python notices that you're done with the object. So destructors don't get a lot of use in Python.

In Rust, they are called `Drop`---and they are *everywhere*.

Let's build a toy example:

```rust
struct MyType {
    label: String
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping {}", self.label);
    }
}

fn main() {
    let var = MyType { label: String::from("MyType") };
}
```

If you run this, it prints `Dropping MyType`. The destructor runs because the `main` function is ending, and the variable it owns is no longer in scope.

How about if you *move* it?

```rust
struct MyType {
    label: String
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping {}", self.label);
    }
}

fn move_here(var: MyType) {
    println!("Ending move_here");
}

fn main() {
    println!("Starting");
    let var = MyType { label: String::from("MyType") };
    move_here(var);
    println!("Ending");
}
```

This prints:

```
Starting
Ending move_here
Dropping MyType
Ending
```

When you *move* the variable, it's owning scope changes to the `move_here` function - so when the variable leaves the function, it is dropped. Immediately, with no delay.

How about borrowing?

```rust
struct MyType {
    label: String
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping {}", self.label);
    }
}

fn borrow_here(var: &MyType) {
    println!("Ending move_here");
}

fn main() {
    println!("Starting");
    let var = MyType { label: String::from("MyType") };
    borrow_here(&var);
    println!("Ending");
}
```

Now you get:

```
Starting
Ending move_here
Ending
Dropping MyType
```

Ownership is never transferred into the `borrow_here` function---so the variable is destroyed at the end of `main()`.

What about if I want to kill it now?

```rust
struct MyType {
    label: String
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping {}", self.label);
    }
}

fn main() {
    let var = MyType { label: String::from("MyType") };
    std::mem::drop(var);
    println!("Program end");
}
```

`drop` lets you delete it immediately. If you try and use `var` after you drop it, the program won't compile. No "use after free" bugs here!