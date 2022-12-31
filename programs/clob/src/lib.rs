use anchor_lang::prelude::*;
mod handlers;
use crate::handlers::*;
declare_id!("2o8FbkLWzfGuFCXpYzbjdKBzp8FNpYbQThDV25Vz4WD9");

#[program]
pub mod clob {
    use super::*;

    pub fn initialize_market(ctx: Context<Initialize>) -> Result<()> {
        handlers::handler_initialize::process(ctx)
    }

    // pub fn place_limit_order(ctx: Context<PlaceLimitOrder>) -> Result<()> {
    //     handlers::handler_place_limit_order::process(ctx)
    // }

    // pub fn place_market_order(ctx: Context<PlaceMarketOrder>) -> Result<()> {
    //     handlers::handler_place_market_order::process(ctx)
    // }

    // pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
    //     handlers::handler_cancel_order::process(ctx)
    // }

    // pub fn amend_order(ctx: Context<AmendOrder>) -> Result<()> {
    //     handlers::handler_amend_order::process(ctx)
    // }
}
