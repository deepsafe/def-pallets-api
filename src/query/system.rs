use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;

pub async fn block_hash(
    sub_client: &DeepSafeSubClient,
    height: u32,
    at_block: Option<Hash>,
) -> Result<Option<Hash>, String> {
    let storage_query = crate::deepsafe::storage().system().block_hash(height);
    return match sub_client.query_storage(storage_query, at_block).await {
        Ok(res) => {
            if res.is_none() {
                log::warn!(target: "pallets_api", "query none block hash for height: {:?}", height);
            }
            Ok(res)
        },
        Err(e) => {
            log::error!(target: "pallets_api", "query block hash failed: height: {:?} for {:?}", height, e);
            Err(e.to_string())
        }
    }
}

pub async fn latest_block_number(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Option<u32>, subxt::Error> {
    let storage_query = crate::deepsafe::storage().system().number();
    return match sub_client.query_storage(storage_query, at_block).await {
        Ok(res) => {
            if res.is_none() {
                log::warn!(target: "pallets_api", "query none latest block number");
            }
            Ok(res)
        },
        Err(e) => {
            log::error!(target: "pallets_api", "query latest block number failed for {:?}", e);
            Err(e)
        }
    }
}

pub async fn latest_block_hash(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Hash, subxt::Error> {
    let number = latest_block_number(sub_client, at_block).await?.ok_or(subxt::Error::Other("latest block number return none".to_string()))?;
    block_hash(sub_client, number, at_block).await?.ok_or(subxt::Error::Other("get block hash by latest number return none".to_string()))
}
