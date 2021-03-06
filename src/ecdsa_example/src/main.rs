mod types;

use candid::{CandidType, candid_method};
use dfn_candid::candid_one;
use dfn_core::{
    api::{call_with_cleanup, CanisterId},
    over_async,
};
use ic_ic00_types::{
    ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAReply, EcdsaKeyId,
    EcdsaCurve,
};
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

#[export_name = "canister_update sign"]
fn sign() {
    over_async(candid_one, |msg: Vec<u8>| request_signature(msg))
}

#[candid_method(update, rename = "sign")]
async fn request_signature(msg: Vec<u8>) -> Result<Bundle, String> {
    assert!(msg.len() == 32);
    let publickey = {
        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path: vec![],
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: "secp256k1".to_string(),
            },
        };
        dfn_core::api::print(format!("Sending signature request = {:?}", request));
        let res: ECDSAPublicKeyResponse = call_with_cleanup(
            CanisterId::from_str("aaaaa-aa").unwrap(),
            "ecdsa_public_key",
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
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: "secp256k1".to_string(),
            },
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