mod types;

use candid::{candid_method, Principal};
use chrono::{prelude::DateTime, Utc};
use dfn_candid::{candid, candid_one};
use dfn_core::{
    api::{call_with_cleanup, CanisterId, PrincipalId},
    over_async,
};
use ic_agent::{
    agent::{construct_message, read_state_content, replica_api::Envelope, update_content},
    blob_from_arguments,
    hash_tree::Label,
    request_id::to_request_id,
};
use ic_ic00_types::{
    GetECDSAPublicKeyArgs, GetECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAReply,
};
use libsecp256k1::{verify, Message, PublicKey, Signature};
use serde::Serialize;
use sha2::Digest;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};
use types::{
    Bundle, CallSignature, GetBalanceError, GetBalanceRequest, GetUtxosError, GetUtxosRequest,
    GetUtxosResponse,
};

#[export_name = "canister_update pubkey"]
fn pubkey() {
    over_async(candid_one, |canister: PrincipalId| request_pubkey(canister))
}

#[candid_method(update, rename = "pubkey")]
async fn request_pubkey(canister: PrincipalId) -> Result<GetECDSAPublicKeyResponse, String> {
    let request = GetECDSAPublicKeyArgs {
        canister_id: Some(CanisterId::new(canister).expect("error canister")),
        derivation_path: vec![],
        key_id: "secp256k1".to_string(),
    };
    dfn_core::api::print(format!("Sending signature request = {:?}", request));
    let res: GetECDSAPublicKeyResponse = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(), // virtual canister, https://github.com/dfinity/interface-spec/blob/master/spec/ic.did
        "get_ecdsa_public_key",
        candid_one,
        request,
    )
    .await
    .map_err(|e| format!("Failed to call get_ecdsa_public_key {}", e.1))?;
    dfn_core::api::print(format!("Got response = {:?}", res));
    Ok(res)
}

#[export_name = "canister_update sign_raw"]
fn sign_raw() {
    over_async(candid_one, |msg: Vec<u8>| request_signature(msg))
}

#[candid_method(update, rename = "sign_raw")]
async fn request_signature(msg: Vec<u8>) -> Result<Bundle, String> {
    assert!(msg.len() == 32);
    let publickey = {
        let request = GetECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path: vec![],
            key_id: "secp256k1".to_string(),
        };
        dfn_core::api::print(format!("Sending signature request = {:?}", request));
        let res: GetECDSAPublicKeyResponse = call_with_cleanup(
            CanisterId::from_str("aaaaa-aa").unwrap(),
            "get_ecdsa_public_key",
            candid_one,
            request,
        )
        .await
        .map_err(|e| format!("Failed to call get_ecdsa_public_key {}", e.1))?;
        dfn_core::api::print(format!("Got response = {:?}", res));
        res.public_key
    };

    let signature = {
        let request = SignWithECDSAArgs {
            message_hash: msg.clone(),
            derivation_path: [].to_vec(),
            key_id: "secp256k1".to_string(),
        };
        dfn_core::api::print(format!("Sending signature request = {:?}", request));
        let res: SignWithECDSAReply = call_with_cleanup(
            CanisterId::from_str("aaaaa-aa").unwrap(),
            "sign_with_ecdsa",
            candid_one,
            request,
        )
        .await
        .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;

        //let response_signature = Signature::from_compact(&res.expect("sign_with_mock_ecdsa returned an error}")).expect("Response is not a valid signature");
        dfn_core::api::print(format!("Got response = {:?}", res));
        res.signature
    };

    let response_signature =
        Signature::parse_standard_slice(&signature).expect("Response is not a valid signature");
    let canister_public_key =
        PublicKey::parse_slice(&publickey, None).expect("Response is not a valid public key");
    // Verify the signature:
    let message = Message::parse_slice(&msg).expect("32 bytes");
    let verified = verify(&message, &response_signature, &canister_public_key);
    dfn_core::api::print(format!("ECDSA signature verification {}", verified));

    Ok(Bundle {
        message: msg,
        publickey,
        signature,
        verified,
    })
}

#[export_name = "canister_update sign_call"]
fn sign_call() {
    over_async(candid_one, |der_pubkey: Vec<u8>| {
        request_sign_call(der_pubkey)
    })
}

#[candid_method(update, rename = "sign_call")]
async fn request_sign_call(der_pubkey: Vec<u8>) -> Result<CallSignature, String> {
    let sender = Principal::self_authenticating(&der_pubkey);
    let canister_id = Principal::from_text("li5ot-tyaaa-aaaah-aa5ma-cai").expect("canister id err");
    let method_name = "whoami";
    let args = blob_from_arguments(None, None, &None).expect("args err");
    let ingress_expiry_sec = ic_cdk::api::time() / 1_000_000_000 + 5 * 60;
    let ingress_expiry_nano = ingress_expiry_sec * 1_000_000_000;
    let request = update_content(
        sender,
        &canister_id,
        &method_name,
        &args,
        ingress_expiry_nano.to_le_bytes().to_vec(), // nonce
        ingress_expiry_nano,
    )
    .expect("request err");
    dfn_core::api::print(format!("update_content result = {:?}", request));

    let request_id = to_request_id(&request).expect("request id err");
    let msg = construct_message(&request_id);
    let mut hasher = sha2::Sha256::new();
    hasher.update(&msg);

    let request_sign = SignWithECDSAArgs {
        message_hash: hasher.finalize().to_vec(),
        derivation_path: [].to_vec(),
        key_id: "secp256k1".to_string(),
    };
    let res: SignWithECDSAReply = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "sign_with_ecdsa",
        candid_one,
        request_sign,
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;
    dfn_core::api::print(format!("Got response = {:?}", res)); // res.signature Vec<u8>

    let envelope = Envelope {
        content: request,
        sender_pubkey: Some(der_pubkey),
        sender_sig: Some(res.signature),
    };

    let mut serialized_bytes = Vec::new();
    let mut serializer = serde_cbor::Serializer::new(&mut serialized_bytes);
    serializer.self_describe().expect("ser err");
    envelope.serialize(&mut serializer).expect("serialize err");
    let content = serialized_bytes;

    let end_d = UNIX_EPOCH + Duration::from_secs(ingress_expiry_sec);
    let end_datatime = DateTime::<Utc>::from(end_d);
    let end = end_datatime.format("%Y-%m-%d %H:%M:%S").to_string();
    let start_d = UNIX_EPOCH + Duration::from_secs(ingress_expiry_sec - 5 * 60);
    let start_datatime = DateTime::<Utc>::from(start_d);
    let start = start_datatime.format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(CallSignature {
        sender,
        request_id: request_id.to_vec(),
        content: content.to_vec(),
        start,
        end,
    })
}

#[export_name = "canister_update sign_request_status"]
fn sign_request_status() {
    over_async(candid, |(der_pubkey, request_id): (Vec<u8>, String)| {
        request_status(der_pubkey, request_id)
    })
}

#[candid_method(update, rename = "sign_request_status")]
async fn request_status(der_pubkey: Vec<u8>, request_id: String) -> Result<Vec<u8>, String> {
    let request_id = hex::decode(&request_id).expect("not a valid request id");
    let ingress_expiry_nano = ic_cdk::api::time() + 5 * 60 * 1_000_000_000;
    let sender = Principal::self_authenticating(&der_pubkey);
    let paths: Vec<Vec<Label>> = vec![vec!["request_status".into(), request_id.into()]];
    let request_new = read_state_content(sender, paths, ingress_expiry_nano).expect("content error");
    let request_id_new = to_request_id(&request_new).expect("request id err");
    let msg = construct_message(&request_id_new);
    let mut hasher = sha2::Sha256::new();
    hasher.update(&msg);

    let request_sign = SignWithECDSAArgs {
        message_hash: hasher.finalize().to_vec(),
        derivation_path: [].to_vec(),
        key_id: "secp256k1".to_string(),
    };
    let res: SignWithECDSAReply = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "sign_with_ecdsa",
        candid_one,
        request_sign,
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;

    let envelope = Envelope {
        content: request_new,
        sender_pubkey: Some(der_pubkey),
        sender_sig: Some(res.signature),
    };

    let mut serialized_bytes = Vec::new();
    let mut serializer = serde_cbor::Serializer::new(&mut serialized_bytes);
    serializer.self_describe().expect("ser err");
    envelope.serialize(&mut serializer).expect("serialize err");
    Ok(serialized_bytes)
}

#[export_name = "canister_update balance"]
fn balance() {
    over_async(candid_one, |_: ()| get_balance())
}

#[candid_method(update, rename = "balance")]
async fn get_balance() -> Result<u64, String> {
    let request = GetBalanceRequest {
        address: "msUVyket8s2obTn8wDyjkHjkpK92dnoD66".to_string(),
        min_confirmations: Some(0),
    };

    let res: Result<u64, GetBalanceError> = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "bitcoin_testnet_get_balance",
        candid_one,
        request,
    )
    .await
    .map_err(|e| format!("{}", e.1))?;
    Ok(res.unwrap())
}

#[export_name = "canister_update utxos"]
fn utxos() {
    over_async(candid_one, |_: ()| get_utxos())
}

#[candid_method(update, rename = "utxos")]
async fn get_utxos() -> Result<GetUtxosResponse, String> {
    let request = GetUtxosRequest {
        address: "msUVyket8s2obTn8wDyjkHjkpK92dnoD66".to_string(),
        min_confirmations: Some(0),
    };

    let res: Result<GetUtxosResponse, GetUtxosError> = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "bitcoin_testnet_get_utxos",
        candid_one,
        request,
    )
    .await
    .map_err(|e| format!("{}", e.1))?;
    Ok(res.unwrap())
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
