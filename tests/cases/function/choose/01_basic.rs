use split_async::split;

fn sync_fn() {}

fn async_fn() {}

#[split]
async fn func() {
    choose!("fn")();
}
