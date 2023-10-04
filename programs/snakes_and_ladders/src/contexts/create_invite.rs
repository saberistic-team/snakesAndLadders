use crate::*;

#[derive(Accounts)]
pub struct CreateInvite<'info> {
    #[account(init, payer = signer, space = 8 + Invite::MAXIMUM_SIZE)]
    pub invite: Account<'info, Invite>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub inviter: Account<'info, User>,
    #[account(mut)]
    pub invitee: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateInvite<'info> {

    #[access_control(Self::constraints(&self))]
    pub fn invite_player(
        &mut self,
        game_pk: Pubkey,
        inviter_pk: Pubkey,
        invitee_pk: Pubkey,
    ) -> Result<()> {
        let Self { invite, .. } = self;
        invite.game=game_pk;
        invite.inviter=inviter_pk;
        invite.invitee=invitee_pk;

        Ok(())
    }

    pub fn constraints(&self) -> Result<()> {
        msg!("game:constraints {}",self.game.players[0].pk);

        // if matches!(self.game.players[0].pk,self.inviter.)
        Ok(())
    }
}
