use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;

pub async fn hash_to_version(
    sub_client: &DeepSafeSubClient,
    version: u16,
    at_block: Option<Hash>,
) -> Result<Option<Vec<u8>>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .facility()
        .hash_to_version(&version);
    sub_client.query_storage(store, at_block).await
}

pub async fn version_list(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<u16>, subxt::Error> {
    let store = crate::deepsafe::storage().facility().version_list();
    sub_client.query_storage_or_default(store, at_block).await
}
