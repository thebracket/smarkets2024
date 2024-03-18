# Whole Program with Hyperfine

It's not Rust specific, but `hyperfine` is a great way to benchmark a whole program. You can use it as follows:

```bash
hyperfine yourprogram
```

For example:

```bash
hyperfine "python3 fib.py"
```

This yields:

```
Benchmark 1: python3 fib.py
  Time (mean ± σ):     11.312 s ±  0.100 s    [User: 11.305 s, System: 0.004 s]
  Range (min … max):   11.134 s … 11.425 s    10 runs
```

Or

```bash
hyperfine fib_rayon
```

Which yields:

```
Benchmark 1: ../../target/release/fib_rayon
  Time (mean ± σ):     124.7 ms ±   6.0 ms    [User: 334.3 ms, System: 3.7 ms]
  Range (min … max):   119.9 ms … 142.6 ms    20 runs
```

This is always more accurate than running the program once---it gives the cache a chance to warm up, and repeated runs highlight outliers.

Hyperfine isn't a very granular benchmark either, but it's a good way to compare larger systems *quickly* when you want a performance answer.