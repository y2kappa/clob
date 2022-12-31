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
