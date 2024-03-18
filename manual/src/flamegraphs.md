# Flamegraphs

> This is Linux only at this present.

Install the tool with `cargo install flamegraph`.

Now you need to allow access to kernel performance events:

```bash
sudo sysctl kernel.perf_event_paranoid=-1
```

And finally, edit `Cargo.toml` so that release builds also emit some debug info (so functions can be named):

```toml
[profile.release]
debug = true
```

Now you can run `cargo flamegraph` and receive a full SVG dump of your execution profile.

(The flamegraph SVG is in `code/benchmark/flamegraph.svg`. It is best read in Firefox.)