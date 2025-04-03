//! Error handling for the game.
//! 
//! This module defines custom error types used throughout the application.

use std::fmt;
use std::error::Error;
use std::io;

/// Represents errors that can occur within the game.
#[derive(Debug)]
pub enum GameError {
    /// An error occurred during I/O operations.
    IoError(io::Error),
    /// An error related to game logic.
    LogicError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::IoError(err) => write!(f, "I/O error: {}", err),
            GameError::LogicError(msg) => write!(f, "Game logic error: {}", msg),
        }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GameError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        GameError::IoError(err)
    }
}

/// Shorthand Result type for the game.
pub type GameResult<T> = Result<T, GameError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_error_display() {
        let io_error = GameError::IoError(io::Error::new(io::ErrorKind::Other, "test io error"));
        let logic_error = GameError::LogicError("game state error".to_string());
        
        assert!(io_error.to_string().contains("I/O error"));
        assert!(logic_error.to_string().contains("Game logic error: game state error"));
    }
    
    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let game_error = GameError::from(io_error);
        
        match game_error {
            GameError::IoError(_) => assert!(true),
            _ => assert!(false, "Expected IoError variant"),
        }
    }
}
