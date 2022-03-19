use anchor_lang::prelude::*;

declare_id!("kKvQQP5sKyTDt33PWYwpT77Ju73gMXE6f3XcFDqtbXX");

#[program]
pub mod solblog {
    use std::str::from_utf8;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let blog= &mut ctx.accounts.blog_account;
        blog.blogger = *ctx.accounts.blogger.key;
        Ok(())
    }
    pub fn make_post(ctx: Context<MakePost>, new_post: Vec<u8>) -> Result<()> {
        let post = from_utf8(&new_post).map_err(|err| {
            msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(post);
        let blog = &mut ctx.accounts.blog_account;
        blog.blog = new_post;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(init, payer = blogger, space = 8 + 32 + 566)]
    blog_account: Account<'info, BlogAccount>,
    #[account(mut)]
    blogger: Signer<'info>,
    system_program: Program<'info,System>,
}
#[derive(Accounts)]
pub struct MakePost<'info>{
    #[account(mut, has_one = blogger)]
    blog_account: Account<'info, BlogAccount>,
    blogger: Signer<'info>,
   
}


#[account]
pub struct BlogAccount{
    blogger: Pubkey,
    blog: Vec<u8>
}