// Copyright © Accudo Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress,
    block_executor::config::BlockExecutorConfigFromOnchain,
    chain_id::ChainId,
    transaction::{
        authenticator::{AccountAuthenticator, AnyPublicKey, AnySignature},
        signature_verified_transaction::{
            into_signature_verified_block, SignatureVerifiedTransaction,
        },
        RawTransaction, RawTransactionWithData, Script, SignedTransaction, Transaction,
        TransactionPayload,
    },
};
use accudo_crypto::{ed25519::*, pq::Dilithium3KeyPair, signing_message, traits::*};

const MAX_GAS_AMOUNT: u64 = 1_000_000;
const TEST_GAS_PRICE: u64 = 100;

// The block executor onchain config (gas limit parameters) for executor tests
pub const TEST_BLOCK_EXECUTOR_ONCHAIN_CONFIG: BlockExecutorConfigFromOnchain =
    BlockExecutorConfigFromOnchain::on_but_large_for_test();

static EMPTY_SCRIPT: &[u8] = include_bytes!("empty_script.mv");

// Create an expiration time 'seconds' after now
fn expiration_time(seconds: u64) -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("System time is before the UNIX_EPOCH")
        .as_secs()
        + seconds
}

// Test helper for transaction creation
pub fn get_test_signed_transaction(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    payload: Option<TransactionPayload>,
    expiration_timestamp_secs: u64,
    gas_unit_price: u64,
    max_gas_amount: Option<u64>,
) -> SignedTransaction {
    debug_assert_eq!(public_key, private_key.public_key());
    let pq_keypair = Dilithium3KeyPair::generate().expect("dilithium keypair generation");
    get_test_signed_transaction_dual(
        sender,
        sequence_number,
        private_key,
        &pq_keypair,
        payload,
        expiration_timestamp_secs,
        gas_unit_price,
        max_gas_amount,
    )
}

/// Version of [`get_test_signed_transaction`] that attaches a Dilithium signature alongside the
/// classical signature. This is useful for PQ-aware test scenarios.
pub fn get_test_signed_transaction_dual(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    pq_keypair: &Dilithium3KeyPair,
    payload: Option<TransactionPayload>,
    expiration_timestamp_secs: u64,
    gas_unit_price: u64,
    max_gas_amount: Option<u64>,
) -> SignedTransaction {
    let raw_txn = RawTransaction::new(
        sender,
        sequence_number,
        payload.unwrap_or_else(|| {
            TransactionPayload::Script(Script::new(EMPTY_SCRIPT.to_vec(), vec![], vec![]))
        }),
        max_gas_amount.unwrap_or(MAX_GAS_AMOUNT),
        gas_unit_price,
        expiration_timestamp_secs,
        ChainId::test(),
    );

    raw_txn
        .sign_dual_with_dilithium(Some(private_key), pq_keypair)
        .expect("dual signing for test transaction")
        .into_inner()
}

// Test helper for creating transactions for which the signature hasn't been checked.
pub fn get_test_unchecked_transaction(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    payload: TransactionPayload,
    expiration_time: u64,
    gas_unit_price: u64,
    max_gas_amount: Option<u64>,
) -> SignedTransaction {
    get_test_unchecked_transaction_(
        sender,
        sequence_number,
        private_key,
        public_key,
        payload,
        expiration_time,
        gas_unit_price,
        max_gas_amount,
        ChainId::test(),
    )
}

// Test helper for creating transactions for which the signature hasn't been checked.
fn get_test_unchecked_transaction_(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    payload: TransactionPayload,
    expiration_timestamp_secs: u64,
    gas_unit_price: u64,
    max_gas_amount: Option<u64>,
    chain_id: ChainId,
) -> SignedTransaction {
    debug_assert_eq!(public_key, private_key.public_key());
    let raw_txn = RawTransaction::new(
        sender,
        sequence_number,
        payload,
        max_gas_amount.unwrap_or(MAX_GAS_AMOUNT),
        gas_unit_price,
        expiration_timestamp_secs,
        chain_id,
    );

    let pq_keypair = Dilithium3KeyPair::generate().expect("dilithium keypair generation");
    raw_txn
        .sign_dual_with_dilithium(Some(private_key), &pq_keypair)
        .expect("dual signing for test transaction")
        .into_inner()
}

// Test helper for transaction creation. Short version for get_test_signed_transaction
// Omits some fields
pub fn get_test_signed_txn(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    payload: Option<TransactionPayload>,
) -> SignedTransaction {
    let expiration_time = expiration_time(10);
    get_test_signed_transaction(
        sender,
        sequence_number,
        private_key,
        public_key,
        payload,
        expiration_time,
        TEST_GAS_PRICE,
        None,
    )
}

pub fn get_test_unchecked_txn(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    payload: TransactionPayload,
) -> SignedTransaction {
    let expiration_time = expiration_time(10);
    get_test_unchecked_transaction(
        sender,
        sequence_number,
        private_key,
        public_key,
        payload,
        expiration_time,
        TEST_GAS_PRICE,
        None,
    )
}

pub fn get_test_unchecked_multi_agent_txn(
    sender: AccountAddress,
    secondary_signers: Vec<AccountAddress>,
    sequence_number: u64,
    sender_private_key: &Ed25519PrivateKey,
    sender_public_key: Ed25519PublicKey,
    secondary_private_keys: Vec<&Ed25519PrivateKey>,
    secondary_public_keys: Vec<Ed25519PublicKey>,
    script: Option<Script>,
) -> SignedTransaction {
    let expiration_time = expiration_time(10);
    let raw_txn = RawTransaction::new(
        sender,
        sequence_number,
        TransactionPayload::Script(
            script.unwrap_or_else(|| Script::new(EMPTY_SCRIPT.to_vec(), vec![], Vec::new())),
        ),
        MAX_GAS_AMOUNT,
        TEST_GAS_PRICE,
        expiration_time,
        ChainId::test(),
    );
    let message =
        RawTransactionWithData::new_multi_agent(raw_txn.clone(), secondary_signers.clone());

    let signing_bytes = signing_message(&message).expect("multi-agent signing message");

    let sender_pq_keypair = Dilithium3KeyPair::generate().expect("sender Dilithium keypair");
    let sender_pq_signature = sender_pq_keypair
        .sign(&signing_bytes)
        .expect("sender Dilithium signing");
    let sender_signature = sender_private_key.sign(&message).unwrap();
    let sender_authenticator = AccountAuthenticator::single_key_dual(
        Some((
            AnyPublicKey::ed25519(sender_public_key.clone()),
            AnySignature::ed25519(sender_signature),
        )),
        sender_pq_signature.scheme,
        sender_pq_keypair.public_key().to_vec(),
        sender_pq_signature,
    );

    let mut secondary_authenticators = Vec::with_capacity(secondary_public_keys.len());
    for (priv_key, pub_key) in secondary_private_keys
        .into_iter()
        .zip(secondary_public_keys)
    {
        let classical_signature = priv_key.sign(&message).unwrap();
        let pq_keypair = Dilithium3KeyPair::generate().expect("secondary Dilithium keypair");
        let pq_signature = pq_keypair
            .sign(&signing_bytes)
            .expect("secondary Dilithium signing");
        secondary_authenticators.push(AccountAuthenticator::single_key_dual(
            Some((
                AnyPublicKey::ed25519(pub_key.clone()),
                AnySignature::ed25519(classical_signature),
            )),
            pq_signature.scheme,
            pq_keypair.public_key().to_vec(),
            pq_signature,
        ));
    }

    SignedTransaction::new_multi_agent(
        raw_txn,
        sender_authenticator,
        secondary_signers,
        secondary_authenticators,
    )
}

pub fn get_test_txn_with_chain_id(
    sender: AccountAddress,
    sequence_number: u64,
    private_key: &Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    chain_id: ChainId,
) -> SignedTransaction {
    debug_assert_eq!(public_key, private_key.public_key());
    let expiration_time = expiration_time(10);
    let raw_txn = RawTransaction::new_script(
        sender,
        sequence_number,
        Script::new(EMPTY_SCRIPT.to_vec(), vec![], Vec::new()),
        MAX_GAS_AMOUNT,
        TEST_GAS_PRICE,
        expiration_time,
        chain_id,
    );

    let pq_keypair = Dilithium3KeyPair::generate().expect("dilithium keypair generation");
    raw_txn
        .sign_dual_with_dilithium(Some(private_key), &pq_keypair)
        .expect("dual signing for test transaction")
        .into_inner()
}

pub fn block(user_txns: Vec<Transaction>) -> Vec<SignatureVerifiedTransaction> {
    into_signature_verified_block(user_txns)
}

pub fn get_test_raw_transaction(
    sender: AccountAddress,
    sequence_number: u64,
    payload: Option<TransactionPayload>,
    expiration_timestamp_secs: Option<u64>,
    gas_unit_price: Option<u64>,
    max_gas_amount: Option<u64>,
) -> RawTransaction {
    RawTransaction::new(
        sender,
        sequence_number,
        payload.unwrap_or_else(|| {
            TransactionPayload::Script(Script::new(EMPTY_SCRIPT.to_vec(), vec![], vec![]))
        }),
        max_gas_amount.unwrap_or(MAX_GAS_AMOUNT),
        gas_unit_price.unwrap_or(TEST_GAS_PRICE),
        expiration_timestamp_secs.unwrap_or(expiration_time(10)),
        ChainId::test(),
    )
}
