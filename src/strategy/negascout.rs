//! Implementation of the NegaScout algorithm.
use super::Strategy;
use super::Greedy;
use crate::configuration::{Configuration, Movement};
use std::fmt;
use crate::shmem::AtomicMove;
extern crate queues;

static LOWER_BOUND: i8 = -10;
static UPPER_BOUND: i8 = 10;
const MAX_DEPTH: u8 = 10;

/// NegaScout algorithm with a given recursion depth.
pub struct NegaScout(pub u8);

impl NegaScout {
    fn evaluate(&mut self, state: &Configuration, mut alpha: i8, beta: i8) -> i8 {
        if self.0 == 1 {
            return Greedy().evaluate(state);
        }
        let mut beta_prim = beta; 
        let mut best_value = std::i8::MIN + 1;
        for mov in state.movements() {
            let next_state = state.play(&mov);
            self.0 -= 1;
            let mut value = -self.evaluate(&next_state, -beta_prim, -alpha);
            if alpha < value && value < beta && beta_prim != beta {
                value = -self.evaluate(&next_state, -beta, -alpha);
            }
            self.0 += 1;
            if value > best_value {
                best_value = value;
                if value > alpha {
                    alpha = value;
                }
            }
            
            if alpha >= beta {
                return alpha;
            }

            beta_prim = alpha + 1;
        }

        best_value
    }

/*
 *    fn next_move(&mut self, state: &Configuration) -> Option<Movement> {
 *        // NegaScout of depth 1 is equivalent to Greedy
 *        if self.0 == 1 {
 *            return Greedy().compute_next_move(state);
 *        }
 *        let mut best_move = None;
 *        let mut best_value = LOWER_BOUND; 
 *        let beta = UPPER_BOUND;
 *        let mut beta_prim = beta;
 *        for mov in state.movements() {
 *            let next_state = state.play(&mov);
 *            self.0 -= 1;
 *            let mut value = -self.evaluate(&next_state, -beta_prim, -best_value);
 *            if best_value < value && beta_prim != beta {
 *                println!("OUI");
 *                value = -self.evaluate(&next_state, -beta, -best_value);
 *            }
 *            self.0 += 1;
 *            if value > best_value {
 *                best_value = value;
 *                best_move = Some(mov);
 *            }
 *
 *            if best_value >= beta {
 *                return best_move
 *            }
 *
 *            beta_prim = best_value + 1;
 *        }
 *
 *        best_move
 *    }
 */
}

impl Strategy for NegaScout {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // NegaScout of depth 1 is equivalent to Greedy
        if self.0 == 1 {
            return Greedy().compute_next_move(state);
        }
        let mut best_move = None;
        let mut best_value = LOWER_BOUND; 
        let beta = UPPER_BOUND;
        let mut beta_prim = beta;
        for mov in state.movements() {
            let next_state = state.play(&mov);
            self.0 -= 1;
            let mut value = -self.evaluate(&next_state, -beta_prim, -best_value);
            if best_value < value && beta_prim != beta {
                println!("OUI");
                value = -self.evaluate(&next_state, -beta, -best_value);
            }
            self.0 += 1;
            if value > best_value {
                best_value = value;
                best_move = Some(mov);
            }

            if best_value >= beta {
                return best_move
            }

            beta_prim = best_value + 1;
        }

        best_move
    }
}

impl fmt::Display for NegaScout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NegaScout (max level: {})", self.0)
    }
}

/// Anytime NegaScout algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn negascout_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..MAX_DEPTH {
        let chosen_movement = NegaScout(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}
