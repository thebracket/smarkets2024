# Safe Concurrency Wrap

So Rust concurrency is fearless because it's relatively easy to use---and *it won't let you forget your locks and make a data race*. Data races won't compile, so your precious data remains intact.

> It's always better to fail to compile than give a wrong answer! It's even better to crash than give a wrong answer. You can diagnose those; you can spend a really long time figuring out why a number isn't quite right... hopefully you noticed!