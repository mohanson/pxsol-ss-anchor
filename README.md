# Pxsol simple storage with Anchor

A simple data storage contract that allows anyone to store data on the chain.

This is an Anchor version of [pxsol simple storage](https://github.com/mohanson/pxsol-ss).

Run typescript tests:

```sh
$ anchor test
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
