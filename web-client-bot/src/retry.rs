use std::error::Error;
use std::future::Future;
// use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

pub async fn retry_async<'a, T, Fut, F>(
    mut task: F,
    retries: usize,
    delay: Duration,
) -> Result<T, Box<dyn Error + Send + Sync>>
where
    T: 'a,
    Fut: Future<Output = Result<T, Box<dyn Error + Send + Sync>>> + Send + 'a,
    F: FnMut() -> Fut,
{
    let mut attempt = 0;
    loop {
        match task().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                attempt += 1;
                if attempt > retries {
                    return Err(err);
                }
                sleep(delay).await;
            }
        }
    }
}

