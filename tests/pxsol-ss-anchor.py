import argparse
import base64
import pxsol


parser = argparse.ArgumentParser()
parser.add_argument('--net', type=str, choices=['develop', 'mainnet', 'testnet'], default='develop')
parser.add_argument('--prikey', type=str, default='11111111111111111111111111111112')
parser.add_argument('args', nargs='+')
args = parser.parse_args()

user = pxsol.wallet.Wallet(pxsol.core.PriKey.base58_decode(args.prikey))
prog_pubkey = pxsol.core.PubKey.base58_decode('GS5XPyzsXRec4sQzxJSpeDYHaTnZyYt5BtpeNXYuH1SM')
data_pubkey = prog_pubkey.derive_pda(b'data' + user.pubkey.p)


def init():
    rq = pxsol.core.Requisition(prog_pubkey, [], bytearray())
    rq.account.append(pxsol.core.AccountMeta(user.pubkey, 3))
    rq.account.append(pxsol.core.AccountMeta(data_pubkey, 1))
    rq.account.append(pxsol.core.AccountMeta(pxsol.program.System.pubkey, 0))
    rq.data = bytearray().join([
        bytearray([220, 59, 207, 236, 108, 250, 47, 100]),
    ])
    tx = pxsol.core.Transaction.requisition_decode(user.pubkey, [rq])
    tx.message.recent_blockhash = pxsol.base58.decode(pxsol.rpc.get_latest_blockhash({})['blockhash'])
    tx.sign([user.prikey])
    txid = pxsol.rpc.send_transaction(base64.b64encode(tx.serialize()).decode(), {})
    pxsol.rpc.wait([txid])
    r = pxsol.rpc.get_transaction(txid, {})
    for e in r['meta']['logMessages']:
        print(e)


def update():
    rq = pxsol.core.Requisition(prog_pubkey, [], bytearray())
    rq.account.append(pxsol.core.AccountMeta(user.pubkey, 3))
    rq.account.append(pxsol.core.AccountMeta(data_pubkey, 1))
    rq.account.append(pxsol.core.AccountMeta(pxsol.program.System.pubkey, 0))
    rq.data = bytearray().join([
        bytearray([219, 200, 88, 176, 158, 63, 253, 127]),
        len(args.args[1].encode()).to_bytes(4, 'little'),
        args.args[1].encode(),
    ])
    tx = pxsol.core.Transaction.requisition_decode(user.pubkey, [rq])
    tx.message.recent_blockhash = pxsol.base58.decode(pxsol.rpc.get_latest_blockhash({})['blockhash'])
    tx.sign([user.prikey])
    txid = pxsol.rpc.send_transaction(base64.b64encode(tx.serialize()).decode(), {})
    pxsol.rpc.wait([txid])
    r = pxsol.rpc.get_transaction(txid, {})
    for e in r['meta']['logMessages']:
        print(e)


def load():
    info = pxsol.rpc.get_account_info(data_pubkey.base58(), {})
    print(base64.b64decode(info['data'][0])[8 + 32 + 1 + 4:].decode())


if __name__ == '__main__':
    eval(f'{args.args[0]}()')
