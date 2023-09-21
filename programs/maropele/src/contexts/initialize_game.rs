use crate::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, payer = signer, space = 8 + (90*(2)) + 1 + 32 + 32 + 1 + 1)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'_> {
    pub fn process(&mut self, player_one: Pubkey, player_two: Pubkey) -> Result<()> {
        let Self { game, .. } = self;
        msg!("program:init process");
        game.state = GameState::PlayerOne;
        game.player_one = player_one;
        game.player_two = player_two;
        game.pos_player_one = 0;
        game.pos_player_two = 0;

        Ok(())
    }
}
