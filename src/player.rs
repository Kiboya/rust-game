//! Player module for managing player attributes and state.
//!
//! This module defines the Player struct and its associated methods for
//! managing player characteristics during gameplay.

/// Represents a player in the game with their characteristics.
#[derive(Debug, Clone)]
pub struct Player {
    /// The player's name
    name: String,
    /// The player's health points
    vitality: u32,
    /// The player's movement speed (affects counter speed)
    speed: u32,
    /// The player's power (affects scoring)
    strength: u32,
}

impl Player {
    /// Creates a new player with the given name and attributes.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the player
    /// * `vitality` - The initial vitality of the player
    /// * `speed` - The initial speed of the player
    /// * `strength` - The initial strength of the player
    ///
    /// # Returns
    ///
    /// A new Player instance
    pub fn new(name: String, vitality: u32, speed: u32, strength: u32) -> Self {
        Self {
            name,
            vitality,
            speed,
            strength,
        }
    }
    
    /// Returns the player's name.
    ///
    /// # Returns
    ///
    /// Reference to the player's name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the player's current vitality.
    ///
    /// # Returns
    ///
    /// The player's vitality value
    pub fn vitality(&self) -> u32 {
        self.vitality
    }
    
    /// Returns the player's current speed.
    ///
    /// # Returns
    ///
    /// The player's speed value
    pub fn speed(&self) -> u32 {
        self.speed
    }
    
    /// Returns the player's current strength.
    ///
    /// # Returns
    ///
    /// The player's strength value
    pub fn strength(&self) -> u32 {
        self.strength
    }
    
    /// Decreases the player's vitality by the given amount.
    /// Vitality will not go below zero.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to decrease
    pub fn decrease_vitality(&mut self, amount: u32) {
        self.vitality = self.vitality.saturating_sub(amount);
    }
    
    /// Decreases the player's speed by the given amount.
    /// Speed will not go below zero.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to decrease
    pub fn decrease_speed(&mut self, amount: u32) {
        self.speed = self.speed.saturating_sub(amount);
    }
    
    /// Decreases the player's strength by the given amount.
    /// Strength will not go below zero.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to decrease
    pub fn decrease_strength(&mut self, amount: u32) {
        self.strength = self.strength.saturating_sub(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let player = Player::new("TestPlayer".to_string(), 100, 60, 70);
        assert_eq!(player.name(), "TestPlayer");
        assert_eq!(player.vitality(), 100);
        assert_eq!(player.speed(), 60);
        assert_eq!(player.strength(), 70);
    }

    #[test]
    fn test_decrease_vitality() {
        let mut player = Player::new("TestPlayer".to_string(), 100, 50, 50);
        player.decrease_vitality(30);
        assert_eq!(player.vitality(), 70);
        
        // Test that vitality doesn't go below 0
        player.decrease_vitality(100);
        assert_eq!(player.vitality(), 0);
    }

    #[test]
    fn test_decrease_speed() {
        let mut player = Player::new("TestPlayer".to_string(), 100, 50, 50);
        player.decrease_speed(20);
        assert_eq!(player.speed(), 30);
        
        // Test that speed doesn't go below 0
        player.decrease_speed(50);
        assert_eq!(player.speed(), 0);
    }

    #[test]
    fn test_decrease_strength() {
        let mut player = Player::new("TestPlayer".to_string(), 100, 50, 50);
        player.decrease_strength(10);
        assert_eq!(player.strength(), 40);
        
        // Test that strength doesn't go below 0
        player.decrease_strength(50);
        assert_eq!(player.strength(), 0);
    }
    
    #[test]
    fn test_player_clone() {
        let player1 = Player::new("TestPlayer".to_string(), 100, 60, 70);
        let player2 = player1.clone();
        
        assert_eq!(player1.name(), player2.name());
        assert_eq!(player1.vitality(), player2.vitality());
        assert_eq!(player1.speed(), player2.speed());
        assert_eq!(player1.strength(), player2.strength());
    }
}
