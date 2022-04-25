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
dfx deploy
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
      public_key = blob "\0296$aG\84g\a3\99\c5\cc2%\07p\a6\da\85\0d\8e\a0\e1\8bl\a4\cb\ffV\d2\ab\c8\01";
      chain_code = blob "\fa\02P\88\d8\8b$\ca\0c5\df\fbR\16\b7\a0nm@4\07=\957\9b)\b8\ed4\1c\d5\97";
    }
  },
)
0239362461478467a399c5cc32250770a6da850d8ea0e18b6ca4cbff56d2abc801

dfx canister call ecdsa_example pubkey '(principal "r7inp-6aaaa-aaaaa-aaabq-cai")'
(
  variant {
    Ok = record {
      public_key = blob "\03\09\f3q\e4\ca6\ef\cb\be\d2\c3!&\d2,#w\fd3\02\19\e8\8f\d9\dd\a2\cc\a5P\a4\a8\e1";
      chain_code = blob "8TD\af\14\98\a8X\91Dk\feV\b3^\d6\a5S:\b5N\db\d4\f6\1f8\9a\c1Q2o\ae";
    }
  },
)

dfx canister call ecdsa_example1 pubkey '(principal "rwlgt-iiaaa-aaaaa-aaaaa-cai")'
(
  variant {
    Ok = record {
      public_key = blob "\0296$aG\84g\a3\99\c5\cc2%\07p\a6\da\85\0d\8e\a0\e1\8bl\a4\cb\ffV\d2\ab\c8\01";
      chain_code = blob "\fa\02P\88\d8\8b$\ca\0c5\df\fbR\16\b7\a0nm@4\07=\957\9b)\b8\ed4\1c\d5\97";
    }
  },
)

dfx canister call ecdsa_example1 pubkey '(principal "r7inp-6aaaa-aaaaa-aaabq-cai")'
(
  variant {
    Ok = record {
      public_key = blob "\03\09\f3q\e4\ca6\ef\cb\be\d2\c3!&\d2,#w\fd3\02\19\e8\8f\d9\dd\a2\cc\a5P\a4\a8\e1";
      chain_code = blob "8TD\af\14\98\a8X\91Dk\feV\b3^\d6\a5S:\b5N\db\d4\f6\1f8\9a\c1Q2o\ae";
    }
  },
)

dfx canister call ecdsa_example sign '(vec{0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;})'
(
  variant {
    Ok = record {
      verified = true;
      signature = blob "\fe\ea\d2F\ef\c1\04\5cu\d2\0f\de4\fcCN\e3L\cb\f3n\c9\9f6X\d6\08\ff\1aG2\9cG\9f\99\d2\e9;E\03\aaBE\a7\b3\cb\ec\ec-{\0d4\9e\8d\d0Vs\f1X\a6\e8\0cQ\ef";
      publickey = blob "\0296$aG\84g\a3\99\c5\cc2%\07p\a6\da\85\0d\8e\a0\e1\8bl\a4\cb\ffV\d2\ab\c8\01";
      message = blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
    }
  },
)

dfx canister --network ic call li5ot-tyaaa-aaaah-aa5ma-cai whoami
(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")

dfx canister --nertwork ic sign li5ot-tyaaa-aaaah-aa5ma-cai whoami
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

## reference

* https://github.com/dfinity/ic/commit/850e17211d2b4db3d6c5bd7a610cdc20bdd94c94#
* https://github.com/dfinity/ic/commit/976ee76630c7bd1cf351282ec5ac814fd98646c6
* https://github.com/dfinity/ic/commit/fb33870d59d787d9c51e8e09b72a091356832d4f
* https://github.com/dfinity/ic/commit/e6d71cf5fe92a63dfd73be91481720f09bda5dc6
* https://github.com/dfinity/ic/commit/8ca90d6fe823b8e8dbeec7650879362ac5731409
* https://github.com/dfinity/ic/commit/898b0332c209b25e88ccb29f2ecfb577d157af5a
