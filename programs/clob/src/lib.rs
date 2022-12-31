use anchor_lang::prelude::*;
mod handlers;
use crate::handlers::*;
declare_id!("2o8FbkLWzfGuFCXpYzbjdKBzp8FNpYbQThDV25Vz4WD9");

#[program]
pub mod clob {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handlers::handler_initialize::process(ctx)
    }
}
