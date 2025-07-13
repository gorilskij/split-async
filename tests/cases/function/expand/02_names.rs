use split_async::split;

#[split(a)]
async fn func() {}

#[split("b")]
async fn func() {}

#[split(c, d)]
async fn func() {}

#[split(e, "f")]
async fn func() {}

#[split("g", h)]
async fn func() {}

#[split(i, j)]
async fn func() {}
