# Tracing

The `tracing` system in Rust is useful in a lot of ways:

* It provides great logging.
* It can be linked to structured output, OpenTelemetry, or just printing results.
* It's highly customizable.
* It provides a built-in measurement system for tracking execution metrics.

Let's look at `code/benchmark/tracing_example`.

Tracing is most useful for the "large picture": You can gather metrics on performance for individual functions. Store them and analyze them over time to get a picture of performance. Are you meeting your goals 100% of the time? 99%? Never?