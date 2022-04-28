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
      public_key = blob "\03\e6]\a96\5ck\94\90\f6\952\14:\e6\c3\b0\03\1cg!T\ad\01\19\fepGIq\12\fe\e4";
      chain_code = blob "ak\14\8c\c0v\907\f9|\0e|\17\e6\f9\1f\9e\f7\ce\d6\9f^v]\a6~\e3\c9\94\22\b0 ";
    }
  },
)
# public_key
03e65da9365c6b9490f69532143ae6c3b0031c672154ad0119fe7047497112fee4
# chain_code
616b148cc0769037f97c0e7c17e6f91f9ef7ced69f5e765da67ee3c99422b020
# caller
w733o-w2qko-xyblv-5w6ji-xnvxb-x7aga-lzfo5-7n74j-2qnp3-rgxgh-bae

# 1651117440 => Thu Apr 28 2022 03:44:00 GMT+0000
dfx canister call ecdsa_example sign_call '(1651117440_000_000_000)'
(
  variant {
    Ok = record {
      request_id = blob "\95\a5^\85%K\a3c\eci*\07\0e\e3v=\9bQ5\13\ed^\a2\9e\be\a9\80\0b\9c\01\d1\7f";
      content = blob "\d9\d9\f7\a3gcontent\a7lrequest_typedcallenonceM\01\02\03\04\05\06\07\08\09h\06\07\08ningress_expiry\1b\16\e9\f2\90M\a7\00\00fsenderX\1dPS\af\80\ae\bd\b7\92\8b\b6\b7\0d\fe\03\01y+\bb\f6\ff\89\d4\1a\fd\c4\d71\c2\02kcanister_idJ\00\00\00\00\00\e0\07X\01\01kmethod_namefwhoamicargFDIDL\00\00msender_pubkeyX!\03\e6]\a96\5ck\94\90\f6\952\14:\e6\c3\b0\03\1cg!T\ad\01\19\fepGIq\12\fe\e4jsender_sigX@\eb\92Vr\8a\e0\27e\c7(\03\87tF\f8\f2\e6\a2\85Z0fA\c2\ffHI#\a6\01\e1k8\dc\f4Y\e4f\ef|\10m\914\a1\dd\049\02\bd\0f\b6m;$-\1a\97\9d32=\9ev";
      sender = principal "w733o-w2qko-xyblv-5w6ji-xnvxb-x7aga-lzfo5-7n74j-2qnp3-rgxgh-bae";
    }
  },
)
# request id 
95a55e85254ba363ec692a070ee3763d9b513513ed5ea29ebea9800b9c01d17f
# content
d9d9f7a367636f6e74656e74a76c726571756573745f747970656463616c6c656e6f6e63654d010203040506070809680607086e696e67726573735f6578706972791b16e9f2904da700006673656e646572581d5053af80aebdb7928bb6b70dfe0301792bbbf6ff89d41afdc4d731c2026b63616e69737465725f69644a0000000000e0075801016b6d6574686f645f6e616d656677686f616d6963617267464449444c00006d73656e6465725f7075626b6579582103e65da9365c6b9490f69532143ae6c3b0031c672154ad0119fe7047497112fee46a73656e6465725f7369675840eb9256728ae02765c72803877446f8f2e6a2855a306641c2ff484923a601e16b38dcf459e466ef7c106d9134a1dd043902bd0fb66d3b242d1a979d33323d9e76
```

reconstruct the message.json
```json
{
    "version":1,
    "creation":"2022-04-28 03:39:00 UTC",
    "expiration":"2022-04-28 03:44:00 UTC",
    "network":"https://ic0.app",
    "call_type":"update",
    "sender":"w733o-w2qko-xyblv-5w6ji-xnvxb-x7aga-lzfo5-7n74j-2qnp3-rgxgh-bae",
    "canister_id":"li5ot-tyaaa-aaaah-aa5ma-cai",
    "method_name":"whoami",
    "arg":[68,73,68,76,0,0],
    "request_id":"95a55e85254ba363ec692a070ee3763d9b513513ed5ea29ebea9800b9c01d17f",
    "content":"d9d9f7a367636f6e74656e74a76c726571756573745f747970656463616c6c656e6f6e63654d010203040506070809680607086e696e67726573735f6578706972791b16e9f2904da700006673656e646572581d5053af80aebdb7928bb6b70dfe0301792bbbf6ff89d41afdc4d731c2026b63616e69737465725f69644a0000000000e0075801016b6d6574686f645f6e616d656677686f616d6963617267464449444c00006d73656e6465725f7075626b6579582103e65da9365c6b9490f69532143ae6c3b0031c672154ad0119fe7047497112fee46a73656e6465725f7369675840eb9256728ae02765c72803877446f8f2e6a2855a306641c2ff484923a601e16b38dcf459e466ef7c106d9134a1dd043902bd0fb66d3b242d1a979d33323d9e76"
}
```

```sh
dfx canister --network ic send message.json
Will send message:
  Creation:    2022-04-28 03:39:00 UTC
  Expiration:  2022-04-28 03:44:00 UTC
  Network:     https://ic0.app
  Call type:   update
  Sender:      w733o-w2qko-xyblv-5w6ji-xnvxb-x7aga-lzfo5-7n74j-2qnp3-rgxgh-bae
  Canister id: li5ot-tyaaa-aaaah-aa5ma-cai
  Method name: whoami
  Arg:         [68, 73, 68, 76, 0, 0]

Okay? [y/N]
y
Error: The replica returned an HTTP Error: Http Error: status 403 Forbidden, content type "application/cbor", content: Failed to authenticate request 0x95a55e85254ba363ec692a070ee3763d9b513513ed5ea29ebea9800b9c01d17f due to: Invalid signature: Invalid public key: Malformed Placeholder public key: 03e65da9365c6b9490f69532143ae6c3b0031c672154ad0119fe7047497112fee4, error: Error in DER encoding: Length field too large for object type: 102

# error happened. as info said, the public key error.

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

can't use std::time
```
[Canister rwlgt-iiaaa-aaaaa-aaaaa-cai] Panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:39:9
```

## reference

* https://github.com/dfinity/ic/commit/850e17211d2b4db3d6c5bd7a610cdc20bdd94c94#
* https://github.com/dfinity/ic/commit/976ee76630c7bd1cf351282ec5ac814fd98646c6
* https://github.com/dfinity/ic/commit/fb33870d59d787d9c51e8e09b72a091356832d4f
* https://github.com/dfinity/ic/commit/e6d71cf5fe92a63dfd73be91481720f09bda5dc6
* https://github.com/dfinity/ic/commit/8ca90d6fe823b8e8dbeec7650879362ac5731409
* https://github.com/dfinity/ic/commit/898b0332c209b25e88ccb29f2ecfb577d157af5a
