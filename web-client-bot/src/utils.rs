use std::error::Error;
use std::future::Future;
use std::time::Duration;
use colored::Colorize;
use tokio::time::sleep;
use tokio::time::timeout;

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

/// Retries the given async task only if it times out.
/// Any other error is returned immediately.
pub async fn retry_on_timeout<'a, T, Fut, F>(
    debug_id: &str,
    mut task: F,
    retries: usize,
    delay: Duration,
    timeout_duration: Duration,
) -> Result<T, Box<dyn Error + Send + Sync>>
where
    T: 'a,
    Fut: Future<Output = Result<T, Box<dyn Error + Send + Sync>>> + Send + 'a,
    F: FnMut() -> Fut,
{
    let mut attempt = 0;

    loop {
        match timeout(timeout_duration, task()).await {
            Ok(result) => return result, // Either Ok(_) or Err(_) — break on both
            Err(_) => {
                attempt += 1;
                if attempt > retries {
                    return Err(Box::new(TimeoutError(String::from("timed out and retries exhausted"))));
                }
                eprintln!("{}", format!(
                    "\t ⚠️ [{debug_id}] timeout on attempt {attempt}, retrying in {:?}",
                    delay,
                ).bright_red());
                sleep(delay).await;
            }
        }
    }
}

/// A general-purpose timeout wrapper. 
/// Returns Ok(result) if successful in time, or a TimeoutError otherwise.
pub async fn with_timeout<T, Fut>(
    fut: Fut,
    duration: Duration,
) -> Result<T, TimeoutError>
where
    Fut: Future<Output = T> + Send,
    T: Send + 'static,
{
    match timeout(duration, fut).await {
        Ok(value) => Ok(value),
        Err(_) => Err(TimeoutError(format!(
            "operation exceeded timeout of {:?}",
            duration
        ))),
    }
}

/// A general-purpose timeout wrapper. 
/// Returns Ok(result) if successful in time, or a TimeoutError otherwise.
pub async fn with_timeout_lazy<T, Fut, F>(
    task: F,
    duration: Duration,
) -> Result<T, TimeoutError>
where
    Fut: Future<Output = T> + Send,
    F: FnOnce() -> Fut,
{
    match timeout(duration, task()).await {
        Ok(value) => Ok(value),
        Err(_) => Err(TimeoutError(format!(
            "operation exceeded timeout of {:?}",
            duration
        ))),
    }
}



#[derive(Debug, Clone)]
pub struct TimeoutError(pub String);
impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[timeout error] {}", self.0)
    }
}
impl std::error::Error for TimeoutError {}

