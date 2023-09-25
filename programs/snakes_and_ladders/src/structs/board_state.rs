use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum BoardState {
    Empty,
    Snake,
    Stair,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState::Empty
    }
}
