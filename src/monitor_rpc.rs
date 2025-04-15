use crate::deepsafe::runtime_types::pallet_channel::types::TxSource;
use crate::no_prefix;
use crate::query::ethereum::evm_chain_id;
use crate::submit::channel::submit_transaction;
use crate::submit::channel::{clear_target_package, import_new_src_hash, sync_status};
use crate::submit::ethereum::transact;
use crate::types::{ExtrinsicData, NeedSignedExtrinsic};
use crate::watcher_rpc::SUBMIT_TRANSACTION_SELECTOR;
use crate::DeepSafeSubClient;
use precompile_utils::prelude::UnboundedBytes;
use precompile_utils::solidity::codec::Writer as EvmDataWriter;
use sp_core::{Encode, H160, U256};

pub async fn submit_extrinsic(
    sub_client: &DeepSafeSubClient,
    extrinsic: NeedSignedExtrinsic,
    need_watch_res: bool,
) -> Result<String, String> {
    match extrinsic.data {
        ExtrinsicData::PreparedCrossTransaction(tx) => {
            let tx_source = TxSource {
                chain_type: tx.chain_type as u16,
                uid: tx.uid,
                from: tx.from.clone(),
                to: tx.to,
                amount: crate::deepsafe::runtime_types::primitive_types::U256(
                    U256::from_little_endian(&tx.amount).0,
                ),
            };

            submit_transaction(
                sub_client,
                tx.channel_id,
                tx.cid,
                tx.msg,
                tx_source,
                need_watch_res,
                None,
            )
            .await
            .map(|hash| "0x".to_string() + &hex::encode(hash.0))
        }
    }
}

pub async fn submit_extrinsic_by_evm(
    sub_client: &DeepSafeSubClient,
    extrinsic: NeedSignedExtrinsic,
) -> Result<String, String> {
    match extrinsic.data {
        ExtrinsicData::PreparedCrossTransaction(tx) => {
            // build writer with 'submitTransaction' select
            let writer =
                EvmDataWriter::new_with_selector(u32::from_be_bytes(SUBMIT_TRANSACTION_SELECTOR))
                    .write(tx.channel_id)
                    .write(tx.cid)
                    .write(UnboundedBytes::from(tx.msg))
                    .write(tx.chain_type as u16)
                    .write(UnboundedBytes::from(tx.uid))
                    .write(UnboundedBytes::from(tx.from))
                    .write(UnboundedBytes::from(tx.to))
                    .write(U256::from(0u128));

            let input = writer.build();

            let chain_id = evm_chain_id(sub_client, None)
                .await
                .map_err(|e| e.to_string())?
                .ok_or("get evm chain failed".to_string())?;

            let mut inner_nonce = sub_client.inner_nonce.write().await;
            let mut call_cache = sub_client.call_cache.write().await;
            let client = sub_client.client.read().await;
            let signer = sub_client
                .signer
                .as_ref()
                .ok_or_else(|| format!("empty sk to sign and submit tx"))?;
            let account_id = signer.account_id();
            let target_nonce = {
                let chain_nonce = client
                    .tx()
                    .account_nonce(account_id)
                    .await
                    .map_err(|e| e.to_string())?;
                // clear cache for lower nonce, retain 10 nonce due to 'chain_nonce' can roll back
                let oldest_nonce = std::cmp::max(chain_nonce, 10);
                let old_nonce = call_cache
                    .keys()
                    .filter(|v| v < &&(oldest_nonce - 10))
                    .cloned()
                    .collect::<Vec<_>>();
                for key in old_nonce {
                    log::trace!(target: "subxt::call_cache", "remove key {:?}", key);
                    call_cache.remove(&key);
                }
                if chain_nonce >= *inner_nonce {
                    chain_nonce
                } else {
                    // Some errors occurred. ie. some tx with nonce not submit to chain seccessfully.
                    if *inner_nonce - chain_nonce >= sub_client.cache_size_for_call {
                        log::warn!(target: "subxt", "Some errors occurred to nonce inner {}, chain {}", *inner_nonce, chain_nonce);
                        for key in chain_nonce..*inner_nonce {
                            if let Some((inner_call, by_evm, input, tip)) = call_cache.get_mut(&key)
                            {
                                let tx = if *by_evm {
                                    let mut eip1995_tx =
                                        <ethereum::EIP1559Transaction as codec::Decode>::decode(
                                            &mut input.as_slice(),
                                        )
                                        .map_err(|e| e.to_string())?;
                                    eip1995_tx.max_priority_fee_per_gas = eip1995_tx
                                        .max_priority_fee_per_gas
                                        + sp_core::U256::from(*tip + 100u128);
                                    let evm_tx = sub_client.build_eip1559_tx_to_v2(eip1995_tx)?;
                                    let evm_call =
                                        crate::deepsafe::tx().ethereum().transact(evm_tx);
                                    client
                                        .tx()
                                        .create_unsigned(&evm_call)
                                        .map_err(|e| e.to_string())?
                                } else {
                                    client
                                        .tx()
                                        .create_signed_with_nonce(
                                            inner_call,
                                            signer,
                                            key,
                                            crate::BaseExtrinsicParamsBuilder::new()
                                                .tip(*tip + 100),
                                        )
                                        .map_err(|e| e.to_string())?
                                };
                                let tx_hash = tx.submit().await;
                                log::warn!(target: "subxt", "re-submit call with nonce: {}, tip: {:?}, res: {:?}", key, *tip + 100, tx_hash);
                                //update tip
                                *tip += 100;
                            } else {
                                log::warn!(target: "subxt", "re-submit call not find nonce: {} in cache", key);
                            }
                        }
                    }
                    *inner_nonce
                }
            };
            let tx = ethereum::EIP1559Transaction {
                chain_id,
                nonce: sp_core::U256::from(target_nonce),
                max_priority_fee_per_gas: sp_core::U256::from(1500000000u128),
                max_fee_per_gas: sp_core::U256::from(4500000000u128),
                gas_limit: sp_core::U256::from(500000u128),
                action: ethereum::TransactionAction::Call(H160::from_low_u64_be(1104)),
                value: sp_core::U256::from(0u128),
                input,
                access_list: Default::default(),
                odd_y_parity: false,
                r: Default::default(),
                s: Default::default(),
            };
            let transaction = sub_client.build_eip1559_tx_to_v2(tx.clone())?;
            return match transact(sub_client, transaction.clone()).await {
                Ok(hash) => {
                    log::debug!(target: "subxt::nonce", "inner_nonce {}, insert cache for nonce: {}", target_nonce + 1, target_nonce);
                    *inner_nonce = target_nonce + 1;
                    // update call_cache
                    call_cache.insert(
                        target_nonce,
                        (
                            Box::new(
                                crate::deepsafe::tx()
                                    .ethereum()
                                    .transact(transaction.clone()),
                            ),
                            true,
                            tx.encode(),
                            0,
                        ),
                    );
                    Ok("0x".to_string() + &hex::encode(hash.0))
                }
                Err(e) => Err(e),
            };
        }
    }
}

pub async fn import_src_hash(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    hash: String,
    src_chain_id: u32,
    uid: String,
    need_watch_res: bool,
) -> Result<String, String> {
    let hash = match hex::decode(&hash[2..]) {
        Ok(hash) => hash,
        Err(e) => return Err(e.to_string()),
    };
    let uid = match hex::decode(&uid[2..]) {
        Ok(hash) => hash,
        Err(e) => return Err(e.to_string()),
    };
    import_new_src_hash(
        sub_client,
        cid,
        hash,
        src_chain_id,
        uid,
        need_watch_res,
        None,
    )
    .await
    .map(|hash| "0x".to_string() + &hex::encode(hash.0))
}

pub async fn sync_tx_status(
    sub_client: &DeepSafeSubClient,
    request: (u32, String),
    watch_res: bool,
) -> Result<String, String> {
    let hash = match hex::decode(no_prefix(&request.1)) {
        Ok(hash) => hash,
        Err(e) => return Err(e.to_string()),
    };
    sync_status(sub_client, request.0, hash, watch_res, None)
        .await
        .map(|hash| "0x".to_string() + &hex::encode(hash.0))
}

pub async fn clear_target_btc_package(
    sub_client: &DeepSafeSubClient,
    request: (u32, String),
    watch_res: bool,
) -> Result<String, String> {
    let package_key = match hex::decode(no_prefix(&request.1)) {
        Ok(package_key) => package_key,
        Err(e) => return Err(e.to_string()),
    };
    clear_target_package(sub_client, request.0, package_key, watch_res, None)
        .await
        .map(|hash| "0x".to_string() + &hex::encode(hash.0))
}
