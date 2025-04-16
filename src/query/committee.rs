use crate::deepsafe::runtime_types::pallet_committee::types::{Committee, GlobalConfig};
use crate::DeepSafeSubClient;
use sp_core::H256 as Hash;
use subxt::ext::subxt_core::utils::AccountId20;

pub async fn global_epoch(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<u64, subxt::Error> {
    let store = crate::deepsafe::storage().committee().global_epoch();
    sub_client.query_storage_or_default(store, at_block).await
}

pub async fn epoch_config(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<GlobalConfig<u32>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().epoch_config();
    sub_client.query_storage_or_default(store, at_block).await
}

pub async fn next_epoch_config(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Option<GlobalConfig<u32>>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().next_epoch_config();
    sub_client.query_storage(store, at_block).await
}

pub async fn committees(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    at_block: Option<Hash>,
) -> Result<Option<Committee<AccountId20, u32>>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().committees(cid);
    sub_client.query_storage(store, at_block).await
}

pub async fn committees_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<Committee<AccountId20, u32>>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().committees_iter();
    sub_client
        .query_storage_value_iter(store, at_block)
        .await
        .map(|res| res.into_iter().map(|v| v.1).collect())
}

pub async fn snapshot(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<Vec<u8>>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().snapshot();
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn candidate_pool(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<Vec<u8>>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().candidate_pool();
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn committee_members(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    epoch: u32,
    fork_id: u8,
    at_block: Option<Hash>,
) -> Result<Option<Vec<Vec<u8>>>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .committee_members(cid, (epoch, fork_id));
    sub_client.query_storage(store, at_block).await
}

pub async fn member_links(
    sub_client: &DeepSafeSubClient,
    member: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<u32, subxt::Error> {
    let store = crate::deepsafe::storage().committee().member_links(member);
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn member_links_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<(Vec<u8>, u32)>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().member_links_iter();
    sub_client
        .query_storage_value_iter(store, at_block)
        .await
        .map(|res| {
            res.into_iter()
                .map(|(k, v)| (k[49..].to_vec(), v))
                .collect()
        })
}

pub async fn candidate_links(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    fork: u8,
    at_block: Option<Hash>,
) -> Result<Vec<u16>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .candidate_links(cid, fork);
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn epoch_change_failures(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    fork: u8,
    at_block: Option<Hash>,
) -> Result<u8, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .epoch_changes_failures(cid, fork);
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn epoch_change_failures_iter(
    sub_client: &DeepSafeSubClient,
    at_block: Option<Hash>,
) -> Result<Vec<(u32, u8, u8)>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .epoch_changes_failures_iter();
    sub_client
        .query_storage_value_iter(store, at_block)
        .await
        .map(|res| {
            res.into_iter()
                .map(|(k, v)| {
                    let mut cid_bytes = [0u8; 4];
                    cid_bytes.copy_from_slice(&k[48..52]);
                    (u32::from_le_bytes(cid_bytes), k[68], v)
                })
                .collect()
        })
}

pub async fn committee_randomness(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    at_block: Option<Hash>,
) -> Result<Option<u64>, subxt::Error> {
    let store = crate::deepsafe::storage().committee().c_randomness(cid);
    sub_client.query_storage(store, at_block).await
}

pub async fn unpaid_sign_fee(
    sub_client: &DeepSafeSubClient,
    pk: Vec<u8>,
    epoch: u32,
    at_block: Option<Hash>,
) -> Result<Option<u128>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .unpaid_sign_fee(pk, epoch);
    sub_client.query_storage(store, at_block).await
}

pub async fn identity_rewards(
    sub_client: &DeepSafeSubClient,
    ident: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<u128, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .identity_rewards(ident);
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn exposed_identity(
    sub_client: &DeepSafeSubClient,
    ident: Vec<u8>,
    at_block: Option<Hash>,
) -> Result<Vec<u8>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .exposed_identity(ident);
    sub_client
        .query_storage(store, at_block)
        .await
        .map(|r| r.unwrap_or_default())
}

pub async fn rewards_for_fork(
    sub_client: &DeepSafeSubClient,
    cid: u32,
    epoch: u32,
    fork_id: u8,
    at_block: Option<Hash>,
) -> Result<Option<(u128, Vec<Vec<u8>>)>, subxt::Error> {
    let store = crate::deepsafe::storage()
        .committee()
        .rewards_for_fork(cid, epoch, fork_id);
    sub_client.query_storage(store, at_block).await
}
