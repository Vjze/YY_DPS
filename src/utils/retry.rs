use std::time::Duration;
use tokio::time::sleep;

/// 重试机制配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

/// 带重试机制的异步操作
pub async fn retry_async<F, T, E, Fut>(
    config: RetryConfig,
    operation: F,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = config.base_delay;
    
    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == config.max_attempts {
                    tracing::error!("操作失败，已重试{}次: {}", attempt, e);
                    return Err(e);
                }
                
                tracing::warn!("操作失败，第{}次重试在{}ms后: {}", attempt, delay.as_millis(), e);
                sleep(delay).await;
                
                delay = std::cmp::min(
                    Duration::from_millis((delay.as_millis() as f64 * config.backoff_multiplier) as u64),
                    config.max_delay,
                );
            }
        }
    }
    
    unreachable!()
}

/// 简化的重试函数，使用默认配置
pub async fn retry_default<F, T, E, Fut>(operation: F) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    retry_async(RetryConfig::default(), operation).await
}