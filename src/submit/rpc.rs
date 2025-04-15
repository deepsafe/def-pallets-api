use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;

pub async fn register_device_rpc(
    client: &DeepSafeSubClient,
    owner: crate::deepsafe::runtime_types::fp_account::AccountId20,
    report: Vec<u8>,
    version: u16,
    signature: Vec<u8>,
    deviceid: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .rpc()
        .register_device(owner, report, version, signature, deviceid);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(|e| e.to_string())
}
