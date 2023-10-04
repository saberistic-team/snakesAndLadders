use crate::*;

#[account]
pub struct User {
    pub name: String,
}

impl User {
    pub const MAXIMUM_SIZE: usize = 8+ 4+(50*4) ;

}

