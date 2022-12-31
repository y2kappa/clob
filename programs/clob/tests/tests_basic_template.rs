mod common;
use common::{
    client,
    setup::{funded_new_kp, setup_ctx, SOL},
};
use solana_program_test::tokio;

#[tokio::test]
async fn test_basic() {
    let mut ctx = setup_ctx().await;

    let owner = funded_new_kp(&mut ctx, SOL::one());

    let res = client::initialize(&mut ctx, owner).await;
    println!("Initialized {:?}", res);
}
