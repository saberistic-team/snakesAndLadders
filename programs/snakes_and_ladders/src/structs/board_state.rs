use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq,Debug)]
pub enum BoardState {
    Empty,
    Snake,
    Ladder,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState::Empty
    }
}
