use std::sync::Arc;

use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, transaction::Transaction,
};

use super::setup::Env;

pub async fn initialize<'a>(env: Env<'a>, payer: Arc<Keypair>) -> Transaction {
    let instruction = Instruction {
        program_id: clob::id(),
        accounts: vec![],
        data: vec![],
    };

    Transaction::new_signed_with_payer(
        std::slice::from_ref(&instruction),
        Some(&payer.pubkey()),
        &[payer.as_ref()],
        env.client.get_latest_blockhash().await.unwrap(),
    )
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
