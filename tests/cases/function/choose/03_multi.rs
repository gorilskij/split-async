use split_async::split;

// note that you can choose! between two sync functions
fn part1_v1(x: usize) -> usize {
    x + 1
}

fn part1_v2(x: usize) -> usize {
    x + 2
}

#[split(foo, bar)]
async fn part2(x: usize) {
    x + 3
}

#[split]
async fn func() {
    let part1 = choose!(part1_v1, part1_v2);
    let part2 = choose!(foo, bar);
    let x = part2(part1(0).await).await;
}
