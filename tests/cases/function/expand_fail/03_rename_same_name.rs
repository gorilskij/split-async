#![no_main]

use split_async::split;

#[split(a, a)]
async fn func() {}
