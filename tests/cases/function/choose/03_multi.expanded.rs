use split_async::split;
fn part1_v1(x: usize) -> usize {
    x + 1
}
fn part1_v2(x: usize) -> usize {
    x + 2
}
fn foo(x: usize) {
    x + 3
}
async fn bar(x: usize) {
    x + 3
}
fn sync_func() {
    let part1 = part1_v1;
    let part2 = foo;
    let x = part2(part1(0));
}
async fn async_func() {
    let part1 = part1_v2;
    let part2 = bar;
    let x = part2(part1(0).await).await;
}
