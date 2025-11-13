use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Instant, Duration};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        if Instant::now() >= self.when {
            // 已经到了延迟的时间，返回Ready
            println!("now: {:?}", Instant::now());
            Poll::Ready("done")
        } else {
            // 还没有到延迟的时间，注册waker，等待到了时间再唤醒
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}


#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(5);
    let mut delay_future = Delay { when };

    let res = delay_future.await;
    println!("{:?}", res);
}