use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Deck is empty")]
    EmptyDeck,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error)
}