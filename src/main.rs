//! # Turn-Based Terminal Game
//! 
//! A terminal-based turn-based game where two players compete using only the ENTER key for input.
//! Players aim to stop a counter as close as possible to randomly generated target numbers.
//! 
//! ## How to Play
//! 
//! 1. At the start of each turn, a target table is generated with random numbers.
//! 2. The player presses ENTER to start their turn and a counter begins incrementing.
//! 3. When the player presses ENTER again, the counter freezes and a score is calculated.
//! 4. The player with the highest average score wins the round.
//! 5. The game continues until one player's vitality reaches zero.

mod player;
mod counter;
mod scoring;
mod game;
mod ui;
mod error;

use clap::{Command, Arg};
use game::Game;
use crate::error::GameResult;

/// The entry point for the game application.
///
/// Parses command line arguments and starts the game.
/// Returns an exit code appropriate to the result.
fn main() -> GameResult<()> {
    // Initialize logger with colors enabled
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .write_style(env_logger::WriteStyle::Always) // Force color output
        .init();

    // Set up command line argument parsing
    let matches = Command::new("Turn-based Game")
        .version("1.0")
        .author("SEC3 Game Developer")
        .about("A terminal-based turn-based game")
        .arg(Arg::new("name1")
            .long("name1")
            .value_name("NAME")
            .help("Name of player 1")
            .default_value("Player 1"))
        .arg(Arg::new("name2")
            .long("name2")
            .value_name("NAME")
            .help("Name of player 2")
            .default_value("Player 2"))
        .arg(Arg::new("vitality")
            .long("vitality")
            .value_name("AMOUNT")
            .help("Starting vitality for both players")
            .default_value("50"))
        .arg(Arg::new("speed")
            .long("speed")
            .value_name("AMOUNT")
            .help("Starting speed for both players")
            .default_value("50"))
        .arg(Arg::new("strength")
            .long("strength")
            .value_name("AMOUNT")
            .help("Starting strength for both players")
            .default_value("50"))
        .arg(Arg::new("objectives")
            .long("objectives")
            .value_name("COUNT")
            .help("Number of targets per turn")
            .default_value("5"))
        .get_matches();

    // Parse command line arguments
    let player1_name = matches.get_one::<String>("name1").unwrap().to_string();
    let player2_name = matches.get_one::<String>("name2").unwrap().to_string();
    
    let vitality = matches.get_one::<String>("vitality")
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            log::error!("Invalid vitality value, using default of 50");
            50
        });
    
    let speed = matches.get_one::<String>("speed")
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            log::error!("Invalid speed value, using default of 50");
            50
        });
    
    let strength = matches.get_one::<String>("strength")
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            log::error!("Invalid strength value, using default of 50");
            50
        });
    
    let target_count = matches.get_one::<String>("objectives")
        .unwrap()
        .parse::<usize>()
        .unwrap_or_else(|_| {
            log::error!("Invalid target count, using default of 5");
            5
        });
    
    // Create and run the game
    loop {
        let mut game = Game::new(
            player1_name.clone(), 
            player2_name.clone(), 
            vitality,
            speed,
            strength,
            target_count
        );
        
        if !game.run() {
            break;
        }
    }
    
    Ok(())
}