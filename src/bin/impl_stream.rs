use async_stream_example::async_identity_v1;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::Stream;

struct Counter {
    current: u32,
    max: u32,
    duration: Duration,
    temp_future: Option<Pin<Box<dyn Future<Output = u32>>>>, // 可変参照を利用するならPin<Box>とする
}

impl Counter {
    pub fn new(max: u32, duration: Duration) -> Self {
        Self {
            current: 0,
            max,
            duration,
            temp_future: None,
        }
    }
}

impl Stream for Counter {
    type Item = u32;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            // 一時的にtemp_futureを取り出す
            let temp_fut = self.as_mut().temp_future.take();

            match temp_fut {
                // 既にFutureが登録されていた場合
                Some(mut temp_fut) => {
                    match temp_fut.as_mut().poll(cx) {
                        Poll::Ready(next_count) => {
                            self.as_mut().current = next_count;
                            Poll::Ready(Some(next_count))
                        }
                        Poll::Pending => {
                            // まだ利用するため取り出したtemp_futureを元に戻す
                            self.as_mut().temp_future = Some(temp_fut);
                            Poll::Pending
                        }
                    }
                }
                // Futureが登録されていない場合
                None => {
                    let mut temp_fut =
                        Box::pin(async_identity_v1(self.duration.clone(), self.current + 1))
                            as Pin<Box<dyn Future<Output = u32>>>;

                    match temp_fut.as_mut().poll(cx) {
                        Poll::Ready(next_count) => {
                            self.as_mut().current = next_count;
                            Poll::Ready(Some(next_count))
                        }
                        Poll::Pending => {
                            self.as_mut().temp_future = Some(temp_fut);
                            Poll::Pending
                        }
                    }
                }
            }
        } else {
            Poll::Ready(None) // イテレーションの終了
        }
    }
}

#[tokio::main]
async fn main() {
    use futures_util::StreamExt;

    let mut stream_counter = Counter::new(5, Duration::from_millis(100));

    while let Some(count) = stream_counter.next().await {
        println!("count: {count}");
    }
}
