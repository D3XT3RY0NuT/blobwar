//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use super::Greedy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;


/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl AlphaBeta {
    fn evaluate(&mut self, state: &Configuration, mut alpha: i8, beta: i8) -> i8 {
        if self.0 == 1 {
            return Greedy().evaluate(state);
        }
        let mut best_value = std::i8::MIN + 1;
        for mov in state.movements() {
            let next_state = state.play(&mov);
            self.0 -= 1;
            let value = -self.evaluate(&next_state, -beta, -alpha);
            self.0 += 1;
            
            if value > best_value {
                best_value = value;
                if value > alpha {
                    alpha = value;
                }
            }

            if alpha >= beta{
                return alpha;
            }
        }

        best_value
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // AlphaBeta of depth 1 is equivalent to Greedy
        if self.0 == 1 {
            return Greedy().compute_next_move(state);
        }
        let mut best_move = None;
        let mut best_value = std::i8::MIN + 1;
        for mov in state.movements() {
            let next_state = state.play(&mov);
            self.0 -= 1;
            let value = -self.evaluate(&next_state, -std::i8::MAX, -best_value);
            self.0 += 1;
            if value > best_value {
                best_value = value;
                best_move = Some(mov);
            }
        }

        best_move
    }
}

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}
