use split_async::split;
fn sync_sub_func() {}
async fn async_sub_func() {}
fn sync_func() {
    sync_sub_func();
}
async fn async_func() {
    async_sub_func();
}
