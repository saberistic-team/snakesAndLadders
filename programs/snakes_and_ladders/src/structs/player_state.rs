use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum PlayerState {
    Waiting,
    MyTurn,
    Stop,
    Won,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Waiting
    }
}
