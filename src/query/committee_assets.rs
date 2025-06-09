use crate::DeepSafeSubClient;
use crate::deepsafe::runtime_types::pallet_committee_assets::pallet::AssetConsensusInfo;
use sp_core::H256 as Hash;

pub async fn all_concerned_brc20(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Option<Vec<Vec<u8>>>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee_assets()
        .all_concerned_brc20();
    sub_client.query_storage(store, at_block).await
}

pub async fn brc20_decimals(
    sub_client: &DeepSafeSubClient,
    tick: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Option<u8>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee_assets()
        .brc20_decimals(tick);
    sub_client.query_storage(store, at_block).await
}

pub async fn assets_consensus(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    at_block: Option<Hash>,
) -> Result<Option<AssetConsensusInfo>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee_assets()
        .assets_consensus(cid);
    sub_client.query_storage(store, at_block).await
}
