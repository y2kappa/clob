mod common;
use common::setup::setup_ctx;
use solana_program_test::tokio;

#[tokio::test]
async fn test_basic() {
    let mut ctx = setup_ctx().await;
    let _owner = &ctx.payer.clone();
}
