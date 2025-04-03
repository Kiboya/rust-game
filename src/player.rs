//! Player module for managing player attributes and state.

/// Represents a player in the game with their characteristics.
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    vitality: u32,
    speed: u32,
    strength: u32,
}

impl Player {
    /// Creates a new player with the given name and vitality.
    /// Speed and strength are initialized to default values of 50.
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
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the player's current vitality.
    pub fn vitality(&self) -> u32 {
        self.vitality
    }
    
    /// Returns the player's current speed.
    pub fn speed(&self) -> u32 {
        self.speed
    }
    
    /// Returns the player's current strength.
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
}
