//! UI module for terminal display and user interaction.

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn display_counter(
    value: Arc<Mutex<u32>>,
    miss: Arc<Mutex<u32>>,
    running: Arc<Mutex<bool>>,
    target: u32
) -> thread::JoinHandle<()> {
    // Show the prompt on its own line
    print!("Press ENTER to stop the counter...");
    io::stdout().flush().unwrap();

    thread::spawn(move || {
        while *running.lock().unwrap() {
            let v = *value.lock().unwrap();
            let m = *miss.lock().unwrap();
            print!("\r\x1B[K→ Objective {}: Miss = {} | Counter = {}", target, m, v);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(30));
        }
    })
}


/// Waits for the user to press ENTER.
///
/// # Returns
///
/// Result indicating whether reading input succeeded.
pub fn wait_for_enter() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}

/// Prompts the user for a choice between given options.
///
/// # Arguments
///
/// * `prompt` - The message to display
/// * `options` - The available options
///
/// # Returns
///
/// The selected option index (0-based)
pub fn get_user_choice(prompt: &str, options: &[&str]) -> usize {
    println!("{}", prompt);
    
    for (i, option) in options.iter().enumerate() {
        println!("→ {}: {}", i + 1, option);
    }
    
    print!(">");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    
    match input.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= options.len() => n - 1,
        _ => {
            println!("Invalid choice. Selecting the first option by default.");
            0
        }
    }
}

/// Prints a formatted heading to the terminal.
///
/// # Arguments
///
/// * `text` - The heading text
/// * `level` - The heading level (1-3)
pub fn print_heading(text: &str, level: u8) {
    match level {
        1 => println!("##### {} #####", text),
        2 => println!("## {} ##", text),
        3 => println!("# {} #", text),
        _ => println!("{}", text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Most UI functions involve IO, so we'll just test the non-IO functions
    
    #[test]
    fn test_print_heading() {
        // This is mostly for coverage, as it just prints to stdout
        print_heading("Test Heading 1", 1);
        print_heading("Test Heading 2", 2);
        print_heading("Test Heading 3", 3);
        print_heading("Test Heading 4", 4);
        // No assertions needed as this just prints to stdout
    }
}
