extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, NegaScout};
use blobwar::board::Board;

fn main() {
    let board = Board::load("x").expect("failed loading board");
    //let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(AlphaBeta(5), NegaScout(6));
}
