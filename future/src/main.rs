use std::future::Future;
use std::task::Poll;
use std::time::{Duration, Instant};
struct Delay {
    time: Instant,
}

impl Delay {
    fn new(exp: Duration) -> Self {
        Delay {
            time: Instant::now() + exp,
        }
    }
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if Instant::now() > self.time {
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let dur = self.time - Instant::now();
            tokio::spawn(async move {
                tokio::time::sleep(dur).await;
                waker.wake();
            });
            Poll::Pending
        }
    }
}
#[tokio::main]
async fn main() {
    let before = Instant::now();
    let rs = Delay::new(Duration::from_secs(1)).await;
    println!(
        "Hello, world!, {}, before = {:?}",
        rs,
        Instant::now() - before
    );
}
