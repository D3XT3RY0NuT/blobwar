//! Implementation of the min max algorithm.
use super::Strategy;
use super::Greedy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use itertools::Itertools;
use std::thread;

/// Min-Max algorithm with a given recursion depth.
pub struct ParallelMinMax(pub u8);

impl ParallelMinMax {
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

impl Strategy for ParallelMinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // Minimax of depth 1 is equivalent to a greedy approach
        if self.0 == 1 {
            return Greedy().compute_next_move(state);
        }
        let mut best_move = None;
        let mut best_value = std::i8::MIN + 1;
        let cpus = num_cpus::get();
        let chunks = state.movements().chunks(cpus);
        for chunk in &chunks {
            let result = thread::spawn(move || {
                let best_value = std::i8::MIN + 1;
                for mov in chunk {
                    /*
                     *let next_state = state.play(&mov);
                     *let value = -self.evaluate(&next_state); 
                     */
                }
            });
        }
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

impl fmt::Display for ParallelMinMax {
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
        movement.store(ParallelMinMax(depth).compute_next_move(state));
    }
}
