//! Counter module for the circular counter logic.
//!
//! This module provides functionality for a counter that runs in a separate thread
//! and can be observed and controlled from the main thread.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::error::GameResult;

/// Represents a circular counter that can be incremented in a separate thread.
/// The counter loops back to 0 after reaching 100, incrementing the miss counter.
pub struct Counter {
    /// The current counter value
    value: Arc<Mutex<u32>>,
    /// Number of times the counter has reset to 0
    miss: Arc<Mutex<u32>>,
    /// Flag indicating if the counter is running
    running: Arc<Mutex<bool>>,
}

impl Counter {
    /// Creates a new Counter instance.
    ///
    /// # Returns
    ///
    /// A new Counter with values initialized to zero
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
            miss: Arc::new(Mutex::new(0)),
            running: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Starts the counter in a separate thread.
    ///
    /// # Arguments
    ///
    /// * `speed_ms` - The increment speed in milliseconds
    pub fn start(&self, speed_ms: u32) -> GameResult<()> {
        let value = Arc::clone(&self.value);
        let miss = Arc::clone(&self.miss);
        let running = Arc::clone(&self.running);
        
        // Reset counters
        *self.value.lock().unwrap() = 0;
        *self.miss.lock().unwrap() = 0;
        *self.running.lock().unwrap() = true;
        
        // Start a thread to update the counter
        thread::spawn(move || {
            while *running.lock().unwrap() {
                thread::sleep(Duration::from_millis(u64::from(speed_ms)));
                let mut val = value.lock().unwrap();
                *val += 1;
                
                // Reset counter and increment miss when exceeding 100
                if *val > 100 {
                    *val = 0;
                    let mut m = miss.lock().unwrap();
                    *m += 1;
                }
            }
        });
        
        Ok(())
    }
    
    /// Stops the counter and returns the current value and miss count.
    ///
    /// # Returns
    ///
    /// A tuple containing the current counter value and miss count.
    pub fn stop(&self) -> (u32, u32) {
        *self.running.lock().unwrap() = false;
        let value = *self.value.lock().unwrap();
        let miss = *self.miss.lock().unwrap();
        (value, miss)
    }
    
    /// Gets shared references to the counter's internal state for display purposes.
    ///
    /// # Returns
    ///
    /// Tuple containing Arc<Mutex> references to value, miss, and running state
    pub fn get_display_values(&self) -> (Arc<Mutex<u32>>, Arc<Mutex<u32>>, Arc<Mutex<bool>>) {
        (
            Arc::clone(&self.value),
            Arc::clone(&self.miss),
            Arc::clone(&self.running)
        )
    }
    
    /// Gets the current counter value.
    ///
    /// # Returns
    ///
    /// The current counter value
    #[cfg(test)]
    pub fn get_value(&self) -> u32 {
        *self.value.lock().unwrap()
    }
    
    /// Gets the current miss count.
    ///
    /// # Returns
    ///
    /// The current miss count
    #[cfg(test)]
    pub fn get_miss(&self) -> u32 {
        *self.miss.lock().unwrap()
    }
}

impl Default for Counter {
    /// Creates a new Counter with default values.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_counter_initial_state() {
        let counter = Counter::new();
        assert_eq!(counter.get_value(), 0);
        assert_eq!(counter.get_miss(), 0);
    }

    #[test]
    fn test_counter_stop() {
        let counter = Counter::new();
        assert!(counter.start(10).is_ok()); // Fast speed for testing
        
        // Let it run briefly
        thread::sleep(Duration::from_millis(50));
        
        let (value, miss) = counter.stop();
        
        // The counter should have incremented at least once
        assert!(value > 0 || miss > 0);
    }
    
    #[test]
    fn test_get_display_values() {
        let counter = Counter::new();
        let (value, miss, running) = counter.get_display_values();
        
        assert_eq!(*value.lock().unwrap(), 0);
        assert_eq!(*miss.lock().unwrap(), 0);
        assert_eq!(*running.lock().unwrap(), false);
    }
    
    #[test]
    fn test_default() {
        let counter = Counter::default();
        assert_eq!(counter.get_value(), 0);
        assert_eq!(counter.get_miss(), 0);
    }
}
