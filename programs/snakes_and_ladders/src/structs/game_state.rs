use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum GameState {
    Waiting,
    Start,
    End
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Waiting
    }
}
