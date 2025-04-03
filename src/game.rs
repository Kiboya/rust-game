//! Game module for managing the game state and logic.

use crate::player::Player;
use crate::counter::Counter;
use crate::scoring;
use crate::ui;
use rand::Rng;
use std::io::{self, Write};

/// Represents the game state.
pub struct Game {
    players: [Player; 2],
    target_count: usize,
    game_over: bool,
    winner_idx: Option<usize>,
}

impl Game {
    /// Creates a new game with the specified players and settings.
    ///
    /// # Arguments
    ///
    /// * `player1_name` - Name of the first player
    /// * `player2_name` - Name of the second player
    /// * `vitality` - Starting vitality for both players
    /// * `speed` - Starting speed for both players
    /// * `strength` - Starting strength for both players
    /// * `target_count` - Number of targets per turn
    ///
    /// # Returns
    ///
    /// A new Game instance
    pub fn new(player1_name: String, player2_name: String, vitality: u32, speed: u32, strength: u32, target_count: usize) -> Self {
        let players = [
            Player::new(player1_name, vitality, speed, strength),
            Player::new(player2_name, vitality, speed, strength),
        ];
        
        Self {
            players,
            target_count,
            game_over: false,
            winner_idx: None,
        }
    }
    
    /// Runs the game until one player's vitality reaches zero.
    ///
    /// # Returns
    ///
    /// true if the player wants to play again, false otherwise
    pub fn run(&mut self) -> bool {
        ui::print_heading("Game Start", 1);
        let mut round = 1;
        
        // While both players have vitality, continue the game
        while self.players[0].vitality() > 0 && self.players[1].vitality() > 0 && !self.game_over {
            ui::print_heading(format!("Round {}", round).as_str(), 2);
            
            // Player 1's turn
            let p1_score = self.play_turn(0);
            
            // Player 2's turn
            let p2_score = self.play_turn(1);
            
            // Determine the winner of the round
            self.process_round_result(p1_score, p2_score);
            
            ui::print_heading(format!("END of Round {}", round).as_str(), 2);
            round += 1;
        }
        
        // One player has lost all vitality or speed reached 0, game over
        ui::print_heading("Game Over", 1);
        
        // Determine winner based on either winner_idx (speed = 0 case) or vitality
        let winner = if let Some(idx) = self.winner_idx {
            self.players[idx].name()
        } else if self.players[0].vitality() > 0 {
            self.players[0].name()
        } else {
            self.players[1].name()
        };
        
        println!("Winner: {}", winner);
        
        // Ask if player wants to play again
        print!("Start a new game? [Y/N]\n>");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();
        
        input.trim().eq_ignore_ascii_case("y")
    }
    
    /// Executes a turn for the specified player.
    ///
    /// # Arguments
    ///
    /// * `player_idx` - The index of the player (0 or 1)
    ///
    /// # Returns
    ///
    /// The player's average score for the turn
    fn play_turn(&self, player_idx: usize) -> u32 {
        let player = &self.players[player_idx];
        println!("{}'s turn (Vitality={}, Speed={}, Strength={})", 
                 player.name(), player.vitality(), player.speed(), player.strength());
        
        // Generate random targets
        let targets = self.generate_targets();
        println!("→ Objectives: {:?}", targets);
        println!("→ Press ENTER to start the turn..");
        
        let _ = ui::wait_for_enter();
        let mut scores = Vec::new();
        
        for (_i, &target) in targets.iter().enumerate() {
            let counter = Counter::new();
            let (value_arc, miss_arc, running_arc) = counter.get_display_values();
            // Capture the join handle from display_counter:
            let ui_handle = ui::display_counter(value_arc, miss_arc, running_arc.clone(), target);
            counter.start(player.speed());
            let _ = ui::wait_for_enter();
            let (value, miss) = counter.stop();
            // Wait for the UI thread to finish
            ui_handle.join().unwrap();
            
            // Clear the current line before printing final result
            print!("\x1B[A\r\x1B[K"); // Move cursor up and clear line
            io::stdout().flush().unwrap();
    
            // Small pause
            std::thread::sleep(std::time::Duration::from_millis(50));
            
            let score = scoring::calculate_score(target, value, player.strength(), miss);
            scores.push(score);
            let base_score = score * (miss + 1) - player.strength();
            
            // Print the complete, final line
            println!("→ Objective {}: Miss = {} | Counter = {} // Score = ({} + {}) / {} = {}",
                target, miss, value, base_score, player.strength(), miss + 1, score);
        }
        
        let avg_score = scoring::calculate_average_score(&scores);
        
        println!("# End of turn #");
        println!("→ Average score: {}", avg_score);
        
        avg_score
    }
    
    
    /// Generates random targets for a turn.
    ///
    /// # Returns
    ///
    /// A vector of random target numbers
    fn generate_targets(&self) -> Vec<u32> {
        let mut rng = rand::rng();
        (0..self.target_count).map(|_| rng.random_range(0..=100)).collect()
    }
    
    /// Processes the result of a round and applies penalties.
    ///
    /// # Arguments
    ///
    /// * `p1_score` - The score of player 1
    /// * `p2_score` - The score of player 2
    fn process_round_result(&mut self, p1_score: u32, p2_score: u32) {
        if p1_score > p2_score {
            // Player 1 wins
            let diff = p1_score.saturating_sub(p2_score);
            self.players[1].decrease_vitality(diff);
            println!("{} wins the round. {} loses {} vitality points.", 
                     self.players[0].name(), self.players[1].name(), diff);
            
            if self.players[1].vitality() > 0 {
                self.apply_penalty(0, 1);
            }
        } else if p2_score > p1_score {
            // Player 2 wins
            let diff = p2_score.saturating_sub(p1_score);
            self.players[0].decrease_vitality(diff);
            println!("{} wins the round. {} loses {} vitality points.", 
                     self.players[1].name(), self.players[0].name(), diff);
            
            if self.players[0].vitality() > 0 {
                self.apply_penalty(1, 0);
            }
        } else {
            // Draw
            println!("The round is a draw. No vitality lost.");
        }
    }
    
    /// Applies a penalty chosen by the winner to the loser.
    ///
    /// # Arguments
    ///
    /// * `winner_idx` - The index of the winning player
    /// * `loser_idx` - The index of the losing player
    fn apply_penalty(&mut self, winner_idx: usize, loser_idx: usize) {
        println!("{}, you must choose which poison to apply to {}:", 
                 self.players[winner_idx].name(), self.players[loser_idx].name());
        
        let options = ["-5 speed", "-5 strength"];
        let choice = ui::get_user_choice("Choose a penalty:", &options);
        
        match choice {
            0 => {
                self.players[loser_idx].decrease_speed(5);
                println!("{}'s speed reduced by 5!", self.players[loser_idx].name());
                
                // Check if speed reached 0
                if self.players[loser_idx].speed() == 0 {
                    println!("Game Over! {} has lost because their speed reached 0!", 
                             self.players[loser_idx].name());
                    self.game_over = true;
                    self.winner_idx = Some(winner_idx);
                }
            },
            1 => {
                self.players[loser_idx].decrease_strength(5);
                println!("{}'s strength reduced by 5!", self.players[loser_idx].name());
            },
            _ => unreachable!(), // get_user_choice ensures a valid index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(
            "Player1".to_string(), 
            "Player2".to_string(),
            100,
            60,
            70,
            5
        );
        
        assert_eq!(game.players[0].name(), "Player1");
        assert_eq!(game.players[1].name(), "Player2");
        assert_eq!(game.players[0].vitality(), 100);
        assert_eq!(game.players[1].vitality(), 100);
        assert_eq!(game.players[0].speed(), 60);
        assert_eq!(game.players[1].speed(), 70);
    }

    #[test]
    fn test_generate_targets() {
        let game = Game::new(
            "Player1".to_string(), 
            "Player2".to_string(),
            100,
            60,  // speed
            70,  // strength
            5    // target_count
        );
        
        let targets = game.generate_targets();
        
        // Check that the correct number of targets is generated
        assert_eq!(targets.len(), 5);
        
        // Check that all targets are within range
        for target in targets {
            assert!(target <= 100);
        }
    }
}
