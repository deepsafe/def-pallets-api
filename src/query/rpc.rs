use crate::deepsafe::runtime_types::fp_account::AccountId20 as RuntimeAccountId20;
use crate::deepsafe::runtime_types::pallet_rpc::pallet::DeviceInfo;
use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;

pub async fn device_info_rpc(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<DeviceInfo<RuntimeAccountId20, u32>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().rpc().devices(id.clone());
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn relate_deviceid_rpc(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<Vec<Vec<u8>>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage()
        .rpc()
        .watcher_deviceid_map_rpc_deviceid(id.clone());
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn eth_checkpoint(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Option<Vec<u8>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().rpc().eth_checkpoint();
    sub_client.query_storage(storage_query, at_block).await
}
