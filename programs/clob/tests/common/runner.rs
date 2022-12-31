use std::sync::Arc;
use std::time::Duration;

use anchor_lang::prelude::{Clock, Pubkey};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::hash::Hash;
use solana_sdk::program_error::ProgramError;
use solana_sdk::program_pack::Pack;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::transport::TransportError;
use solana_sdk::{system_instruction, system_program};
use spl_associated_token_account as ata;
use spl_token::state::Mint;

use crate::common::setup::KP;

use super::types::TestContext;

pub mod test {
    use solana_program_test::{processor, ProgramTest};

    use crate::common::types::TestContext;

    use super::*;
    pub fn program() -> ProgramTest {
        let program_test = ProgramTest::new("clob", clob::ID, processor!(clob::entry));
        program_test
    }

    pub async fn start(test: ProgramTest, payer: &KP) -> TestContext {
        let mut context = test.start_with_context().await;
        let rent = context.banks_client.get_rent().await.unwrap();

        TestContext {
            context,
            rent,
            payer: payer.clone(),
        }
    }
}

pub mod state {

    use anchor_lang::{AccountDeserialize, Discriminator};
    use solana_sdk::account::Account;

    use crate::common::types::TestError;

    use super::*;
    pub async fn get<T: AccountDeserialize + Discriminator>(
        env: &mut TestContext,
        address: Pubkey,
    ) -> T {
        let acc = try_get::<T>(env, address).await;
        acc.unwrap()
    }
    pub async fn try_get<T: AccountDeserialize + Discriminator>(
        env: &mut TestContext,
        address: Pubkey,
    ) -> Result<T, TestError> {
        match env
            .context
            .banks_client
            .get_account(address)
            .await
            .map_err(|e| {
                println!("Error {:?}", e);
                TestError::UnknownError
            })? {
            Some(data) => deserialize::<T>(&data).map_err(|e| {
                println!("Error {:?}", e);
                TestError::CannotDeserialize
            }),
            None => return Err(TestError::AccountNotFound),
        }
    }
    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }

    pub fn deserialize<T: AccountDeserialize + Discriminator>(
        account: &Account,
    ) -> Result<T, TestError> {
        let discriminator = &account.data[..8];
        if discriminator != T::discriminator() {
            return Err(TestError::BadDiscriminator);
        }

        let mut data: &[u8] = &account.data;
        let user: T = T::try_deserialize(&mut data).map_err(|_| TestError::CannotDeserialize)?;

        return Ok(user);
    }
}

pub mod token {
    use arrayref::array_ref;
    use solana_program_test::BanksClientError;

    use crate::{common::types::TestContext, send_transaction};

    use super::*;

    pub async fn create_ata(env: &mut TestContext, user: &KP, mint: &Pubkey) -> Pubkey {
        let address = ata::get_associated_token_address(&user.pubkey(), mint);
        let instruction =
            ata::create_associated_token_account(&user.pubkey(), &user.pubkey(), mint);
        let transaction = Transaction::new_signed_with_payer(
            std::slice::from_ref(&instruction),
            Some(&user.pubkey()),
            &[user.as_ref()],
            env.context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await
            .unwrap();
        address
    }

    pub async fn create_token_account(
        test_ctx: &mut TestContext,
        payer: Arc<Keypair>,
        account: &KP,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Result<(), BanksClientError> {
        let rent = test_ctx.context.banks_client.get_rent().await.unwrap();
        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &account.pubkey(),
                    rent.minimum_balance(spl_token::state::Account::LEN),
                    spl_token::state::Account::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_account(
                    &spl_token::id(),
                    &account.pubkey(),
                    mint,
                    owner,
                )
                .unwrap(),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(
            &[payer.as_ref(), account],
            test_ctx
                .context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        test_ctx
            .context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn create_mint(
        test_ctx: &mut TestContext,
        mint: &KP,
        decimals: u8,
        mint_authority: &Pubkey,
    ) {
        let rent = test_ctx.context.banks_client.get_rent().await.unwrap();
        let mint_rent = rent.minimum_balance(Mint::LEN);

        let transaction = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &test_ctx.payer.pubkey(),
                    &mint.pubkey(),
                    mint_rent,
                    Mint::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &mint.pubkey(),
                    mint_authority,
                    None,
                    decimals,
                )
                .unwrap(),
            ],
            Some(&test_ctx.payer.pubkey()),
            &[&test_ctx.payer, mint.as_ref()],
            test_ctx
                .context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        send_transaction!(test_ctx, transaction).unwrap();
    }

    pub async fn create_mint_with_freeze_auth(
        test_ctx: &mut TestContext,
        mint: &KP,
        decimals: u8,
        mint_authority: &Pubkey,
    ) {
        let rent = test_ctx.context.banks_client.get_rent().await.unwrap();
        let mint_rent = rent.minimum_balance(Mint::LEN);

        let transaction = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &test_ctx.payer.pubkey(),
                    &mint.pubkey(),
                    mint_rent,
                    Mint::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &mint.pubkey(),
                    mint_authority,
                    Some(mint_authority),
                    decimals,
                )
                .unwrap(),
            ],
            Some(&test_ctx.payer.pubkey()),
            &[&test_ctx.payer, mint.as_ref()],
            test_ctx
                .context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        send_transaction!(test_ctx, transaction).unwrap();
    }

    pub async fn mint_to(
        env: &mut TestContext,
        mint: &Pubkey,
        mint_into_account: &Pubkey,
        amount: u64,
    ) -> Result<(), TransportError> {
        let transaction = Transaction::new_signed_with_payer(
            &[spl_token::instruction::mint_to(
                &spl_token::id(),
                mint,
                mint_into_account,
                &env.payer.pubkey(),
                &[],
                amount,
            )
            .unwrap()],
            Some(&env.payer.pubkey()),
            &[&env.payer, env.payer.as_ref()],
            env.context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await?;
        Ok(())
    }

    pub async fn mint_with_authority(
        env: &mut TestContext,
        mint: &Pubkey,
        mint_into_account: &Pubkey,
        authority: &KP,
        amount: u64,
    ) -> Result<(), TransportError> {
        let transaction = Transaction::new_signed_with_payer(
            &[spl_token::instruction::mint_to(
                &spl_token::id(),
                mint,
                mint_into_account,
                &authority.pubkey(),
                &[],
                amount,
            )
            .unwrap()],
            Some(&env.payer.pubkey()),
            &[&env.payer, authority.as_ref()],
            env.context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await?;
        Ok(())
    }

    pub async fn transfer(
        env: &mut TestContext,
        from: &Pubkey,
        to: &Pubkey,
        signer: &Keypair,
        amount: u64,
    ) -> Result<(), TransportError> {
        let transaction = Transaction::new_signed_with_payer(
            &[spl_token::instruction::transfer(
                &spl_token::id(),
                &from,
                to,
                &signer.pubkey(),
                &[],
                amount,
            )
            .unwrap()],
            Some(&signer.pubkey()),
            &[signer],
            env.context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await?;
        Ok(())
    }

    fn check_data_len(data: &[u8], min_len: usize) -> Result<(), ProgramError> {
        if data.len() < min_len {
            Err(ProgramError::AccountDataTooSmall)
        } else {
            Ok(())
        }
    }

    fn get_token_balance(data: &[u8]) -> u64 {
        check_data_len(&data, spl_token::state::Account::get_packed_len()).unwrap();
        let amount = array_ref![data, 64, 8];

        u64::from_le_bytes(*amount)
    }

    pub async fn balance(env: &mut TestContext, account: &Pubkey) -> u64 {
        let acc = env
            .context
            .banks_client
            .get_account(*account)
            .await
            .unwrap()
            .unwrap();

        get_token_balance(&acc.data)
    }
}

impl TestContext {
    pub async fn fast_forward_minutes(&mut self, minutes: u64) {
        self.fast_forward(Duration::from_secs(minutes * 60)).await
    }

    pub async fn fast_forward_seconds(&mut self, seconds: u64) {
        self.fast_forward(Duration::from_secs(seconds)).await
    }

    async fn fast_forward(&mut self, duration: Duration) {
        let mut clock = self
            .context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .unwrap();
        let target = clock.unix_timestamp + duration.as_secs() as i64;

        while clock.unix_timestamp <= target {
            // The exact time is not deterministic, we have to keep wrapping by arbitrary 400 slots
            self.context.warp_to_slot(clock.slot + 2 * 400).unwrap();
            clock = self
                .context
                .banks_client
                .get_sysvar::<Clock>()
                .await
                .unwrap();
        }
    }

    pub async fn get_recent_blockhash(&mut self) -> Hash {
        self.context
            .banks_client
            .get_latest_blockhash()
            .await
            .unwrap()
    }

    pub async fn get_now_timestamp(&mut self) -> u64 {
        let clock: Clock = self
            .context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .unwrap();
        clock.unix_timestamp as u64
    }

    pub async fn new_keypair(&mut self, min_lamports: u64) -> Arc<Keypair> {
        let account = Keypair::new();
        let transaction = Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &self.context.payer.pubkey(),
                &account.pubkey(),
                min_lamports,
                0,
                &system_program::id(),
            )],
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer, &account],
            self.context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );

        self.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await
            .unwrap();

        Arc::new(account)
    }
}
