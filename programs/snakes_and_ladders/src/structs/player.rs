use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub struct Player {
    pub pk: Pubkey,
    pub position: u8,
    pub state: PlayerState,
}

pub fn create_player(pubkey:Pubkey)->Player{
    Player{
        pk:pubkey,
        position:0,
        state:PlayerState::Waiting,
    }
}

