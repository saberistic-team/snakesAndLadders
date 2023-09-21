use crate::*;

#[account]
pub struct Game {
    pub board: [BoardItem; 90],
    pub state: GameState,
    pub player_one: Pubkey,
    pub player_two: Pubkey,
    pub pos_player_one: u8,
    pub pos_player_two: u8,
}

impl Game {
    pub fn status(&mut self) {
        self.state=GameState::Waiting;
    }

}
