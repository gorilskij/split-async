use split_async::split;
fn sync_fn() {}
fn async_fn() {}
fn sync_func() {
    sync_fn();
}
async fn async_func() {
    async_fn();
}
