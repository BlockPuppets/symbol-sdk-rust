use std::time::Duration;

use super::{
    consts::{MAX_RETRIES, WAIT_DELAY},
    Error,
};

pub trait RetryStrategy: Clone + Copy + std::fmt::Debug + Send + Sync {
    fn max_retries(&self, err: &Error) -> u32;
    fn delay(&self, err: &Error, retries: u32) -> Duration;
}

#[derive(Clone, Copy, Debug)]
pub struct Retry {
    pub max_retries: u32,
    pub delay: Duration,
}

impl Retry {
    pub fn default() -> Self {
        Self {
            max_retries: MAX_RETRIES,
            delay: WAIT_DELAY,
        }
    }
}

impl RetryStrategy for Retry {
    fn max_retries(&self, _: &Error) -> u32 {
        self.max_retries
    }

    fn delay(&self, _: &Error, retries: u32) -> Duration {
        self.delay * retries
    }
}
