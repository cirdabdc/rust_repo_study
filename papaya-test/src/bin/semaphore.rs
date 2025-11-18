use tokio::sync::Semaphore;



#[tokio::main]
async fn main() {
    let semaphore = Semaphore::new(3);
    let a_permit = semaphore.acquire().await.unwrap();
    let two_permit = semaphore.acquire_many(2).await.unwrap();

    assert_eq!(semaphore.available_permits(), 0);

    let b_permit = semaphore.try_acquire();
    assert_eq!(b_permit.err(), Some(tokio::sync::TryAcquireError::NoPermits));
}
