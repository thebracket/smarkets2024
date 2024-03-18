# Hot and Cold Path Prediction

Modern CPUs have really impressive branch prediction, and will often start to speculatively run a likely branch.

In C and C++, there are intrinsics to indicate the "likely" path of execution. Rust has these, but only in "nightly" unstable builds at this time. Instead, a convention has appeared.

For a "hot path" function, add the inline tag:

```rust
#[inline]
fn my_hot_function() {
    // Likely
}

#[cold]
fn my_cold_function() {
    // Unlikely
}

fn main() {
}
```

You usually don't want to *force* inlining with `#[inline(always)]` unless you are unhappy with the inlining decisions the compiler makes (it takes into account instruction cache sizes). Adding the `#[inline]` tag with no specifier is a *hint*---and the optimizer is more likely to decide to use that inline candidacy.