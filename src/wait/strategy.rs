use std::time::Duration;

/// Wait strategy configuration for polling intervals
#[derive(Debug, Clone)]
pub struct WaitStrategy {
    /// Initial polling interval
    pub initial_interval: Duration,
    /// Maximum polling interval
    pub max_interval: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Strategy type
    pub strategy_type: WaitStrategyType,
}

impl Default for WaitStrategy {
    fn default() -> Self {
        Self {
            initial_interval: Duration::from_millis(100),
            max_interval: Duration::from_millis(1000),
            backoff_multiplier: 1.5,
            strategy_type: WaitStrategyType::FixedInterval,
        }
    }
}

impl WaitStrategy {
    /// Creates a fixed interval wait strategy
    pub fn fixed_interval(interval: Duration) -> Self {
        Self {
            initial_interval: interval,
            max_interval: interval,
            backoff_multiplier: 1.0,
            strategy_type: WaitStrategyType::FixedInterval,
        }
    }

    /// Creates an exponential backoff wait strategy
    pub fn exponential_backoff(
        initial_interval: Duration,
        max_interval: Duration,
        multiplier: f64,
    ) -> Self {
        Self {
            initial_interval,
            max_interval,
            backoff_multiplier: multiplier,
            strategy_type: WaitStrategyType::ExponentialBackoff,
        }
    }

    /// Creates a linear backoff wait strategy
    pub fn linear_backoff(initial_interval: Duration, max_interval: Duration) -> Self {
        Self {
            initial_interval,
            max_interval,
            backoff_multiplier: 1.0,
            strategy_type: WaitStrategyType::LinearBackoff,
        }
    }

    /// Creates a fibonacci backoff wait strategy
    pub fn fibonacci_backoff(initial_interval: Duration, max_interval: Duration) -> Self {
        Self {
            initial_interval,
            max_interval,
            backoff_multiplier: 1.0,
            strategy_type: WaitStrategyType::FibonacciBackoff,
        }
    }

    /// Calculates the next polling interval based on the strategy
    pub fn next_interval(
        &self,
        attempt: u32,
        _elapsed: Duration,
        _timeout: Duration,
    ) -> Duration {
        match self.strategy_type {
            WaitStrategyType::FixedInterval => self.initial_interval,
            WaitStrategyType::ExponentialBackoff => {
                let multiplier = self.backoff_multiplier.powi((attempt - 1) as i32);
                let interval_ms = (self.initial_interval.as_millis() as f64 * multiplier) as u64;
                let interval = Duration::from_millis(interval_ms);
                interval.min(self.max_interval)
            }
            WaitStrategyType::LinearBackoff => {
                let interval_ms = self.initial_interval.as_millis() as u64 * attempt as u64;
                let interval = Duration::from_millis(interval_ms);
                interval.min(self.max_interval)
            }
            WaitStrategyType::FibonacciBackoff => {
                let fib_value = fibonacci(attempt);
                let interval_ms = self.initial_interval.as_millis() as u64 * fib_value;
                let interval = Duration::from_millis(interval_ms);
                interval.min(self.max_interval)
            }
        }
    }
}

/// Types of wait strategies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaitStrategyType {
    /// Fixed polling interval
    FixedInterval,
    /// Exponential backoff (interval increases exponentially)
    ExponentialBackoff,
    /// Linear backoff (interval increases linearly)
    LinearBackoff,
    /// Fibonacci backoff (interval follows Fibonacci sequence)
    FibonacciBackoff,
}

/// Calculates the nth Fibonacci number
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_interval_strategy() {
        let strategy = WaitStrategy::fixed_interval(Duration::from_millis(500));
        
        assert_eq!(
            strategy.next_interval(1, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(500)
        );
        
        assert_eq!(
            strategy.next_interval(5, Duration::from_secs(2), Duration::from_secs(10)),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn test_exponential_backoff_strategy() {
        let strategy = WaitStrategy::exponential_backoff(
            Duration::from_millis(100),
            Duration::from_millis(2000),
            2.0,
        );
        
        assert_eq!(
            strategy.next_interval(1, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(100)
        );
        
        assert_eq!(
            strategy.next_interval(2, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(200)
        );
        
        assert_eq!(
            strategy.next_interval(3, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(400)
        );
        
        // Should cap at max_interval
        assert_eq!(
            strategy.next_interval(10, Duration::from_secs(0), Duration::from_secs(60)),
            Duration::from_millis(2000)
        );
    }

    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn test_fibonacci_backoff_strategy() {
        let strategy = WaitStrategy::fibonacci_backoff(
            Duration::from_millis(50),
            Duration::from_millis(1000),
        );
        
        assert_eq!(
            strategy.next_interval(1, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(50)  // 50 * 1
        );
        
        assert_eq!(
            strategy.next_interval(2, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(50)  // 50 * 1
        );
        
        assert_eq!(
            strategy.next_interval(3, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(100)  // 50 * 2
        );
        
        assert_eq!(
            strategy.next_interval(4, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(150)  // 50 * 3
        );
        
        assert_eq!(
            strategy.next_interval(5, Duration::from_secs(0), Duration::from_secs(10)),
            Duration::from_millis(250)  // 50 * 5
        );
    }
}
