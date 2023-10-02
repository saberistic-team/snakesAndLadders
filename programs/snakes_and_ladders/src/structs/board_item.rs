use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub struct BoardItem {
    pub type_board: BoardState,
    pub value_board: u8,
}
impl Default for BoardItem {
    fn default() -> Self  {
        Self  {
            value_board: 0,
            type_board: BoardState::Empty,
        }
    }
}