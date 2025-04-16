use crate::deepsafe::runtime_types::{
    pallet_facility::pallet::DIdentity,
    pallet_mining::types::{DeviceInfo, MonitorState, RegisterData},
    primitive_types::U256,
};
use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;
use subxt::ext::subxt_core::utils::AccountId20;

pub async fn challenges(
    sub_client: &DeepSafeSubClient,
    session: u32,
    at_block: Option<Hash>,
) -> Result<Option<U256>, subxt::Error> {
    let store = crate::deepsafe::storage().mining().challenges(session);
    sub_client.query_storage(store, at_block).await
}

pub async fn working_devices(
    sub_client: &DeepSafeSubClient,
    session: Option<u32>,
    at_block: Option<Hash>,
) -> Result<Option<(Vec<(DIdentity, bool)>, u32)>, subxt::Error> {
    let session = match session {
        Some(session) => session,
        None => {
            let client = sub_client.client.read().await.blocks();
            let current_block = match at_block {
                Some(hash) => client.at(hash).await,
                None => client.at_latest().await,
            };
            let current_number = current_block.map(|b| b.number())?;
            let constant_query = crate::deepsafe::constants().mining().era_block_number();
            sub_client
                .query_constant(constant_query)
                .await
                .map(|era_block_number| current_number / era_block_number)?
        }
    };
    let store = crate::deepsafe::storage().mining().working_devices(session);
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|res| res.and_then(|data| Some((data, session))))
}

pub async fn device_info(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<DeviceInfo<AccountId20, u32, u128>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().mining().devices(id.clone());
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn device_info_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<DeviceInfo<AccountId20, u32, u128>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().mining().devices_iter();
    sub_client
        .query_storage_value_iter(storage_query, at_block)
        .await
        .map(|res| res.into_iter().map(|v| v.1).collect())
}

pub async fn device_identity_map(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<Vec<u8>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().mining().device_identity_map(id);
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn device_identity_map_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<(Vec<u8>, Vec<u8>)>, subxt::Error> {
    let storage_query = crate::deepsafe::storage()
        .mining()
        .device_identity_map_iter();
    sub_client
        .query_storage_value_iter(storage_query, at_block)
        .await
        .map(|res| {
            res.into_iter()
                .map(|(k, v)| (k[49..].to_vec(), v))
                .collect()
        })
}

pub async fn device_monitor_state(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<MonitorState>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().mining().device_monitor_state(id);
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn device_votes_for_current_epoch(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<Vec<(AccountId20, u128)>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage()
        .mining()
        .device_votes_for_current_epoch(id.clone());
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn device_votes_for_next_epoch(
    sub_client: &DeepSafeSubClient,
    id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<Vec<(AccountId20, u128)>>, subxt::Error> {
    let storage_query = crate::deepsafe::storage()
        .mining()
        .device_votes_for_next_epoch(id.clone());
    sub_client.query_storage(storage_query, at_block).await
}

pub async fn device_data(
    sub_client: &DeepSafeSubClient,
    did: DIdentity,
    at_block: Option<Hash>,
) -> Result<Option<Vec<u8>>, subxt::Error> {
    let store = crate::deepsafe::storage().mining().device_data(did.clone());
    sub_client.query_storage(store, at_block).await
}

pub async fn devices_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<DeviceInfo<AccountId20, u32, u128>>, subxt::Error> {
    let store = crate::deepsafe::storage().mining().devices_iter();
    sub_client
        .query_storage_value_iter(store, at_block)
        .await
        .map(|res| res.into_iter().map(|(_, v)| v).collect())
}

pub async fn device_register_data(
    sub_client: &DeepSafeSubClient,
    device_id: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<RegisterData>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .mining()
        .device_register_data(device_id);
    sub_client.query_storage(store, at_block).await
}

pub async fn device_register_data_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<(Vec<u8>, RegisterData)>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .mining()
        .device_register_data_iter();
    sub_client
        .query_storage_value_iter(store, at_block)
        .await
        .map(|res| {
            res.into_iter()
                .map(|(k, v)| (k[49..].to_vec(), v))
                .collect()
        })
}
