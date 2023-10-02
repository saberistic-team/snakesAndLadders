use crate::*;
#[derive(Accounts)]
pub struct SnakesAndLaddersGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

impl<'info> SnakesAndLaddersGame<'info> {
    pub fn add_player(&mut self, player_pk: Pubkey)->Result<()>{
        let Self { game, .. } = self;
        msg!("game:add player");
        let new_player=create_player(player_pk);
        game.players.push(new_player);
        Ok(())
    }
    
    pub fn move_player(&mut self) -> Result<()> { 

       Ok(())
   }
}
