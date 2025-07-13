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

#[split]
async fn func() {
    let part1 = choose!(part1::part1);
    let part2 = choose!(part2::nested::foo, part2::nested::bar);
    let x = part2(part1(0).await).await;
}
