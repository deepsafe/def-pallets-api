use crate::{handle_custom_error, DeepSafeSubClient};
use sp_core::H256 as Hash;

pub async fn report_health(
    client: &DeepSafeSubClient,
    ident: Vec<u8>,
    sig: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .committee_health()
        .report_health(ident, sig);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn report_health_call_bytes(
    client: &DeepSafeSubClient,
    ident: Vec<u8>,
    sig: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .committee_health()
        .report_health(ident, sig);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn report_state_vote(
    client: &DeepSafeSubClient,
    device_id: Vec<u8>,
    sig: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .committee_health()
        .report_state_vote(device_id, sig);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn report_state_vote_call_bytes(
    client: &DeepSafeSubClient,
    device_id: Vec<u8>,
    sig: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .committee_health()
        .report_state_vote(device_id, sig);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}
