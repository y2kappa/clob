mod common;
use common::fixtures::setup_empty_market_with_dependencies;
use solana_program_test::tokio;

#[tokio::test]
async fn test_basic() {
    let mut ctx = setup_empty_market_with_dependencies(&[]).await;
    let _owner = &ctx.payer.clone();
}
