use split_async::split;

#[split]
async fn func(x: usize) -> usize {
    async fn nested() {
        let deep = async { async || {} };
    }
}
