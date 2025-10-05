use anchor_lang::prelude::*;

declare_id!("E1EUbfJkNxMJwi9HyqMwt6yqb2h28QXod73iiGX9f5uK");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
