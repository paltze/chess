//! # Implements the core functionality of the program
//!

pub mod board;
pub mod moves;

#[derive(thiserror::Error, Debug)]
pub enum CoreErrors {
    #[error(transparent)]
    Moves(#[from] crate::core::moves::MovesError),
    #[error(transparent)]
    Board(#[from] crate::core::board::BoardError)
}
