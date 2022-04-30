use k256::{
    pkcs8::{PublicKeyDocument, SubjectPublicKeyInfo},
    elliptic_curve::AlgorithmParameters,
    Secp256k1,
};
use candid::Principal;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let public_key_bytes = hex::decode(&args[1]).expect("not a valid hex");
    let pubkey = libsecp256k1::PublicKey::parse_slice(&public_key_bytes, None).expect("not a valid public key");
    let pubkey_bytes_uncompress = pubkey.serialize();
    let der_encoded_public_key: PublicKeyDocument = SubjectPublicKeyInfo {
        algorithm: Secp256k1::algorithm_identifier(),
        subject_public_key: &pubkey_bytes_uncompress,
    }.try_into().unwrap();
    println!("{:?}", der_encoded_public_key.as_ref());
    println!("{:}", Principal::self_authenticating(
        der_encoded_public_key.as_ref(),
    ));
}
