//! Counter module for the circular counter logic.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Represents a circular counter that can be incremented in a separate thread.
/// The counter loops back to 0 after reaching 100, incrementing the miss counter.
pub struct Counter {
    value: Arc<Mutex<u32>>,
    miss: Arc<Mutex<u32>>,
    running: Arc<Mutex<bool>>,
}

impl Counter {
    /// Creates a new Counter instance.
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
    pub fn start(&self, speed_ms: u32) {
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
                thread::sleep(Duration::from_millis(speed_ms as u64));
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
    pub fn get_display_values(&self) -> (Arc<Mutex<u32>>, Arc<Mutex<u32>>, Arc<Mutex<bool>>) {
        (
            Arc::clone(&self.value),
            Arc::clone(&self.miss),
            Arc::clone(&self.running)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_counter_stop() {
        let counter = Counter::new();
        counter.start(10); // Fast speed for testing
        
        // Let it run briefly
        thread::sleep(Duration::from_millis(50));
        
        let (value, miss) = counter.stop();
        
        // The counter should have incremented at least once
        assert!(value > 0 || miss > 0);
    }
}
