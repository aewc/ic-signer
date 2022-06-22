use reqwest::{header, Body, Client, Method, Request, Url};
use ic_agent::{
    agent::{
        replica_api::{ReadStateResponse, Certificate},
    },
    hash_tree::{Label, LookupResult},
};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let content_bytes = hex::decode(&args[1]).expect("not a valid hex content");
    let request_id_bytes = hex::decode(&args[2]).expect("not a valid hex request id");
    let body = Some(content_bytes.to_vec());

    let url = Url::parse("https://ic0.app/api/v2/canister/li5ot-tyaaa-aaaah-aa5ma-cai/read_state")
        .expect("error url");
    let mut http_request = Request::new(Method::POST, url);
    http_request
        .headers_mut()
        .insert(header::CONTENT_TYPE, "application/cbor".parse().unwrap());
    *http_request.body_mut() = body.map(Body::from);

    let client = Client::new();
    let response = client
        .execute(
            http_request
                .try_clone()
                .expect("Could not clone a request."),
        )
        .await
        .expect("reqwest error");

    let http_status = response.status();
    let response_headers = response.headers().clone();
    let bytes = response
        .bytes()
        .await
        .expect("response bytes error")
        .to_vec();
    println!("status: {:?}, headers: {:?}", http_status, response_headers);
    let read_state_response: ReadStateResponse = serde_cbor::from_slice(&bytes).expect("cbor decode error");
    let cert: Certificate = serde_cbor::from_slice(&read_state_response.certificate).expect("error in cert");
    println!("cert: {:?}", cert);
    let request_status_response = lookup_reply(&cert, request_id_bytes).expect("request_status_response err");
    println!("request_status_response: {:?}", request_status_response);
    let result = candid::IDLArgs::from_bytes(&request_status_response);
    println!("result: {:?}", result);
}

pub(crate) fn lookup_reply(
    certificate: &Certificate,
    request_id: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let path = [
        "request_status".into(),
        request_id.into(),
        "reply".into(),
    ];
    let reply_data = lookup_value(certificate, path).expect("lookup_value error");
    Ok(Vec::from(reply_data))
}

pub fn lookup_value<'a, P>(
    certificate: &'a Certificate<'a>,
    path: P,
) -> Result<&'a [u8], String>
where
    for<'p> &'p P: IntoIterator<Item = &'p Label>,
    P: Into<Vec<Label>>,
{
    match certificate.tree.lookup_path(&path) {
        LookupResult::Found(value) => Ok(value),
        _ => Err("LookupPathError(path.into())".to_string()),
    }
}