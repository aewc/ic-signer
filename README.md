# ECDSA Signing Demo

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
dfx deploy --no-wallet
```

This should build and deploy both backend and frontend canisters.
If all is successful, you may point your browser to the asset canister's URL and see the frontend UI of this demo.

![screenshot](https://github.com/ninegua/ecdsa_example/raw/master/screenshot.png) 

At the moment the build is against IC commit [d004accc3904e24dddb13a11d93451523e1a8a5f](https://github.com/dfinity/ic/commit/d004accc3904e24dddb13a11d93451523e1a8a5f) because that is what dfx 0.9.3 uses.
If you want to run against newer version of IC (which has changed the ECDSA API), you will need to compile and run your own replica, and also don't forget to edit the file `src/ecdsa_example/Cargo.toml` to pin the same version.

## sign a request
```sh
dfx canister call ecdsa_example pubkey '(principal "rwlgt-iiaaa-aaaaa-aaaaa-cai")' 
(
  variant {
    Ok = record {
      public_key = blob "\03\09\fc\ad_0\c9\c1\17\b0\06\ba\be\f2\00\d3E\cep\c7\cb.c\b4\86\b9h\05\a3\e2\b4\11R";
      chain_code = blob "$\11\be\d2\aa\19\b8\beY\15\b55\d7\e4p\07\10\982I\b6\e5c\f7x\a7uch\04(q";
    }
  },
)
# public_key
0309fcad5f30c9c117b006babef200d345ce70c7cb2e63b486b96805a3e2b41152
# chain_code
2411bed2aa19b8be5915b535d7e4700710983249b6e563f778a7756368042871

# generate the der_encode_public_key and principal
cd src/temp
cargo run 0309fcad5f30c9c117b006babef200d345ce70c7cb2e63b486b96805a3e2b41152
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `/Users/flyq/workspace/github-aewc/ecdsa_example/target/debug/temp 0309fcad5f30c9c117b006babef200d345ce70c7cb2e63b486b96805a3e2b41152`
[48, 86, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 10, 3, 66, 0, 4, 9, 252, 173, 95, 48, 201, 193, 23, 176, 6, 186, 190, 242, 0, 211, 69, 206, 112, 199, 203, 46, 99, 180, 134, 185, 104, 5, 163, 226, 180, 17, 82, 206, 93, 154, 125, 4, 213, 237, 192, 125, 10, 176, 12, 76, 78, 102, 13, 235, 0, 167, 168, 41, 107, 204, 248, 167, 204, 193, 149, 212, 214, 148, 29]
6e7xd-4ksnm-awqip-kmiiq-saneo-ce6hq-tjqkc-qtutj-czvs5-xr2z6-zae

# der_encode_public_key
[48, 86, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 10, 3, 66, 0, 4, 9, 252, 173, 95, 48, 201, 193, 23, 176, 6, 186, 190, 242, 0, 211, 69, 206, 112, 199, 203, 46, 99, 180, 134, 185, 104, 5, 163, 226, 180, 17, 82, 206, 93, 154, 125, 4, 213, 237, 192, 125, 10, 176, 12, 76, 78, 102, 13, 235, 0, 167, 168, 41, 107, 204, 248, 167, 204, 193, 149, 212, 214, 148, 29]
# caller
6e7xd-4ksnm-awqip-kmiiq-saneo-ce6hq-tjqkc-qtutj-czvs5-xr2z6-zae

# 1651293900 => Thu Apr 30 2022 04:45:00 GMT+0000
dfx canister call ecdsa_example sign_call '(1651293900_000_000_000, vec{48;86;48;16;6;7;42;134;72;206;61;2;1;6;5;43;129;4;0;10;3;66;0;4;9;252;173;95;48;201;193;23;176;6;186;190;242;0;211;69;206;112;199;203;46;99;180;134;185;104;5;163;226;180;17;82;206;93;154;125;4;213;237;192;125;10;176;12;76;78;102;13;235;0;167;168;41;107;204;248;167;204;193;149;212;214;148;29})'
(
  variant {
    Ok = record {
      request_id = blob "\b3\e7cj!,\b5\85-\d2\ca\f9y\c3\5c@\86\81v\fb\00\f5\c5\1c\8d@\cb\86D\cd\14\9f";
      content = blob "\d9\d9\f7\a3gcontent\a7lrequest_typedcallenonceM\01\02\03\04\05\06\07\08\09h\06\07\08ningress_expiry\1b\16\ea\93\0d\99D\f8\00fsenderX\1dRk\01h!\eab\11\09\01\a4p\89\e3\c2i\82\85\09\d2i\16k.\de:\cf\b2\02kcanister_idJ\00\00\00\00\00\e0\07X\01\01kmethod_namefwhoamicargFDIDL\00\00msender_pubkeyXX0V0\10\06\07*\86H\ce=\02\01\06\05+\81\04\00\0a\03B\00\04\09\fc\ad_0\c9\c1\17\b0\06\ba\be\f2\00\d3E\cep\c7\cb.c\b4\86\b9h\05\a3\e2\b4\11R\ce]\9a}\04\d5\ed\c0}\0a\b0\0cLNf\0d\eb\00\a7\a8)k\cc\f8\a7\cc\c1\95\d4\d6\94\1djsender_sigX@\c4\e8\87\98\d0b\12\03\aeh\09\8f\db+\eb\7f\e5\27\da@\b3\01\b1#)\d66\f3Q\0e\f7\c7\01O\8f\8c|\11\ceW\80Cx\ce\078\c4\81$\a8Y\11\d3\d1\0a\89\ecE\fd\98\96\96\04\a0";
      sender = principal "6e7xd-4ksnm-awqip-kmiiq-saneo-ce6hq-tjqkc-qtutj-czvs5-xr2z6-zae";
    }
  },
)
# request id 
b3e7636a212cb5852dd2caf979c35c40868176fb00f5c51c8d40cb8644cd149f
# content
d9d9f7a367636f6e74656e74a76c726571756573745f747970656463616c6c656e6f6e63654d010203040506070809680607086e696e67726573735f6578706972791b16ea930d9944f8006673656e646572581d526b016821ea62110901a47089e3c269828509d269166b2ede3acfb2026b63616e69737465725f69644a0000000000e0075801016b6d6574686f645f6e616d656677686f616d6963617267464449444c00006d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a0342000409fcad5f30c9c117b006babef200d345ce70c7cb2e63b486b96805a3e2b41152ce5d9a7d04d5edc07d0ab00c4c4e660deb00a7a8296bccf8a7ccc195d4d6941d6a73656e6465725f7369675840c4e88798d0621203ae68098fdb2beb7fe527da40b301b12329d636f3510ef7c7014f8f8c7c11ce57804378ce0738c48124a85911d3d10a89ec45fd98969604a0
```

reconstruct the message.json
```json
{
    "version":1,
    "creation":"2022-04-30 04:40:00 UTC",
    "expiration":"2022-04-30 04:45:00 UTC",
    "network":"https://ic0.app",
    "call_type":"update",
    "sender":"6e7xd-4ksnm-awqip-kmiiq-saneo-ce6hq-tjqkc-qtutj-czvs5-xr2z6-zae",
    "canister_id":"li5ot-tyaaa-aaaah-aa5ma-cai",
    "method_name":"whoami",
    "arg":[68,73,68,76,0,0],
    "request_id":"b3e7636a212cb5852dd2caf979c35c40868176fb00f5c51c8d40cb8644cd149f",
    "content":"d9d9f7a367636f6e74656e74a76c726571756573745f747970656463616c6c656e6f6e63654d010203040506070809680607086e696e67726573735f6578706972791b16ea930d9944f8006673656e646572581d526b016821ea62110901a47089e3c269828509d269166b2ede3acfb2026b63616e69737465725f69644a0000000000e0075801016b6d6574686f645f6e616d656677686f616d6963617267464449444c00006d73656e6465725f7075626b657958583056301006072a8648ce3d020106052b8104000a0342000409fcad5f30c9c117b006babef200d345ce70c7cb2e63b486b96805a3e2b41152ce5d9a7d04d5edc07d0ab00c4c4e660deb00a7a8296bccf8a7ccc195d4d6941d6a73656e6465725f7369675840c4e88798d0621203ae68098fdb2beb7fe527da40b301b12329d636f3510ef7c7014f8f8c7c11ce57804378ce0738c48124a85911d3d10a89ec45fd98969604a0"
}
```

```sh
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

# error happened. as info said, the public key error.

```

finally successful:
```sh
dfx canister --network ic send ./message.json 

Will send message:
  Creation:    2022-04-30 04:40:00 UTC
  Expiration:  2022-04-30 04:45:00 UTC
  Network:     https://ic0.app
  Call type:   update
  Sender:      6e7xd-4ksnm-awqip-kmiiq-saneo-ce6hq-tjqkc-qtutj-czvs5-xr2z6-zae
  Canister id: li5ot-tyaaa-aaaah-aa5ma-cai
  Method name: whoami
  Arg:         [68, 73, 68, 76, 0, 0]

Okay? [y/N]
y
To check the status of this update call, append `--status` to current command.
e.g. `dfx canister send message.json --status`
Alternatively, if you have the correct identity on this machine, using `dfx canister request-status` with following arguments.
Request ID: 0xb3e7636a212cb5852dd2caf979c35c40868176fb00f5c51c8d40cb8644cd149f
Canister ID: li5ot-tyaaa-aaaah-aa5ma-cai
```


## others 

can't use ic-agent, for it use the socket2 and socket2 can't support the wasm env.

* https://github.com/rust-lang/socket2/issues/35#issuecomment-921188930

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

can't use std::time in wasm
```sh
[Canister rwlgt-iiaaa-aaaaa-aaaaa-cai] Panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:39:9
```

can't use k256 in wasm, for the dependencies of k256 require rand_core, and the rand_core used the getrandom, and not used the ["js"] features, and only ["js"] features support wasm.

## reference

* https://github.com/dfinity/ic/commit/850e17211d2b4db3d6c5bd7a610cdc20bdd94c94#
* https://github.com/dfinity/ic/commit/976ee76630c7bd1cf351282ec5ac814fd98646c6
* https://github.com/dfinity/ic/commit/fb33870d59d787d9c51e8e09b72a091356832d4f
* https://github.com/dfinity/ic/commit/e6d71cf5fe92a63dfd73be91481720f09bda5dc6
* https://github.com/dfinity/ic/commit/8ca90d6fe823b8e8dbeec7650879362ac5731409
* https://github.com/dfinity/ic/commit/898b0332c209b25e88ccb29f2ecfb577d157af5a
