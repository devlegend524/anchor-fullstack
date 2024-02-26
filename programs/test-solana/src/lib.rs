use anchor_lang::prelude::*;

declare_id!("HkJeHa96zGkVhKaEoJBWnxrTcJYqJRxDrYHNF1rwZ9Kc");

#[program]
pub mod test_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
