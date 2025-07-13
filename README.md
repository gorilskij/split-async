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
