use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum PlayerState {
    None,
    MyTurn,
    Stop,
    Won,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::None
    }
}
