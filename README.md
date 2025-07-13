# split-async

**Why bother writing similar code twice for blocking and async code?**

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
<!-- [![Latest Version](https://img.shields.io/crates/v/maybe-async.svg)](https://crates.io/crates/split-async) -->
<!-- [![split-async](https://docs.rs/split-async/badge.svg)](https://docs.rs/split-async) -->

This is a modified version of the [`maybe-async`](https://crates.io/crates/maybe-async) crate by Guoli Lyu.

When implementing both sync and async versions of a function, the implementations tend to be
very similar, with the exception of the `async` and `.await` keywords.

This crate allows you to write just the `async` implementation of your function, a blocking
version is then generated automatically.

Features:
- **Split** your function into a blocking and an `async` version
- Within your function, use other **split** functions by **choosing** which version to use

### Examples

Here, `loop_body` can be arbitrarily complex and does not need to be duplicated.
It can call either `sync_operation` or `async_operation` as appropriate, which
can have completely different functionality.
The loop mechanics are different so we write them individually in two small wrapper
functions `sync_loop` and `async_loop` which can be used dynamically at will.
```rust
fn sync_operation(x: usize) -> usize { ... }

async fn async_operation(x: usize) -> usize { ... }

#[split]
async fn loop_body(x: usize) -> usize {
    // arbitrarily complex async logic
    choose!(operation)(x)
}

fn sync_loop() -> Vec<usize> {
    (0..100)
        .map(|x| sync_loop_body(x))
        .collect()
}

async fn async_loop() -> Vec<usize> {
    futures::stream::iter(0..100)
        .map(|x| sync_loop_body(x))
        .collect()
}
```
