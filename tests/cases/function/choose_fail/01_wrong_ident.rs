#![no_main]

use split_async::split;

#[split]
async fn foo() {}

#[split]
async fn bar() {
    choose!(baz)
}
