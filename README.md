# ECDSA Signing Demo

## run the demo

To run this demo you must be running dfx 0.9.3 or higher, and you will have to start replica and icx-proxy manually.

First, we start replica in one terminal:

```
~/.cache/dfinity/versions/0.9.3/ic-starter \
    --replica-path ~/.cache/dfinity/versions/0.9.3/replica \
    --subnet-features ecdsa_signatures \
    --dkg-interval-length=20 \
    --subnet-type system \
    --create-funds-whitelist '*'
```

This starts replica with all states saved in a randomly created temporary directory under `/tmp`, *everytime*.
If this is not what you want, please add `--state-dir <state_dir>` flag.
Also, by default replica binds port 8080 on localhost, and you can add flag `--http-port <port>` to change it.

Next, in another terminal, we'll start icx-proxy to listen at port 8000 and forward request to replica at 8080.

```
~/.cache/dfinity/versions/0.9.3/icx-proxy \
    --fetch-root-key \
    --address 127.0.0.1:8000 \
    --replica http://localhost:8080
```

Finally, in a third terminal, we deploy the canister:

```
dfx deploy --no-wallet ecdsa_example
```

At the moment the build is against IC commit [d004accc3904e24dddb13a11d93451523e1a8a5f](https://github.com/dfinity/ic/commit/d004accc3904e24dddb13a11d93451523e1a8a5f) because that is what dfx 0.9.3 uses.
If you want to run against newer version of IC (which has changed the ECDSA API), you will need to compile and run your own replica, and also don't forget to edit the file `src/ecdsa_example/Cargo.toml` to pin the same version.

## get candid

```sh
cargo run -p ecdsa_example > src/ecdsa_example/ecdsa_example.did
```

## test

### sign raw
```sh
# the key pair will change in different environment.
# canisters can get other canisters' pubkey, yes public.
# in the same environment, the ecdsa key pair is depended on canister id and derivation path(set by developer).
# here, we get the ecdsa_example's public key.
dfx canister call ecdsa_example pubkey '(principal "rwlgt-iiaaa-aaaaa-aaaaa-cai")' 
(
  variant {
    Ok = record {
      public_key = blob "\03\98\1e\ff\194\f05\cc\e8\df\1aq\82y?\ba+\9a^\96\cf\c4#\ca\10)\02\b6\02W\c8\fb";
      chain_code = blob "\b3T\22\83\a6\99\14\e1\fa\d5\f0\18\0f\e1\d9\15c\b0\00A85\c6\9e\b7\d8\1d<j\14\c8\bf";
    }
  },
)

# sign_raw
dfx canister call ecdsa_example sign_raw '(vec {0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;})' 
(
  variant {
    Ok = record {
      verified = true;
      signature = blob "\e7Y^\02\a62\ca\e1\92\db\e9+>J\b3Q\e3\b1\c6/\e5\eeGidK\1c\b4b\96\1d\a4\27c\9f\969\9du\a1\e5\fe\5c\d2\d0?d4\e5\9d\81,7\aeB\aer-\daE\d3\91d\84";
      publickey = blob "\03\98\1e\ff\194\f05\cc\e8\df\1aq\82y?\ba+\9a^\96\cf\c4#\ca\10)\02\b6\02W\c8\fb";
      message = blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
    }
  },
)
```

### a simple call
```sh
dfx build --network ic whoami

dfx identity get-principal
yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae

dfx canister --network ic call whoami whoami
(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")
```

### sign the simple call using ecdsa
```sh
# compressed public_key
didc decode -f blob 'blob "\03\98\1e\ff\194\f05\cc\e8\df\1aq\82y?\ba+\9a^\96\cf\c4#\ca\10)\02\b6\02W\c8\fb"'
03981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fb

# generate the der_encode_public_key and principal
cargo run --bin der 03981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fb
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/der 03981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fb`
[48, 86, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 10, 3, 66, 0, 4, 152, 30, 255, 25, 52, 240, 53, 204, 232, 223, 26, 113, 130, 121, 63, 186, 43, 154, 94, 150, 207, 196, 35, 202, 16, 41, 2, 182, 2, 87, 200, 251, 208, 26, 138, 21, 221, 251, 147, 43, 144, 216, 172, 31, 217, 124, 69, 205, 161, 89, 36, 6, 89, 203, 231, 134, 226, 90, 62, 168, 242, 100, 183, 137]
nhi75-vjqm5-4hg36-fsegq-jgzbe-jr5iu-r52qg-p4fjf-uzr5o-oqcdb-hqe

# der_encode_public_key
[48, 86, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 10, 3, 66, 0, 4, 152, 30, 255, 25, 52, 240, 53, 204, 232, 223, 26, 113, 130, 121, 63, 186, 43, 154, 94, 150, 207, 196, 35, 202, 16, 41, 2, 182, 2, 87, 200, 251, 208, 26, 138, 21, 221, 251, 147, 43, 144, 216, 172, 31, 217, 124, 69, 205, 161, 89, 36, 6, 89, 203, 231, 134, 226, 90, 62, 168, 242, 100, 183, 137]
# principal of the public_key
nhi75-vjqm5-4hg36-fsegq-jgzbe-jr5iu-r52qg-p4fjf-uzr5o-oqcdb-hqe

dfx canister call ecdsa_example sign_call '(vec{48;86;48;16;6;7;42;134;72;206;61;2;1;6;5;43;129;4;0;10;3;66;0;4;152;30;255;25;52;240;53;204;232;223;26;113;130;121;63;186;43;154;94;150;207;196;35;202;16;41;2;182;2;87;200;251;208;26;138;21;221;251;147;43;144;216;172;31;217;124;69;205;161;89;36;6;89;203;231;134;226;90;62;168;242;100;183;137})'
(
  variant {
    Ok = record {
      end = "2022-05-01 14:09:25";
      request_id = blob "\12\aa3a\ffO\85A],\f42\c9\1e\de4\d9\eeS\0b\ca\02\9b\ef2z\c9\12\dc\c1\94\d4";
      content = blob "\d9\d9\f7\a3gcontent\a7lrequest_typedcallenonceH\00\92\df\f9n\00\eb\16ningress_expiry\1b\16\eb\00n\f9\df\92\00fsenderX\1d0gxso\c5\91\0d\04\9b!\22c\d4R=\d4\0c\fe\15%\a6c\d7:\02\18O\02kcanister_idJ\00\00\00\00\00\e0\07X\01\01kmethod_namefwhoamicargFDIDL\00\00msender_pubkeyXX0V0\10\06\07*\86H\ce=\02\01\06\05+\81\04\00\0a\03B\00\04\98\1e\ff\194\f05\cc\e8\df\1aq\82y?\ba+\9a^\96\cf\c4#\ca\10)\02\b6\02W\c8\fb\d0\1a\8a\15\dd\fb\93+\90\d8\ac\1f\d9|E\cd\a1Y$\06Y\cb\e7\86\e2Z>\a8\f2d\b7\89jsender_sigX@ \fc*\c6\18\93FP2\d6\17MT\06\d5b\da\8c\b4\de\b9\b5w\b5\84x\01{\9a\fdP;+J\f6\a8\e3\1a\d3\86\cb\d9\be\da\97\e5)\9e\c2\0c\eb\c01\0a}<>\a0\85\95\d4@\d4f";
      sender = principal "nhi75-vjqm5-4hg36-fsegq-jgzbe-jr5iu-r52qg-p4fjf-uzr5o-oqcdb-hqe";
      start = "2022-05-01 14:04:25";
    }
  },
)
# request id 
12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4
# content
d9d9f7a367636f6e74656e74a76c726571756573745f747970656463616c6c656e6f6e6365480092dff96e00eb166e696e67726573735f6578706972791b16eb006ef9df92006673656e646572581d306778736fc5910d049b212263d4523dd40cfe1525a663d73a02184f026b63616e69737465725f69644a0000000000e0075801016b6d6574686f645f6e616d656677686f616d6963617267464449444c00006d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a03420004981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fbd01a8a15ddfb932b90d8ac1fd97c45cda159240659cbe786e25a3ea8f264b7896a73656e6465725f736967584020fc2ac61893465032d6174d5406d562da8cb4deb9b577b58478017b9afd503b2b4af6a8e31ad386cbd9beda97e5299ec20cebc0310a7d3c3ea08595d440d466
```

construct the message.json
```json
{
    "version":1,
    "creation":"2022-05-01 14:04:25 UTC",
    "expiration":"2022-05-01 14:09:25 UTC",
    "network":"https://ic0.app",
    "call_type":"update",
    "sender":"nhi75-vjqm5-4hg36-fsegq-jgzbe-jr5iu-r52qg-p4fjf-uzr5o-oqcdb-hqe",
    "canister_id":"li5ot-tyaaa-aaaah-aa5ma-cai",
    "method_name":"whoami",
    "arg":[68,73,68,76,0,0],
    "request_id":"12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4",
    "content":"d9d9f7a367636f6e74656e74a76c726571756573745f747970656463616c6c656e6f6e6365480092dff96e00eb166e696e67726573735f6578706972791b16eb006ef9df92006673656e646572581d306778736fc5910d049b212263d4523dd40cfe1525a663d73a02184f026b63616e69737465725f69644a0000000000e0075801016b6d6574686f645f6e616d656677686f616d6963617267464449444c00006d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a03420004981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fbd01a8a15ddfb932b90d8ac1fd97c45cda159240659cbe786e25a3ea8f264b7896a73656e6465725f736967584020fc2ac61893465032d6174d5406d562da8cb4deb9b577b58478017b9afd503b2b4af6a8e31ad386cbd9beda97e5299ec20cebc0310a7d3c3ea08595d440d466"
}
```

finally successful:
```sh
dfx canister --network ic send ./message.json 
Will send message:
  Creation:    2022-05-01 14:04:25 UTC
  Expiration:  2022-05-01 14:09:25 UTC
  Network:     https://ic0.app
  Call type:   update
  Sender:      nhi75-vjqm5-4hg36-fsegq-jgzbe-jr5iu-r52qg-p4fjf-uzr5o-oqcdb-hqe
  Canister id: li5ot-tyaaa-aaaah-aa5ma-cai
  Method name: whoami
  Arg:         [68, 73, 68, 76, 0, 0]

Okay? [y/N]
y
To check the status of this update call, append `--status` to current command.
e.g. `dfx canister send message.json --status`
Alternatively, if you have the correct identity on this machine, using `dfx canister request-status` with following arguments.
Request ID: 0x12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4
Canister ID: li5ot-tyaaa-aaaah-aa5ma-cai

# get the request status
# /request_status/<request_id>. Can only be read if /request_id/<request_id> is not present in the state tree, or if this read_state request was signed by same sender as the original request referenced by <request_id>, and the effective canister id of the original request matches the <effective_canister_id> (see Effective canister id) in this HTTP request’s path.
dfx canister call ecdsa_example sign_request_status '(vec{48;86;48;16;6;7;42;134;72;206;61;2;1;6;5;43;129;4;0;10;3;66;0;4;152;30;255;25;52;240;53;204;232;223;26;113;130;121;63;186;43;154;94;150;207;196;35;202;16;41;2;182;2;87;200;251;208;26;138;21;221;251;147;43;144;216;172;31;217;124;69;205;161;89;36;6;89;203;231;134;226;90;62;168;242;100;183;137}, "12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4")'
(
  variant {
    Ok = blob "\d9\d9\f7\a3gcontent\a4lrequest_typejread_stateningress_expiry\1b\16\eb\00\95\ffI\0bxfsenderX\1d0gxso\c5\91\0d\04\9b!\22c\d4R=\d4\0c\fe\15%\a6c\d7:\02\18O\02epaths\81\82Nrequest_statusX \12\aa3a\ffO\85A],\f42\c9\1e\de4\d9\eeS\0b\ca\02\9b\ef2z\c9\12\dc\c1\94\d4msender_pubkeyXX0V0\10\06\07*\86H\ce=\02\01\06\05+\81\04\00\0a\03B\00\04\98\1e\ff\194\f05\cc\e8\df\1aq\82y?\ba+\9a^\96\cf\c4#\ca\10)\02\b6\02W\c8\fb\d0\1a\8a\15\dd\fb\93+\90\d8\ac\1f\d9|E\cd\a1Y$\06Y\cb\e7\86\e2Z>\a8\f2d\b7\89jsender_sigX@)\f8)\e3\e9\a6\f5)c,\ff]\82\bd\a1L\07,\00\d8iUp\e6\94L\db\04\c8\f5\cd\9c\07\c4\86\b5S0oJ1m\8dp\cc\d3\e6Hv\d6m\dc\fd\f0C\03\d5V\9a\ac\e7H\f7\93"
  },
)
# read state content 
d9d9f7a367636f6e74656e74a46c726571756573745f747970656a726561645f73746174656e696e67726573735f6578706972791b16eb0095ff490b786673656e646572581d306778736fc5910d049b212263d4523dd40cfe1525a663d73a02184f0265706174687381824e726571756573745f737461747573582012aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d46d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a03420004981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fbd01a8a15ddfb932b90d8ac1fd97c45cda159240659cbe786e25a3ea8f264b7896a73656e6465725f736967584029f829e3e9a6f529632cff5d82bda14c072c00d8695570e6944cdb04c8f5cd9c07c486b553306f4a316d8d70ccd3e64876d66ddcfdf04303d5569aace748f793

cargo run --bin request d9d9f7a367636f6e74656e74a46c726571756573745f747970656a726561645f73746174656e696e67726573735f6578706972791b16eb0095ff490b786673656e646572581d306778736fc5910d049b212263d4523dd40cfe1525a663d73a02184f0265706174687381824e726571756573745f737461747573582012aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d46d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a03420004981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fbd01a8a15ddfb932b90d8ac1fd97c45cda159240659cbe786e25a3ea8f264b7896a73656e6465725f736967584029f829e3e9a6f529632cff5d82bda14c072c00d8695570e6944cdb04c8f5cd9c07c486b553306f4a316d8d70ccd3e64876d66ddcfdf04303d5569aace748f793 12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/request d9d9f7a367636f6e74656e74a46c726571756573745f747970656a726561645f73746174656e696e67726573735f6578706972791b16eb0095ff490b786673656e646572581d306778736fc5910d049b212263d4523dd40cfe1525a663d73a02184f0265706174687381824e726571756573745f737461747573582012aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d46d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a03420004981eff1934f035cce8df1a7182793fba2b9a5e96cfc423ca102902b60257c8fbd01a8a15ddfb932b90d8ac1fd97c45cda159240659cbe786e25a3ea8f264b7896a73656e6465725f736967584029f829e3e9a6f529632cff5d82bda14c072c00d8695570e6944cdb04c8f5cd9c07c486b553306f4a316d8d70ccd3e64876d66ddcfdf04303d5569aace748f793 12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4`
status: 200, headers: {"server": "nginx/1.21.3", "date": "Sun, 01 May 2022 14:08:04 GMT", "content-type": "application/cbor", "content-type": "application/cbor", "content-length": "1470", "connection": "keep-alive", "x-ic-subnet-id": "gmq5v-hbozq-uui6y-o55wc-ihop3-562wb-3qspg-nnijg-npqp5-he3cj-3ae", "x-ic-node-id": "ir3eo-e57fl-ll7wt-edk6p-pzibj-4zoqn-zui3n-rpy6p-inhyz-zfxjh-jae", "x-ic-canister-id": "0000000000e007580101", "access-control-allow-methods": "POST, GET", "access-control-allow-origin": "*", "access-control-allow-headers": "Accept, Authorization, Content-Type", "x-content-type-options": "nosniff"}
cert: Certificate { tree: HashTree { root: Fork(Fork(Pruned(ed0feff55931cee632821b471b8344bb7925702a23323cb236dc5b174fb0cd90), Fork(Label("request_status", Fork(Fork(Fork(Fork(Pruned(0be907f4dac87aed338e7782a93baf3243ee5dc2c63bb0b1ba62819fe13c2fcc), Fork(Fork(Fork(Fork(Fork(Fork(Pruned(19b763811d5c9dc3414ab66c8f5307c38f3577d3fb0c95a118033482b6849120), Fork(Pruned(70e349841c8b15af75646a07a146b617461b7c61c120064a67bd05e5bd90879d), Fork(Label(0x12aa3361ff4f85415d2cf432c91ede34d9ee530bca029bef327ac912dcc194d4, Fork(Label("reply", Leaf(38 bytes)), Label("status", Leaf("replied")))), Pruned(97450688e179c0cb19f1e98467f4dbec361bd206d0295b775aa4ea8f20bcd68c)))), Pruned(247aad7eed6c788274bd522014f22cd1eca50171cd3df44177acaa150d4f82ec)), Pruned(3a139ffcb121d5127d027cd13e653c440ba5c875a0c75c635862cbe079b6d31d)), Pruned(e8f9b2ff71af962fa69d24414dfddea72eaaf1b9b54eec48d6de469ca61b931e)), Pruned(bfe1d6873e1b63af9d9426e72d1d48c2a9cf49036c41a37faa8f55bd2207b267)), Pruned(a62aa56d628848846d268d5afa0a8e1e332ab5cc0abe5474d1910af9b16aeeef))), Pruned(c6c50add8edd24d69ba72a29dc636e0b40002c8110980ab82243577e7d1c1d00)), Pruned(5199864118717afd466bb1668143bc462c9e1da5cefbbfe324a1afe198e2fb61)), Pruned(126523f48062ac0787996ed78a9183e0126c2df1bdde0f207af245c6427478df))), Pruned(7091cc0466cb114b5623e3d2f0b86f5c757d9b9fb2ef2bbdd1bd81f5c9f8a8bb))), Fork(Pruned(d5d9a573f236839eb3bc78ffd49a626cba73e7a6b0456f6b3b01d8124b18eac4), Label("time", Leaf(0xa39da5c4bf8bc0f516)))) }, signature: [163, 79, 51, 57, 78, 244, 131, 132, 131, 227, 216, 34, 180, 72, 182, 13, 177, 249, 4, 186, 210, 203, 58, 34, 183, 254, 249, 118, 84, 55, 111, 197, 95, 53, 193, 35, 9, 158, 128, 132, 253, 134, 220, 165, 164, 191, 74, 75], delegation: Some(Delegation { subnet_id: [46, 204, 41, 68, 123, 14, 239, 108, 36, 29, 207, 223, 125, 171, 7, 112, 147, 204, 214, 161, 38, 107, 224, 254, 156, 155, 18, 118, 2], certificate: [217, 217, 247, 162, 100, 116, 114, 101, 101, 131, 1, 130, 4, 88, 32, 253, 168, 183, 219, 77, 15, 4, 120, 117, 167, 121, 204, 157, 5, 214, 112, 154, 74, 231, 116, 77, 58, 77, 66, 48, 230, 148, 232, 155, 33, 163, 90, 131, 1, 131, 2, 70, 115, 117, 98, 110, 101, 116, 131, 1, 131, 1, 131, 1, 130, 4, 88, 32, 130, 219, 144, 61, 119, 58, 159, 32, 192, 108, 85, 89, 254, 206, 214, 225, 170, 254, 179, 40, 239, 173, 26, 72, 179, 49, 99, 161, 7, 123, 134, 92, 131, 1, 131, 1, 131, 1, 131, 2, 88, 29, 46, 204, 41, 68, 123, 14, 239, 108, 36, 29, 207, 223, 125, 171, 7, 112, 147, 204, 214, 161, 38, 107, 224, 254, 156, 155, 18, 118, 2, 131, 1, 131, 2, 79, 99, 97, 110, 105, 115, 116, 101, 114, 95, 114, 97, 110, 103, 101, 115, 130, 3, 88, 27, 217, 217, 247, 129, 130, 74, 0, 0, 0, 0, 0, 224, 0, 0, 1, 1, 74, 0, 0, 0, 0, 0, 239, 255, 255, 1, 1, 131, 2, 74, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 130, 3, 88, 133, 48, 129, 130, 48, 29, 6, 13, 43, 6, 1, 4, 1, 130, 220, 124, 5, 3, 1, 2, 1, 6, 12, 43, 6, 1, 4, 1, 130, 220, 124, 5, 3, 2, 1, 3, 97, 0, 145, 84, 28, 220, 123, 101, 196, 130, 130, 134, 201, 17, 96, 45, 148, 56, 222, 86, 73, 214, 152, 182, 15, 192, 106, 236, 115, 88, 147, 149, 208, 188, 167, 23, 70, 82, 78, 210, 255, 23, 178, 200, 218, 159, 188, 137, 127, 15, 7, 164, 11, 32, 72, 113, 182, 254, 150, 212, 94, 241, 11, 81, 209, 241, 213, 48, 208, 103, 154, 93, 184, 45, 233, 105, 41, 128, 95, 161, 124, 115, 121, 148, 235, 204, 35, 18, 210, 162, 91, 217, 71, 71, 236, 248, 243, 75, 130, 4, 88, 32, 54, 151, 125, 46, 181, 120, 26, 48, 243, 146, 170, 73, 182, 138, 153, 231, 82, 227, 241, 128, 231, 214, 198, 93, 193, 21, 91, 172, 39, 32, 150, 3, 130, 4, 88, 32, 112, 255, 200, 176, 116, 236, 63, 22, 198, 60, 78, 246, 123, 255, 250, 8, 111, 129, 171, 215, 28, 146, 202, 43, 251, 88, 160, 251, 95, 111, 154, 24, 130, 4, 88, 32, 223, 167, 220, 80, 25, 162, 165, 255, 191, 96, 57, 93, 20, 21, 11, 15, 203, 98, 48, 20, 132, 219, 225, 226, 90, 233, 88, 110, 128, 106, 32, 210, 130, 4, 88, 32, 233, 223, 84, 29, 142, 11, 157, 219, 51, 225, 113, 50, 143, 142, 38, 211, 147, 195, 170, 217, 73, 106, 55, 227, 136, 223, 110, 121, 244, 188, 192, 132, 130, 4, 88, 32, 153, 111, 23, 187, 146, 107, 227, 49, 87, 69, 222, 167, 40, 32, 5, 167, 147, 181, 142, 118, 175, 235, 93, 67, 209, 162, 140, 226, 157, 45, 21, 133, 131, 2, 68, 116, 105, 109, 101, 130, 3, 73, 238, 157, 161, 159, 143, 177, 133, 245, 22, 105, 115, 105, 103, 110, 97, 116, 117, 114, 101, 88, 48, 137, 236, 194, 135, 112, 161, 76, 112, 31, 212, 140, 249, 241, 232, 40, 185, 14, 244, 32, 161, 241, 252, 123, 92, 235, 120, 180, 53, 220, 54, 184, 228, 146, 223, 120, 178, 174, 32, 226, 85, 141, 34, 52, 222, 88, 17, 16, 231] }) }
request_status_response: [68, 73, 68, 76, 0, 1, 104, 1, 29, 48, 103, 120, 115, 111, 197, 145, 13, 4, 155, 33, 34, 99, 212, 82, 61, 212, 12, 254, 21, 37, 166, 99, 215, 58, 2, 24, 79, 2]
result: Ok((principal "nhi75-vjqm5-4hg36-fsegq-jgzbe-jr5iu-r52qg-p4fjf-uzr5o-oqcdb-hqe"))
```

As all update call of a canister, there is request queue to save request, then the canister will process the Request queue in a **single thread**. 

1. ECDSA generate a simple call: Request A and then generate another simple call: Request B.
2. send Request A to ic then send Request B to ic.
3. ECDSA generate B's read_state request: Request Read B and send Request Read B to ic, get the B result correctly.
4. ECDSA generate A's read_state request: Request Read A and send Request Read A to ic, get the A result correctly.

in next test, I do switch 3 and 4: I do 4 first and then do 3. both works

```sh
# error happened before. as info said, the public key error.
dfx canister --network ic send ./message.json 
Will send message:
  Creation:    2022-04-30 04:15:00 UTC
  Expiration:  2022-04-30 04:20:00 UTC
  Network:     https://ic0.app
  Call type:   update
  Sender:      axzdn-57yuf-vaexo-kpymw-dvsog-7ztcw-qztxo-gvcgh-y5bwi-kowhh-kqe
  Canister id: li5ot-tyaaa-aaaah-aa5ma-cai
  Method name: whoami
  Arg:         [68, 73, 68, 76, 0, 0]

Okay? [y/N]
y
Error: The replica returned an HTTP Error: Http Error: status 403 Forbidden, content type "application/cbor", content: Failed to authenticate request 0xf2f3e8309e88d73716a79d067a2a757072f406d8da8c840960fc34e552c5994c due to: Invalid signature: Invalid public key: Malformed EcdsaSecp256k1 public key: 3036301006072a8648ce3d020106052b8104000a0322000309fcad5f30c9c117b006babef200d345ce70c7cb2e63b486b96805a3e2b41152, error: non-canonical encoding
```


## others 

can't use ic-agent directly, for it use the socket2 and socket2 can't support the wasm env.

```sh
error[E0583]: file not found for module `sys`
   --> /Users/flyq/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.4/src/lib.rs:124:1
    |
124 | mod sys;
    | ^^^^^^^^
    |
    = help: to create the module `sys`, create file "/Users/flyq/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.4/src/sys.rs" or "/Users/flyq/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.4/src/sys/mod.rs"

error: Socket2 doesn't support the compile target
   --> /Users/flyq/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.4/src/lib.rs:127:1
    |
127 | compile_error!("Socket2 doesn't support the compile target");
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
* https://github.com/rust-lang/socket2/issues/35#issuecomment-921188930



can't use std::time to get local time in wasm, use the `cdk::api::time()`
```sh
[Canister rwlgt-iiaaa-aaaaa-aaaaa-cai] Panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:39:9
```

can't use k256 in wasm, for the dependencies of k256 require rand_core, and the rand_core used the getrandom, and not used the ["js"] features, and only ["js"] features support wasm.

the request will be delete within some minutes, and if it be deleted, it will be error when run request:
```sh
thread 'main' panicked at 'lookup_value error: "LookupPathError(path.into())"', src/request/src/main.rs:61:54
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

# reference
Original demo source: 
* https://github.com/ninegua/ecdsa_example

ic commit for ECDSA and Bitcoin:

* https://github.com/dfinity/ic/commit/850e17211d2b4db3d6c5bd7a610cdc20bdd94c94#
* https://github.com/dfinity/ic/commit/976ee76630c7bd1cf351282ec5ac814fd98646c6
* https://github.com/dfinity/ic/commit/fb33870d59d787d9c51e8e09b72a091356832d4f
* https://github.com/dfinity/ic/commit/e6d71cf5fe92a63dfd73be91481720f09bda5dc6
* https://github.com/dfinity/ic/commit/8ca90d6fe823b8e8dbeec7650879362ac5731409
* https://github.com/dfinity/ic/commit/898b0332c209b25e88ccb29f2ecfb577d157af5a

about http-read-state:
* https://smartcontracts.org/docs/interface-spec/index.html#http-read-state
