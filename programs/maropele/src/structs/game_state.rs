use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum GameState {
    Waiting,
    PlayerOne,
    PlayerTwo,
    PlayerOneWon,
    PlayerTwoWon,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Waiting
    }
}
