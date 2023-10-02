use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub struct BoardItem {
    pub state_board: BoardState,
    pub value_board: u8,
}
impl Default for BoardItem {
    fn default() -> Self  {
        Self  {
            value_board: 0,
            state_board: BoardState::Empty,
        }
    }
}