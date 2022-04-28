mod types;

use candid::{CandidType, candid_method, Principal};
use dfn_candid::candid_one;
use dfn_core::{
    api::{call_with_cleanup, CanisterId, PrincipalId},
    over_async,
};
use ic_ic00_types::{
    GetECDSAPublicKeyArgs, GetECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAReply, 
};
use sign::{
    agent::{construct_message, update_content, replica_api::Envelope}, request_id::to_request_id,
    blob_from_arguments,
};
use sha2::Digest;
use types::{
    GetBalanceRequest, GetBalanceError, GetUtxosRequest, GetUtxosResponse, GetUtxosError, 
};
use libsecp256k1::{verify, Message, PublicKey, Signature};
use serde::Serialize;
use std::str::FromStr;

#[derive(CandidType, Serialize, Debug)]
pub struct Bundle {
    pub message: Vec<u8>,
    pub publickey: Vec<u8>,
    pub signature: Vec<u8>,
    pub verified: bool,
}

#[derive(CandidType, Serialize, Debug)]
pub struct CallSignature {
    pub sender: Principal,
    pub request_id: Vec<u8>,
    pub content: Vec<u8>,
}

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
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "get_ecdsa_public_key",
        candid_one,
        request,
    )
    .await
    .map_err(|e| format!("Failed to call get_ecdsa_public_key {}", e.1))?;
    dfn_core::api::print(format!("Got response = {:?}", res));
    dfn_core::api::print(format!("{}", Principal::self_authenticating(&res.public_key)));
    Ok(res)
}

#[export_name = "canister_update sign"]
fn sign() {
    over_async(candid_one, |msg: Vec<u8>| request_signature(msg))
}

#[candid_method(update, rename = "sign")]
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
    over_async(candid_one, |ingress_expiry: u64| request_call(ingress_expiry))
}

#[candid_method(update, rename = "sign_call")]
async fn request_call(ingress_expiry: u64) -> Result<CallSignature, String> {
    let publickey = {
        let request = GetECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path: vec![],
            key_id: "secp256k1".to_string(),
        };
        let res: GetECDSAPublicKeyResponse = call_with_cleanup(
            CanisterId::from_str("aaaaa-aa").unwrap(),
            "get_ecdsa_public_key",
            candid_one,
            request,
        )
        .await
        .map_err(|e| format!("Failed to call get_ecdsa_public_key {}", e.1))?;
        res.public_key
    };

    let sender = Principal::self_authenticating(&publickey);
    let canister_id = Principal::from_text("li5ot-tyaaa-aaaah-aa5ma-cai").expect("canister id err");
    let method_name = "whoami";
    let args = blob_from_arguments(None, None, &None).expect("args err");
    let request = update_content(
        sender,
        &canister_id,
        &method_name,
        &args,
        [1,2,3,4,5,6,7,8,9,104,6,7,8].to_vec(), // nonce, need rand
        ingress_expiry,
    ).expect("request err");
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
    dfn_core::api::print(format!("Got response = {:?}", res));   // res.signature Vec<u8>

    let envelope = Envelope {
        content: request,
        sender_pubkey: Some(publickey),
        sender_sig: Some(res.signature),
    };

    let mut serialized_bytes = Vec::new();
    let mut serializer = serde_cbor::Serializer::new(&mut serialized_bytes);
    serializer.self_describe().expect("ser err");
    envelope.serialize(&mut serializer).expect("serialize err");
    let content = serialized_bytes;
    
    Ok(CallSignature {
        sender,
        request_id: request_id.to_vec(),
        content: content.to_vec(),
    })
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

    let res : Result<u64, GetBalanceError> = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "bitcoin_testnet_get_balance",
        candid_one,
        request,
    ).await
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

    let res : Result<GetUtxosResponse, GetUtxosError> = call_with_cleanup(
        CanisterId::from_str("aaaaa-aa").unwrap(),
        "bitcoin_testnet_get_utxos",
        candid_one,
        request,
    ).await
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
