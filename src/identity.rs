use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::Future;
use tokio::time::{sleep, Sleep};

/// async fnを用いたdelayする非同期恒等写像(意味は無い)
pub async fn async_identity_v1<T>(duration: Duration, input: T) -> T {
    sleep(duration).await;
    input
}

/// Futureを実装する用の非同期恒等写像(意味は無い)
pub struct Identity<T> {
    async_sleep: Pin<Box<Sleep>>,
    input: Option<T>,
}

impl<T> Identity<T> {
    pub fn new(duration: Duration, input: T) -> Self {
        Self {
            async_sleep: Box::pin(sleep(duration)),
            input: Some(input),
        }
    }
}

impl<T: Unpin> Future for Identity<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.async_sleep.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(self.as_mut().input.take().unwrap()), // 一度Readyののち再び呼ばれた場合はパニックする
        }
    }
}

/// Identityを返す関数
pub fn async_identity_v2<T: Unpin>(duration: Duration, input: T) -> impl Future<Output = T> {
    Identity::new(duration, input)
}

#[cfg(test)]
mod test {
    use super::{async_identity_v1, async_identity_v2};
    use std::time::Duration;

    #[tokio::test]
    async fn test_identity() {
        assert_eq!(0, async_identity_v1(Duration::from_millis(100), 0).await);

        assert_eq!(10, async_identity_v2(Duration::from_millis(100), 10).await);
    }
}
