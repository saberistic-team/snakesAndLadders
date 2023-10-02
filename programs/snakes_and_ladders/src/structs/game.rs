use crate::*;

#[account]
pub struct Game {
    pub board: Vec<BoardItem>,
    pub state: GameState,
    pub players: Vec<Player>,
    pub board_size: u8,
    pub number_of_snakes:u8,
    pub number_of_ladders:u8,
    pub is_private:bool,
    pub is_beatable:bool,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = 8+(90*(2)) + 1 + (8+(10*34)) + 1 + 1 + 1 + 1 + 1 ;
    pub fn status(&mut self) {
        self.state=GameState::Waiting;
    }

}
