use split_async::split;
fn sync_func(x: usize) -> usize {
    fn nested() {
        let deep = || {};
    }
}
async fn async_func(x: usize) -> usize {
    async fn nested() {
        let deep = async { async || {} };
    }
}
