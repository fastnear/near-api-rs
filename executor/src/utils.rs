pub async fn retry<R, E, T, F>(
    mut task: F,
    retries: u8,
    initial_sleep: std::time::Duration,
    exponential_backoff: bool,
) -> T::Output
where
    F: FnMut() -> T + Send,
    T: core::future::Future<Output = core::result::Result<R, E>> + Send,
    T::Output: Send,
{
    let mut retries = (1..=retries).rev();
    let mut sleep_duration = initial_sleep;
    loop {
        let result = task().await;
        match result {
            Ok(result) => return Ok(result),
            Err(_) if retries.next().is_some() => {
                tokio::time::sleep(sleep_duration).await;
                sleep_duration = if exponential_backoff {
                    sleep_duration * 2
                } else {
                    sleep_duration
                };
            }
            Err(err) => return Err(err),
        }
    }
}
