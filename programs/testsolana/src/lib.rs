use anchor_lang::prelude::*;

declare_id!("9px9BwRqAwGaFfkkx2yWiqZt2rCdeVSPhkpZmQz8tdgt");

#[program]
pub mod testsolana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
