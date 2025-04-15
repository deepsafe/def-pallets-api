#![allow(clippy::too_many_arguments)]
use crate::deepsafe::runtime_types::pallet_committee::types::CryptoType;
use crate::{handle_custom_error, DeepSafeSubClient};
use sp_core::H256 as Hash;

pub async fn create_committee(
    client: &DeepSafeSubClient,
    t: u16,
    n: u16,
    crypto: CryptoType,
    fork: u8,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .committee()
        .create_committee(t, n, crypto, fork);
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn enter_epoch(
    client: &DeepSafeSubClient,
    epoch: u64,
    proofs: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().committee().enter_epoch(epoch, proofs);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn enter_epoch_call_bytes(
    client: &DeepSafeSubClient,
    epoch: u64,
    proofs: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx().committee().enter_epoch(epoch, proofs);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn expose_identity(
    client: &DeepSafeSubClient,
    identity: Vec<u8>,
    joins: Vec<(u32, Vec<(u8, u32, u32)>)>,
    device_id: Vec<u8>,
    ident_sig: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .committee()
        .expose_identity(identity, joins, device_id, ident_sig);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn expose_identity_call_bytes(
    client: &DeepSafeSubClient,
    identity: Vec<u8>,
    joins: Vec<(u32, Vec<(u8, u32, u32)>)>,
    device_id: Vec<u8>,
    ident_sig: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .committee()
        .expose_identity(identity, joins, device_id, ident_sig);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn active_committee(
    client: &DeepSafeSubClient,
    cid: u32,
    chain_id: u32,
    address: Vec<u8>,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .committee()
        .active_committee(cid, chain_id, address);
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn report_change(
    client: &DeepSafeSubClient,
    pk: Vec<u8>,
    sig: Vec<u8>,
    cid: u32,
    epoch: u32,
    fork_id: u8,
    signature: Vec<u8>,
    pubkey: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .committee()
        .report_change(pk, sig, cid, epoch, fork_id, signature, pubkey);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn report_change_call_bytes(
    client: &DeepSafeSubClient,
    pk: Vec<u8>,
    sig: Vec<u8>,
    cid: u32,
    epoch: u32,
    fork_id: u8,
    signature: Vec<u8>,
    pubkey: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .committee()
        .report_change(pk, sig, cid, epoch, fork_id, signature, pubkey);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}
