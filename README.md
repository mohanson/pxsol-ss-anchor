# Pxsol simple storage with Anchor

A simple data storage contract that allows anyone to store data on the chain.

This is an Anchor version of [pxsol simple storage](https://github.com/mohanson/pxsol-ss).

Run typescript tests:

```sh
$ anchor keys sync
$ anchor test
```

if you have this error, `error Command "ts-mocha" not found.`, you can install it with:

```sh
yarn install
```

if you have this error, it indicates that your Solana local environment has already been started，so you can add the `--skip-local-validator` flag to skip starting a local validator.
```sh
Error: Your configured rpc port: 8899 is already in use
```

use this command:
```sh
$ anchor test --skip-local-validator
```


Deployed on the local test chain and interacted with via Python script:

```sh
$ solana-test-validator -l /tmp/solana-ledger
$ anchor deploy
# Program Id: GS5XPyzsXRec4sQzxJSpeDYHaTnZyYt5BtpeNXYuH1SM

$ python tests/pxsol-ss-anchor.py init

$ python tests/pxsol-ss-anchor.py update "The quick brown fox jumps over the lazy dog"
$ python tests/pxsol-ss-anchor.py load
# The quick brown fox jumps over the lazy dog

$ python tests/pxsol-ss-anchor.py update "片云天共远, 永夜月同孤."
$ python tests/pxsol-ss-anchor.py load
# 片云天共远, 永夜月同孤.
```
