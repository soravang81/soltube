use anchor_lang::prelude::*;

declare_id!("AvDYsky4CnTp59GY1Ws1JFAmeXzBuracMTycJXRLGkmn");

#[program]
pub mod soltube {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
