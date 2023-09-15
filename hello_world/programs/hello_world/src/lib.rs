use anchor_lang::prelude::*;

declare_id!("PEnBQGAbFqZHnaWEaeu1Eu6MbbM5CNBw6oTWsSejAbB");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
