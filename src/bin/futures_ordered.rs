use async_stream_example::async_identity_v2;
use std::time::Duration;

use futures_util::stream::FuturesOrdered;
use futures_util::Stream;

/// 状態をイテレーションさせるわけではなく、全てのFuture(同じ型)を独立して実行するストリームを作成．
/// つまりjoin_allのストリーム版．以下では一度のdurationの後全て実行される．
fn counter(max: u32, duration: Duration) -> impl Stream<Item = u32> {
    let mut ordered = FuturesOrdered::new();

    for i in 1..max + 1 {
        ordered.push_back(async move { async_identity_v2(duration, i).await });
    }

    ordered
}

#[tokio::main]
async fn main() {
    use futures_util::StreamExt;

    let mut stream_counter = counter(5, Duration::from_millis(1000));

    while let Some(count) = stream_counter.next().await {
        println!("count: {count}");
    }
}
