use async_stream_example::async_identity_v1;

use std::time::Duration;

use async_fn_stream::fn_stream;
use futures_util::Stream;

/// async_fn_stream::fn_streamによってStreamを作成できる
fn counter(max: u32, duration: Duration) -> impl Stream<Item = u32> {
    fn_stream(|emitter| async move {
        for i in 1..max + 1 {
            emitter.emit(async_identity_v1(duration, i).await).await;
        }
    })
}

#[tokio::main]
async fn main() {
    use futures_util::{pin_mut, StreamExt};

    let stream_counter = counter(5, Duration::from_millis(100));
    pin_mut!(stream_counter); // async_fn_streamは利用前にPinする必要がある．

    while let Some(count) = stream_counter.next().await {
        println!("count: {count}");
    }
}
