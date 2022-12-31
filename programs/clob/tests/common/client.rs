use solana_program_test::BanksClientError;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, transaction::Transaction,
};
use std::sync::Arc;

use super::types::TestContext;

pub async fn initialize_market<'a>(
    ctx: &mut TestContext,
    admin: Arc<Keypair>,
) -> Result<(), BanksClientError> {
    let data = {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&dispatch_sig("global", "initialize_market"));
        data
    };

    let instruction = Instruction {
        program_id: clob::id(),
        accounts: vec![crate::writable_signer!(admin.pubkey())],
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        std::slice::from_ref(&instruction),
        Some(&admin.pubkey()),
        &[admin.as_ref()],
        ctx.context
            .banks_client
            .get_latest_blockhash()
            .await
            .unwrap(),
    );

    ctx.context
        .banks_client
        .process_transaction_with_commitment(tx, CommitmentLevel::Processed)
        .await
}

fn dispatch_sig(namespace: &str, name: &str) -> [u8; 8] {
    use sha2::Digest;
    use sha2::Sha256;
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0; 8];
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    sighash.copy_from_slice(&hasher.finalize()[..8]);
    sighash
}

#[macro_export]
macro_rules! readable {
    ($res:expr) => {
        AccountMeta::new_readonly($res, false)
    };
}

#[macro_export]
macro_rules! signer {
    ($res:expr) => {
        AccountMeta::new_readonly($res, true)
    };
}

#[macro_export]
macro_rules! writable {
    ($res:expr) => {
        AccountMeta::new($res, false)
    };
}

#[macro_export]
macro_rules! writable_signer {
    ($res:expr) => {
        AccountMeta::new($res, true)
    };
}

#[macro_export]
macro_rules! send_transaction {
    ($ctx: expr, $transaction: expr) => {
        $ctx.context
            .banks_client
            .process_transaction_with_commitment(
                $transaction,
                solana_sdk::commitment_config::CommitmentLevel::Processed,
            )
            .await
    };
}
