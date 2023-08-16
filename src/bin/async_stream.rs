use async_stream_example::async_identity_v2;

use std::time::Duration;

use async_stream::stream;
use futures_util::Stream;

/// async_stream::streamマクロによってストリームを定義できる．
fn counter(max: u32, duration: Duration) -> impl Stream<Item = u32> {
    stream! {
        for i in 1_u32..(max + 1) {
            yield async_identity_v2(duration, i).await;
        }
    }
}

#[tokio::main]
async fn main() {
    use futures_util::{pin_mut, StreamExt};

    let stream_counter = counter(5, Duration::from_millis(100));
    pin_mut!(stream_counter); // async_streamは利用前にPinする必要がある．

    while let Some(count) = stream_counter.next().await {
        println!("count: {count}");
    }
}
