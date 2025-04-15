use crate::deepsafe::runtime_types::pallet_channel::types::{
    CmtType, HandleConnection, TaprootType, TxSource, XudtStatus,
};
use crate::{handle_custom_error, DeepSafeSubClient};
use sp_core::H256 as Hash;

pub async fn create_channel(
    client: &DeepSafeSubClient,
    info: Vec<u8>,
    connections: Vec<HandleConnection>,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .create_channel(info, connections);
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn bind_committees(
    client: &DeepSafeSubClient,
    channel_id: u32,
    connections: Vec<HandleConnection>,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .bind_committees(channel_id, connections);

    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn submit_transaction(
    client: &DeepSafeSubClient,
    channel_id: u32,
    cid: u32,
    msg: Vec<u8>,
    source: TxSource,
    need_watch_res: bool,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .import_new_tx(channel_id, cid, msg, source);
    if need_watch_res {
        client
            .submit_extrinsic_with_signer_and_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    } else {
        client
            .submit_extrinsic_with_signer_without_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn import_new_src_hash(
    client: &DeepSafeSubClient,
    cid: u32,
    hash: Vec<u8>,
    src_chain_id: u32,
    uid: Vec<u8>,
    need_watch_res: bool,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .import_new_source_hash(cid, hash, src_chain_id, uid);
    if need_watch_res {
        client
            .submit_extrinsic_with_signer_and_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    } else {
        client
            .submit_extrinsic_with_signer_without_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn report_result(
    client: &DeepSafeSubClient,
    pk: Vec<u8>,
    sig: Vec<u8>,
    cid: u32,
    fork_id: u8,
    hash: Hash,
    signature: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .submit_tx_sign_result(pk, sig, cid, fork_id, hash, signature);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn report_result_call_bytes(
    client: &DeepSafeSubClient,
    pk: Vec<u8>,
    sig: Vec<u8>,
    cid: u32,
    fork_id: u8,
    hash: Hash,
    signature: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .submit_tx_sign_result(pk, sig, cid, fork_id, hash, signature);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn request_sign(
    client: &DeepSafeSubClient,
    cid: u32,
    hash: Hash,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().channel().request_sign(cid, hash);
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn sync_status(
    client: &DeepSafeSubClient,
    cid: u32,
    hash: Vec<u8>,
    watch_res: bool,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().channel().sync_status(cid, hash);
    if watch_res {
        client
            .submit_extrinsic_with_signer_and_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    } else {
        client
            .submit_extrinsic_with_signer_without_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn clear_target_package(
    client: &DeepSafeSubClient,
    cid: u32,
    package_key: Vec<u8>,
    watch_res: bool,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .clear_target_package(cid, package_key);
    if watch_res {
        client
            .submit_extrinsic_with_signer_and_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    } else {
        client
            .submit_extrinsic_with_signer_without_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn create_channel_with_taproot(
    client: &DeepSafeSubClient,
    info: Vec<u8>,
    connections: Vec<(u32, u32, Vec<u8>, CmtType)>,
    taproot_types: Vec<(u32, TaprootType)>,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().channel().create_channel_with_taproot(
        info,
        connections,
        taproot_types,
    );
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn request_to_sign_refresh(
    client: &DeepSafeSubClient,
    cid: u32,
    inscription_tx: Vec<u8>,
    inscription_pos: u8,
    msg: Vec<u8>,
    watch_res: bool,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().channel().request_to_sign_refresh(
        cid,
        inscription_tx,
        inscription_pos,
        msg,
    );
    if watch_res {
        client
            .submit_extrinsic_with_signer_and_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    } else {
        client
            .submit_extrinsic_with_signer_without_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn submit_refresh_result(
    client: &DeepSafeSubClient,
    cid: u32,
    inscription_tx: Vec<u8>,
    inscription_pos: u8,
    sender_pk: Vec<u8>,
    sender_sig: Vec<u8>,
    cmt_sig: Vec<u8>,
    fork_id: u8,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx().channel().submit_refresh_result(
        cid,
        inscription_tx,
        inscription_pos,
        sender_pk,
        sender_sig,
        cmt_sig,
        fork_id,
    );
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn submit_refresh_result_call_bytes(
    client: &DeepSafeSubClient,
    cid: u32,
    inscription_tx: Vec<u8>,
    inscription_pos: u8,
    sender_pk: Vec<u8>,
    sender_sig: Vec<u8>,
    cmt_sig: Vec<u8>,
    fork_id: u8,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx().channel().submit_refresh_result(
        cid,
        inscription_tx,
        inscription_pos,
        sender_pk,
        sender_sig,
        cmt_sig,
        fork_id,
    );
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn sign_issue_xudt(
    client: &DeepSafeSubClient,
    cid: u32,
    args_of_token: Vec<u8>,
    msg: Vec<u8>,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .sign_issue_xudt(cid, args_of_token, msg);
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn submit_issue_xudt_sign_result(
    client: &DeepSafeSubClient,
    cid: u32,
    args_of_token: Vec<u8>,
    pk: Vec<u8>,
    sig: Vec<u8>,
    fork_id: u8,
    signature: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .submit_issue_xudt_sign_result(cid, args_of_token, pk, sig, fork_id, signature);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn submit_issue_xudt_sign_result_call_bytes(
    client: &DeepSafeSubClient,
    cid: u32,
    args_of_token: Vec<u8>,
    pk: Vec<u8>,
    sig: Vec<u8>,
    fork_id: u8,
    signature: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .submit_issue_xudt_sign_result(cid, args_of_token, pk, sig, fork_id, signature);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn sync_issue_xudt_result(
    client: &DeepSafeSubClient,
    cid: u32,
    args_of_token: Vec<u8>,
    status: XudtStatus,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .sync_issue_xudt_result(cid, args_of_token, status);
    client
        .submit_extrinsic_with_signer_and_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_src_hash_seq(
    client: &DeepSafeSubClient,
    cid: u32, // dst_cid
    src_chain: u32,
    src_hash: Vec<u8>,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .update_src_hash_seq(cid, src_chain, src_hash);
    client
        .submit_extrinsic_with_signer_without_watch(call, nonce)
        .await
        .map_err(|e| e.to_string())
}

pub async fn submit_uid_sign_result(
    client: &DeepSafeSubClient,
    cid: u32,
    uid: Vec<u8>,
    pk: Vec<u8>,
    sig: Vec<u8>,
    fork_id: u8,
    signature: Vec<u8>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .submit_uid_sign_result(cid, uid, pk, sig, fork_id, signature);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn submit_uid_sign_result_call_bytes(
    client: &DeepSafeSubClient,
    cid: u32,
    uid: Vec<u8>,
    pk: Vec<u8>,
    sig: Vec<u8>,
    fork_id: u8,
    signature: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .submit_uid_sign_result(cid, uid, pk, sig, fork_id, signature);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn request_to_sign_forced_withdrawal(
    client: &DeepSafeSubClient,
    tx_nonce: u128,
    msg: Vec<u8>,
    watch_res: bool,
    nonce: Option<u32>,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .sign_forced_withdrawal(tx_nonce, msg);
    if watch_res {
        client
            .submit_extrinsic_with_signer_and_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    } else {
        client
            .submit_extrinsic_with_signer_without_watch(call, nonce)
            .await
            .map_err(|e| e.to_string())
    }
}
pub async fn finish_forced_withdrawal_result(
    client: &DeepSafeSubClient,
    cid: u32,
    tx_nonce: u128,
    sender_pk: Vec<u8>,
    sender_sig: Vec<u8>,
    cmt_sig: Vec<u8>,
    fork_id: u8,
) -> Result<Hash, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .finish_forced_withdrawal(cid, tx_nonce, sender_pk, sender_sig, cmt_sig, fork_id);
    client
        .submit_extrinsic_without_signer(call)
        .await
        .map_err(handle_custom_error)
}

pub async fn finish_forced_withdrawal_result_call_bytes(
    client: &DeepSafeSubClient,
    cid: u32,
    tx_nonce: u128,
    sender_pk: Vec<u8>,
    sender_sig: Vec<u8>,
    cmt_sig: Vec<u8>,
    fork_id: u8,
) -> Result<Vec<u8>, String> {
    let call = crate::deepsafe::tx()
        .channel()
        .finish_forced_withdrawal(cid, tx_nonce, sender_pk, sender_sig, cmt_sig, fork_id);
    client
        .unsigned_tx_encode_to_bytes(call)
        .await
        .map_err(handle_custom_error)
}
