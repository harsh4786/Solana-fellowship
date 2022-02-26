use anchor_lang::prelude::*;

declare_id!("FzcGTXnN67jFKEUoJTfE4mLkP9yK9oY2wKLFbYDzvPyL");

#[program]
pub mod rust_calculator {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, msg: String )-> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greet = msg;


        Ok(())
    }
    pub fn add(ctx: Context<Add>,value_1: u64, value_2: u64 ) -> Result<()> {
        let mut calc = &mut ctx.accounts.calculator;
        calc.result = value_1 + value_2;
        Ok(())
    }
    pub fn subtract(ctx: Context<Subtract>,value_1: u64, value_2: u64 ) -> Result<()> {
        let mut calc = &mut ctx.accounts.calculator;
        calc.result = value_1 - value_2;
        Ok(())
    }
    pub fn multiply(ctx: Context<Multiply>,value_1: u64, value_2: u64 ) -> Result<()> {
        let mut calc = &mut ctx.accounts.calculator;
        calc.result = value_1 * value_2;
        Ok(())
    }
    pub fn divide(ctx: Context<Divide>,value_1: u64, value_2: u64 ) -> Result<()>{
        let mut calc = &mut ctx.accounts.calculator;
        calc.result = value_1 / value_2;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub calculator: Account<'info,Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>,
}
#[derive(Accounts)]
pub struct Add<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>,
}
#[derive(Accounts)]
pub struct Subtract<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>,
}
#[derive(Accounts)]
pub struct Multiply<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>,
}
#[derive(Accounts)]
pub struct Divide<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>,
}

#[account]
pub struct Calculator {
    pub greet: String,
    pub remainder: u64,
    pub result: u64,
}
