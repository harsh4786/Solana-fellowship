use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod identity {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, username: String, bump: u8) -> Result<()> {
        if username.len() > User::STRING_SIZE{
            return Err(error!(ErrorCode::UsernameTooLong));
        }
        ctx.accounts.user.username = username;
        ctx.accounts.user.bump = bump;
        ctx.accounts.user.authority = ctx.accounts.authority.key();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String, bump:u8)]
pub struct CreateUser<'info> {
    #[account(init, 
        seeds = [b"username", username.as_bytes()], bump, payer = authority, space = User::SIZE)]
    user: Account<'info, User>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}
#[account]
pub struct User {
   username: String,
   authority: Pubkey,
   bump: u8,
}
impl User {
    const STRING_SIZE: usize = 140;
    const SIZE: usize = 8 + 1 + 32 + Self::STRING_SIZE;
}
#[error_code]
pub enum ErrorCode{
    #[msg("Please provide a username with less than 140 characters")]
    UsernameTooLong,
}