use std::sync::Arc;

use solana_program_test::ProgramTest;
use solana_sdk::{
    account::{Account, AccountSharedData},
    native_token::sol_to_lamports,
    signature::Keypair,
    signer::Signer,
    system_program,
};

use super::{runner::test, types::TestContext};

pub async fn setup_ctx() -> TestContext {
    let mut program = test::program();
    let admin = funded_kp(&mut program, SOL::from(10.0));
    let ctx = test::start(program, &admin).await;
    ctx
}

pub type KP = Arc<Keypair>;
pub fn kp() -> KP {
    Arc::new(Keypair::new())
}

pub fn fund_kp(test: &mut ProgramTest, min_balance_lamports: u64, user: Arc<Keypair>) -> KP {
    test.add_account(
        user.pubkey(),
        Account {
            lamports: min_balance_lamports,
            ..Account::default()
        },
    );
    user
}

pub fn funded_kp(test: &mut ProgramTest, min_balance_lamports: u64) -> KP {
    fund_kp(test, min_balance_lamports, kp())
}

pub fn funded_new_kp(test: &mut TestContext, min_balance_lamports: u64) -> KP {
    fund_new_kp(test, min_balance_lamports, kp())
}

pub fn fund_new_kp(test: &mut TestContext, min_balance_lamports: u64, user: Arc<Keypair>) -> KP {
    let account = AccountSharedData::new(min_balance_lamports, 0, &system_program::ID);
    test.context.set_account(&user.pubkey(), &account);
    user
}

pub fn funded_new_kps<const NUM: usize>(
    test: &mut TestContext,
    min_balance_lamports: u64,
) -> [KP; NUM] {
    (0..NUM)
        .map(|_| funded_new_kp(test, min_balance_lamports))
        .collect::<Vec<KP>>()
        .try_into()
        .unwrap()
}

pub fn funded_kps<const NUM: usize>(
    test: &mut ProgramTest,
    min_balance_lamports: u64,
) -> [KP; NUM] {
    (0..NUM)
        .map(|_| funded_kp(test, min_balance_lamports))
        .collect::<Vec<KP>>()
        .try_into()
        .unwrap()
}

pub struct SOL;
impl SOL {
    pub fn one() -> u64 {
        Self::from(1.0)
    }
    pub fn from(amt: f64) -> u64 {
        sol_to_lamports(amt)
    }
}
