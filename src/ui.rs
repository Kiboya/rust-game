//! UI module for terminal display and user interaction.
//!
//! This module provides functions for displaying information and gathering input
//! from users in a terminal environment.

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::error::{GameError, GameResult};

/// Displays a counter that increments in real-time.
///
/// # Arguments
///
/// * `value` - Shared counter value
/// * `miss` - Shared miss counter
/// * `running` - Shared flag indicating if counter is running
/// * `target` - Target number to display
///
/// # Returns
///
/// A handle to the display thread
pub fn display_counter(
    value: Arc<Mutex<u32>>,
    miss: Arc<Mutex<u32>>,
    running: Arc<Mutex<bool>>,
    target: u32
) -> GameResult<thread::JoinHandle<()>> {
    // Show the prompt on its own line
    print!("Press ENTER to stop the counter...");
    io::stdout().flush().map_err(GameError::from)?;

    let handle = thread::spawn(move || {
        while *running.lock().unwrap() {
            let v = *value.lock().unwrap();
            let m = *miss.lock().unwrap();
            print!("\r\x1B[K→ Objective {}: Miss = {} | Counter = {}", target, m, v);
            // Ignoring potential errors here as we can't propagate from thread
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(30));
        }
    });
    
    Ok(handle)
}

/// Waits for the user to press ENTER.
///
/// # Returns
///
/// Result indicating whether reading input succeeded.
pub fn wait_for_enter() -> GameResult<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(GameError::from)?;
    Ok(())
}

/// Prompts the user for a choice between given options.
///
/// # Arguments
///
/// * `prompt` - The message to display
/// * `options` - The available options
/// * `test_input` - Optional test input for automated testing
///
/// # Returns
///
/// Result containing the selected option index (0-based)
pub fn get_user_choice(prompt: &str, options: &[&str], test_input: Option<usize>) -> GameResult<usize> {
    if let Some(choice) = test_input {
        if choice < options.len() {
            return Ok(choice);
        } else {
            return Ok(0); // Default to first option if invalid test input
        }
    }
    
    println!("{}", prompt);
    
    for (i, option) in options.iter().enumerate() {
        println!("→ {}: {}", i + 1, option);
    }
    
    print!(">");
    io::stdout().flush().map_err(GameError::from)?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(GameError::from)?;
    
    match input.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= options.len() => Ok(n - 1),
        Ok(_) => {
            println!("Invalid choice. Selecting the first option by default.");
            Ok(0)
        },
        Err(_) => {
            println!("Could not parse input. Selecting the first option by default.");
            Ok(0)
        }
    }
}

/// Prints a formatted heading to the terminal.
///
/// # Arguments
///
/// * `text` - The heading text
/// * `level` - The heading level (1-3)
///
/// # Returns
///
/// Result indicating whether the print operation succeeded
pub fn print_heading(text: &str, level: u8) -> GameResult<()> {
    match level {
        1 => println!("##### {} #####", text),
        2 => println!("## {} ##", text),
        3 => println!("# {} #", text),
        _ => println!("{}", text),
    }
    
    io::stdout().flush().map_err(GameError::from)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    
    #[test]
    fn test_print_heading() {
        // This is mostly for coverage, as it just prints to stdout
        assert!(print_heading("Test Heading 1", 1).is_ok());
        assert!(print_heading("Test Heading 2", 2).is_ok());
        assert!(print_heading("Test Heading 3", 3).is_ok());
        assert!(print_heading("Test Heading 4", 4).is_ok());
    }
    
    #[test]
    fn test_display_counter() {
        let value = Arc::new(Mutex::new(42));
        let miss = Arc::new(Mutex::new(0));
        let running = Arc::new(Mutex::new(true));
        
        let handle_result = display_counter(
            Arc::clone(&value),
            Arc::clone(&miss),
            Arc::clone(&running),
            50
        );
        
        assert!(handle_result.is_ok());
        
        // Stop the counter thread
        *running.lock().unwrap() = false;
        if let Ok(handle) = handle_result {
            assert!(handle.join().is_ok());
        }
    }
}
