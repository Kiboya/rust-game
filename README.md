# Rust Game

A terminal-based turn-based game where players compete to hit target numbers using only the ENTER key.

## Overview

In this game, two players take turns trying to stop a counter as close as possible to randomly generated target numbers. Each player has unique characteristics (vitality, speed, and strength) that affect gameplay. Players earn points based on their precision, and the game ends when one player's vitality reaches zero.

The goal of this project was to go over the fundamentals of Rust, including ownership, borrowing, and threading. 

## Features

- Terminal-based user interface
- Turn-based gameplay using only the ENTER key
- Dynamic player characteristics that change during gameplay
- Threaded counter display with adjustable speeds
- Scoring system based on precision and player stats
- Round-based gameplay with penalties for the losing player

## Installation

### Prerequisites

- Rust
- Cargo (Rust's package manager)

### Steps

1. Clone the repository:
   ```
   git clone https://github.com/Kiboya/rust-game.git
   cd rust_game
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the game:

    The game can be run with or without arguments. If no arguments are provided, the game will use default values for player names and characteristics.

    No arguments:
   ```
   cargo run
   ```

    With arguments:
    ```
    cargo run -- --name1 <player1_name> --name2 <player2_name> --vitality <vitality> --objectives <objectives> --speed <speed> --strength <strength>
    ```
    Example:
    ```
    cargo run -- --name1 Alice --name2 Bob --vitality 100 --objectives 5 --speed 50 --strength 10
    ```
    This will start the game with Alice and Bob as players, each with 100 vitality, 5 objectives, a speed of 50 milliseconds, and a strength of 10.

## How to Play

1. Start the game and enter names for two players
2. Each round consists of players taking turns
3. On your turn:
   - A table of random target numbers will be displayed
   - Press ENTER to start the counter
   - Press ENTER again to stop the counter when you think it's close to a target
   - Repeat for each target in the table

## Game Mechanics

### Player Characteristics

- **Name**: Player's identifier
- **Vitality**: Health points; game ends when this reaches zero
- **Speed**: Determines counter increment speed in milliseconds (lower is faster)
- **Strength**: Affects scoring (higher gives better scores)

### Counter Mechanics

- The counter increments from 0 to 100, looping back to 0 after reaching 100
- Each complete loop increments a "miss" counter that reduces scoring
- The counter update rate is determined by the player's speed characteristic

### Scoring System

Points are calculated based on the difference between counter value and target:

| Difference (absolute) | Score Formula                |
|-----------------------|-----------------------------|
| 0                     | (100 + strength) / (miss + 1) |
| 1–5                   | (80 + strength) / (miss + 1)  |
| 6–10                  | (60 + strength) / (miss + 1)  |
| 11–20                 | (40 + strength) / (miss + 1)  |
| 21–50                 | (20 + strength) / (miss + 1)  |
| >50                   | (0 + strength) / (miss + 1)   |

The final score for a turn is the average across all targets, rounded up.

### Round Resolution

- The player with the higher score wins the round
- The loser loses vitality equal to the score difference
- The winner chooses a penalty ("poison") for their opponent, reducing either their speed or strength by 5 points

### Game End

The game ends when one player's vitality reaches zero.

## Project Structure

- **main.rs**: Entry point and main game loop
- **player.rs**: Player data structures and methods
- **counter.rs**: Counter mechanics and threading
- **scoring.rs**: Score calculation logic
- **game.rs**: Game state and round management
- **ui.rs**: Terminal UI rendering
- **error.rs**: Error handling

## Dependencies

- **chrono**: Time handling
- **clap**: Command-line argument parsing
- **env_logger/log**: Logging functionality
- **rand**: Random number generation
