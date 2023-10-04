use anchor_lang::prelude::*;

mod contexts;
mod errors;
mod structs;
mod utils;

pub use contexts::*;
pub use errors::ErrorCode;
pub use structs::*;
pub use utils::*;

declare_id!("GBBsHD9uxT3ggRFDqRCreKCSWJkhyB2czhsi5Vg2nEb8");

#[program]
pub mod snakes_and_ladders{
    use super::*;

    pub fn create_user(
        ctx: Context<CreateUser>,
        name: String,
    ) -> Result<()> {
        msg!("game:create user");
        // return Err(ErrorCode::Initialized.into())
        ctx.accounts.process(name)
    }

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        player: Pubkey,
        board_size:u8,
        number_of_snakes:u8,
        number_of_ladders:u8,
        is_private:bool,
        is_beatable:bool,
    ) -> Result<()> {
        msg!("game:lib init");
        ctx.accounts.process(player,board_size,number_of_snakes,number_of_ladders,is_private,is_beatable)
    }

    pub fn add_player(ctx: Context<SnakesAndLaddersGame>,
        player: Pubkey) -> Result<()> {
        msg!("game:lib add player");
        ctx.accounts.add_player(player);
        Ok(())
    }
    pub fn start_game(ctx: Context<SnakesAndLaddersGame>) -> Result<()> {
        msg!("game:lib add player");
        ctx.accounts.start_game();
        Ok(())
    }
    pub fn move_player(ctx: Context<SnakesAndLaddersGame>,
        player: Pubkey) -> Result<()> {
        msg!("game:lib add player");
        _=ctx.accounts.move_player(player);
        Ok(())
    }

    pub fn invite_player(
        ctx: Context<CreateInvite>,
        game_pk: Pubkey,
        inviter_pk: Pubkey,
        invitee_pk: Pubkey,
    ) -> Result<()> {
        msg!("game:create user");
        ctx.accounts.invite_player(game_pk,inviter_pk,invitee_pk)
    }
}
