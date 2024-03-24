use counter_rs_sys as ffi;
use std::error::Error;
use std::ffi::{c_int, c_void};
use std::fmt::{Display, Formatter};
use std::slice;

/// A safe wrapper around cpp-example/Counter
#[derive(Debug)]
pub struct Counter {
    ptr: *mut c_void,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CounterError {
    Overflow,
    Underflow,
    UnknownError(c_int),
}

impl Counter {
    pub fn new(initial_value: u32) -> Self {
        let ptr = unsafe { ffi::CreateInstance(initial_value) };
        Self { ptr }
    }

    pub fn increment(&mut self, amount: u32) -> Result<(), CounterError> {
        let result = unsafe { ffi::Increment(self.ptr, amount) };

        match result {
            0 => Ok(()),
            -1 => Err(CounterError::Overflow),
            _ => Err(CounterError::UnknownError(result)),
        }
    }

    pub fn decrement(&mut self, amount: u32) -> Result<(), CounterError> {
        let result = unsafe { ffi::Decrement(self.ptr, amount) };

        match result {
            0 => Ok(()),
            -1 => Err(CounterError::Underflow),
            _ => Err(CounterError::UnknownError(result)),
        }
    }

    pub fn current_value(&mut self) -> u32 {
        let current = unsafe { ffi::GetCurrentValue(self.ptr) };
        current
    }

    pub fn history(&mut self) -> Vec<u32> {
        unsafe {
            let mut length = 0;
            let ptr = ffi::GetHistory(self.ptr, &mut length);
            let history = slice::from_raw_parts(ptr, length).to_vec();

            libc::free(ptr as *mut c_void);

            history
        }
    }
}

/// SAFETY
///
/// As long as only one thread accesses the counter then it's safe
/// to move between threads. This means it is Send but not Sync.
unsafe impl Send for Counter {}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe {
            ffi::ReleaseInstance(self.ptr);
        }
    }
}

impl Display for CounterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CounterError::Overflow => write!(f, "Overflow error"),
            CounterError::Underflow => write!(f, "Underflow error"),
            CounterError::UnknownError(code) => write!(f, "Unknown error with code {code}"),
        }
    }
}

impl Error for CounterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let mut counter = Counter::new(42);
        assert_eq!(counter.current_value(), 42);

        counter.increment(1).unwrap();
        assert_eq!(counter.current_value(), 43);

        counter.decrement(42).unwrap();
        assert_eq!(counter.current_value(), 1);

        assert_eq!(counter.increment(u32::MAX), Err(CounterError::Overflow));
        assert_eq!(counter.decrement(2), Err(CounterError::Underflow));

        assert_eq!(counter.history(), &[42, 43, 1]);

        drop(counter);
    }
}
