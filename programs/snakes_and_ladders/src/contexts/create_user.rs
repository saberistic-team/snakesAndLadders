use crate::*;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = signer, space = 8 + User::MAXIMUM_SIZE)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUser<'_> {
    pub fn process(
        &mut self,
        name: String,
    ) -> Result<()> {
        let Self { user, .. } = self;
        user.name=name;

        Ok(())
    }
}
