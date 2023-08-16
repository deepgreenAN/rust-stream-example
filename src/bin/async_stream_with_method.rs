use async_stream_example::async_identity_v1;

use std::time::Duration;

use async_fn_stream::fn_stream;
use futures_util::Stream;

#[derive(Clone)]
struct Counter {
    current: u32,
    max: u32,
    duration: Duration,
}

impl Counter {
    pub fn new(max: u32, duration: Duration) -> Self {
        Self {
            current: 0,
            max,
            duration,
        }
    }

    // 一度のイテレーションに対応するasync関数
    async fn async_next(&mut self) -> Option<u32> {
        if self.current < self.max {
            let res = async_identity_v1(self.duration, self.current + 1).await;
            self.current = res;
            Some(res)
        } else {
            None
        }
    }

    pub fn into_stream(mut self) -> impl Stream<Item = u32> {
        fn_stream(|emitter| async move {
            while let Some(item) = self.async_next().await {
                emitter.emit(item).await;
            }
        })
    }
}

#[tokio::main]
async fn main() {
    use futures_util::{pin_mut, StreamExt};

    let stream_counter = Counter::new(5, Duration::from_millis(100)).into_stream();
    pin_mut!(stream_counter);

    while let Some(count) = stream_counter.next().await {
        println!("count: {count}");
    }
}
