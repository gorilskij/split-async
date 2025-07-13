use split_async::split;
fn sync_func1(x: usize) -> usize {
    let inc = |y| y + 1;
    inc(x)
}
async fn async_func1(x: usize) -> usize {
    let inc = async |y| y + 1;
    inc(x).await
}
fn sync_func2(x: usize) -> usize {
    {
        let x = || {};
    }
}
async fn async_func2(x: usize) -> usize {
    async {
        let x = async || async {}.await;
    }
}
