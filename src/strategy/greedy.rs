//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::fmt;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Greedy {
    /// Computes the value of the best immediate move
    pub fn evaluate(&mut self, state: &Configuration) -> i8 {
        let mut best_value = std::i8::MIN + 1;
        for mov in state.movements() {
            let value = state.play(&mov).value();
            if value > best_value {
                best_value = value;
            }
        }

        best_value
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut best_value = std::i8::MIN + 1;
        let mut best_move = None;
        for mov in state.movements() {
            let value = state.play(&mov).value();
            if value > best_value {
                best_value = value;
                best_move = Some(mov);
            }
        }

        best_move
    }
}
