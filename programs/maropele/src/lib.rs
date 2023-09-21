use anchor_lang::prelude::*;

mod contexts;
mod errors;
mod structs;

pub use contexts::*;
pub use errors::ErrorCode;
pub use structs::*;

declare_id!("JDLsRY6pAitTHRRfXq6u1KAztc2fPAZJz7Pw2ChkwgnG");

#[program]
pub mod snakes_and_ladders{
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        player_one: Pubkey,
        player_two: Pubkey,
    ) -> Result<()> {
        msg!("program:initialize_game");
        ctx.accounts.process(player_one, player_two)
    }

    pub fn initialize_board(ctx: Context<SnakesAndLaddersGame>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn player_moves(ctx: Context<SnakesAndLaddersGame>) -> Result<()> {
        ctx.accounts.move_player()
    }
}
