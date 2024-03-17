# Move by Default

One of the first things that surprises everyone who comes to Rust is "move by default". Let's take a look!

You'd normally expect a simple program like this to Just Work(TM):

```rust
fn print(s: String) {
    println!("{s}");
}

fn main() {
    let my_string = String::from("Hello");
    print(my_string);
    println!("{my_string}");
}
```

It doesn't even compile! What's up with that?

**Rust is "move by default"** : When you pass an undecorated variable to a function, the variable *moves* into the function and no longer exists in the parent scope!

You *could* get around this by moving it right back out again:

```rust
fn print(s: String) -> String {
    println!("{s}");
    s
}

fn main() {
    let my_string = String::from("Hello");
    let my_string = print(my_string);
    println!("{my_string}");
}
```

This compiles, but would make for a pretty tedious programming experience (not to denegrate some functional languages in which is the norm!).

What Rust is *really* doing is keeping track of "ownership". `main` declared `my_string`, so it belongs to `main`. *Moving* it into `print` means that `print` now *owns* it. It's up to `print` to destroy it (no manual destruction needed - we'll talk about that later).

But what if you want `print` to just *use* your string? You can *lend* it to other functions, who in turn *borrow* it (hence the name "borrow checker"):

```rust
fn print(s: &String) {
    println!("{s}");
}

fn main() {
    let my_string = String::from("Hello");
    print(&my_string);
    println!("{my_string}");
}
```

Notice that we've had to add `&` to *lend*, and `&` to indicate that we know we're *borrowing*. Rust is strict about this: you can't change a function signature, forget to update the caller and have odd things happen!

So ownership now remains with `main` the whole time.

You can even let functions change/*mutate* your loaned variable:

```rust
fn print(s: &mut String) {
    s.push_str(" World");
    println!("{s}");
}

fn main() {
    let mut my_string = String::from("Hello");
    print(&mut my_string);
    println!("{my_string}");
}
```

Notice again Rust is being a little pedantic:

* We had to add `mut` to the variable itself to allow it to be changed.
* We had to indicate both that we're *lending* and *borrowing* mutably.

> The need for explicit borrowing like this is a hangover from C++, where you can change the function and not remember to change the caller---and suddenly copy gigabytes of memory for no good reason.