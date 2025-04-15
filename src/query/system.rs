use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;

pub async fn block_hash(
    sub_client: &DeepSafeSubClient,
    height: u32,
    at_block: Option<Hash>,
) -> Result<Option<Hash>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().system().block_hash(height);
    sub_client.query_storage(storage_query, at_block).await
}
