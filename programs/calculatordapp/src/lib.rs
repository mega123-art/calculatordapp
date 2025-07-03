use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("uJ5p9PdfKaPFE47AHERd7x7AB4yMt4seUZv92kwQYHC");

#[program]
pub mod calculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,firstmessage:String) -> ProgramResult<()> {
        let calculator=&mut ctx.accounts.calculator;
        calculator.greeting=firstmessage
        Ok(())
    }

    pub fn add(ctx: Context<Addition>,num1:i64,num2:i64)-> ProgramResult{
        let calculator=&mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }
    pub fn subtract(ctx: Context<Subtraction>,num1:i64,num2:i64)-> ProgramResult{
        let calculator=&mut ctx.accounts.calculator;
        calculator.result = num1 -num2;
        Ok(())
    }
    pub fn multiply(ctx: Context<Multiplication>,num1:i64,num2:i64)-> ProgramResult{
        let calculator=&mut ctx.accounts.calculator;
        calculator.result = num1*num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>,num1:i64,num2:i64)-> ProgramResult{
        let calculator=&mut ctx.accounts.calculator;
        calculator.result = num1/num2;
        calculator.remainder=num1%num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer=user,
        space=8+8+240+8
    )]
    pub calculator:Account<'info,Calculator>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub systemprogram: Program<'info,System>
}
#[derive(Accounts)]
pub struct Addition<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>
}

#[derive(Accounts)]
pub struct Subtraction<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>

}
#[derive(Accounts)]
pub struct Division<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>

}
#[derive(Accounts)]
pub struct Multiplication<'info>{
    #[account(mut)]
    pub calculator: Account<'info,Calculator>

}
#[account]
pub struct Calculator{
    pub greeting:String,
    pub result:i64,
    pub remainder:i64
}
