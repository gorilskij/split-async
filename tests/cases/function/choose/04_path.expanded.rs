use split_async::split;
mod part1 {
    #[split]
    async fn part1(x: usize) -> usize {
        x + 1
    }
}
mod part2 {
    mod nested {
        #[split(foo, bar)]
        async fn part2(x: usize) {
            x + 3
        }
    }
}
fn sync_func() {
    let part1 = part1::sync_part1;
    let part2 = part2::nested::foo;
    let x = part2(part1(0));
}
async fn async_func() {
    let part1 = part1::async_part1;
    let part2 = part2::nested::bar;
    let x = part2(part1(0).await).await;
}
