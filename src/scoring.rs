//! Scoring module for calculating scores based on counter proximity to targets.

/// Calculates the score for a single target based on the counter value.
///
/// # Arguments
///
/// * `target` - The target number to match
/// * `counter_value` - The counter value when stopped
/// * `strength` - The player's strength attribute
/// * `miss` - The number of times the counter looped back to zero
///
/// # Returns
///
/// The calculated score according to the scoring formula
pub fn calculate_score(target: u32, counter_value: u32, strength: u32, miss: u32) -> u32 {
    // Calculate the distance considering the circular nature (0-100)
    // We need to find the shortest distance between target and counter_value in a 0-100 circle
    let direct_distance = if target > counter_value {
        target - counter_value
    } else {
        counter_value - target
    };
    
    // The circular distance will be the minimum of:
    // 1. Direct distance
    // 2. Going the other way around the circle (101 - direct_distance)
    let difference = std::cmp::min(direct_distance, 101 - direct_distance);
    
    // Base score depends on the difference between target and counter
    let base_score = match difference {
        0 => 100,
        1..=5 => 80,
        6..=10 => 60,
        11..=20 => 40,
        21..=50 => 20,
        _ => 0,
    };
    
    // Apply the scoring formula: (base_score + strength) / (miss + 1)
    (base_score + strength) / (miss + 1)
}

/// Calculates the average score from a collection of individual scores.
///
/// # Arguments
///
/// * `scores` - A slice of individual scores
///
/// # Returns
///
/// The average score rounded up to the nearest integer
pub fn calculate_average_score(scores: &[u32]) -> u32 {
    if scores.is_empty() {
        return 0;
    }
    
    let sum: u32 = scores.iter().sum();
    let avg = (sum as f64) / (scores.len() as f64);
    avg.ceil() as u32 // Round up to nearest integer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_calculation() {
        // Test exact match (difference = 0)
        assert_eq!(calculate_score(50, 50, 50, 0), 150);
        
        // Test difference 1-5
        assert_eq!(calculate_score(50, 52, 50, 0), 130);
        assert_eq!(calculate_score(50, 47, 50, 0), 130);
        
        // Test difference 6-10
        assert_eq!(calculate_score(50, 58, 50, 0), 110);
        assert_eq!(calculate_score(50, 42, 50, 0), 110);
        
        // Test difference 11-20
        assert_eq!(calculate_score(50, 65, 50, 0), 90);
        assert_eq!(calculate_score(50, 35, 50, 0), 90);
        
        // Test difference 21-50
        assert_eq!(calculate_score(50, 80, 50, 0), 70);
        assert_eq!(calculate_score(50, 25, 50, 0), 70);
        
        // Test circular difference (95 to 15)
        assert_eq!(calculate_score(15, 95, 50, 0), 90);  // Difference is 20 (going around the circle)
        assert_eq!(calculate_score(95, 15, 50, 0), 90);  // Difference is 20 (going around the circle)
        
        // Test circular edge cases
        assert_eq!(calculate_score(0, 100, 50, 0), 130); // Difference is 1 (going around the circle)
        assert_eq!(calculate_score(100, 0, 50, 0), 130); // Difference is 1 (going around the circle)
        
        // Test with misses
        assert_eq!(calculate_score(50, 50, 50, 1), 75);  // (100 + 50) / (1 + 1)
        assert_eq!(calculate_score(50, 50, 50, 2), 50);  // (100 + 50) / (2 + 1)
    }

    #[test]
    fn test_average_score_calculation() {
        // Test with normal values
        assert_eq!(calculate_average_score(&[100, 80, 60, 40, 20]), 60);
        
        // Test with empty array
        assert_eq!(calculate_average_score(&[]), 0);
        
        // Test with one value
        assert_eq!(calculate_average_score(&[75]), 75);
        
        // Test rounding up
        assert_eq!(calculate_average_score(&[10, 11]), 11);  // 10.5 rounds up to 11
    }
}
