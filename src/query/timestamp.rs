use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;

pub async fn now(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Option<u64>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().timestamp().now();
    sub_client.query_storage(storage_query, at_block).await
}
