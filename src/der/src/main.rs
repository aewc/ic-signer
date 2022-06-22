use candid::Principal;
use k256::{
    elliptic_curve::AlgorithmParameters,
    pkcs8::{PublicKeyDocument, SubjectPublicKeyInfo},
    Secp256k1,
};
use libsecp256k1::PublicKey;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pubkey_bytes = hex::decode(&args[1]).expect("not a valid hex");
    let pubkey = PublicKey::parse_slice(&pubkey_bytes, None).expect("not a valid public key");
    let pubkey_bytes_uncompress = pubkey.serialize();
    let der_encoded_public_key: PublicKeyDocument = SubjectPublicKeyInfo {
        algorithm: Secp256k1::algorithm_identifier(),
        subject_public_key: &pubkey_bytes_uncompress,
    }
    .try_into()
    .expect("not a valid PublicKeyDocument");
    println!("{:?}", der_encoded_public_key.as_ref());
    println!(
        "{:}",
        Principal::self_authenticating(der_encoded_public_key.as_ref())
    );
}
