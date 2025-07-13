use split_async::split;

#[split]
async fn func1(x: usize) -> usize {
    let inc = async |y| y + 1;
    inc(x).await
}

#[split]
async fn func2(x: usize) -> usize {
    async {
        let x = async || async {}.await;
    }
}
