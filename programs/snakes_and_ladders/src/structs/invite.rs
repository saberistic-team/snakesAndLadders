use crate::*;

#[account]
pub struct Invite {
    pub game: Pubkey,
    pub inviter: Pubkey,
    pub invitee: Pubkey,
}

impl Invite {
    pub const MAXIMUM_SIZE: usize = 8+ 32 + 32 + 32 ;

}

