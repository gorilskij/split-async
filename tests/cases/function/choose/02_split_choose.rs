use split_async::split;

#[split]
async fn sub_func() {}

#[split]
async fn func() {
    choose!(sub_func)();
}
