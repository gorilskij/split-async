#![no_main]

use split_async::split;

#[split(a, b, c)]
async fn func() {}
