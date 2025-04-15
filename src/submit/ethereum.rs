use crate::deepsafe::runtime_types::ethereum::transaction::TransactionV2 as Transaction;
use crate::{handle_custom_error, DeepSafeSubClient};
use sp_core::H256 as Hash;

pub async fn transact(
    client: &DeepSafeSubClient,
    transaction: Transaction,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().ethereum().transact(transaction);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn transact_unsigned(
    client: &DeepSafeSubClient,
    transaction: Transaction,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .ethereum()
        .transact_unsigned(transaction);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn transact_unsigned_call_bytes(
    client: &DeepSafeSubClient,
    transaction: Transaction,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .ethereum()
        .transact_unsigned(transaction);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}
