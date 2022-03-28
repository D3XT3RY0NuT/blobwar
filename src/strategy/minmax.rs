//! Implementation of the min max algorithm.
use super::Strategy;
use super::Greedy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl MinMax {
    fn evaluate(&mut self, state: &Configuration) -> i8 {
        if self.0 == 1 {
            return Greedy().evaluate(state);
        }
        let mut best_value = std::i8::MIN + 1;
        for mov in state.movements() {
            let next_state = state.play(&mov);
            self.0 -= 1;
            let value = -self.evaluate(&next_state);
            self.0 += 1;
            if value > best_value {
                best_value = value;
            }
        }

        best_value
    }
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // Minimax of depth 1 is equivalent to a greedy approach
        if self.0 == 1 {
            return Greedy().compute_next_move(state);
        }
        let mut best_move = None;
        let mut best_value = std::i8::MIN + 1;
        for mov in state.movements() {
            let next_state = state.play(&mov);
            self.0 -= 1;
            let value = -self.evaluate(&next_state);
            self.0 += 1;
            if value > best_value {
                best_value = value;
                best_move = Some(mov);
            }
        }

        best_move
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
