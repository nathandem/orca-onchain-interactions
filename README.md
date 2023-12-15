## program id (for test)
82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj

keypair: tests/test_program_keypair.json

🚨DO NOT use this address in production(mainnet). This keypair file is registered in the public repo.

## wallet address (for test)
- wallet: AyZz8kpXG3MKMkaXFS3yiEHbzKf98NwwrAaoLGnoYy27 (10,000 SOL)
- ATA for USDC: 5xvwi54sv4Zp5jDnGE1NwjxHDegHYmtZRsT2mfXYykgj (100,000 USDC)
- ATA for BONO: 9zZ8q7tB3LV9oVWFLuaRwwCmLEDku1hdyeMvFEMPigwL (100,000 BONO)

keypair: tests/test_wallet.json

🚨DO NOT use this address in production(mainnet). This keypair file is registered in the public repo.

### balance
After starting test validator...
```
$ solana config set -u l
$ solana balance AyZz8kpXG3MKMkaXFS3yiEHbzKf98NwwrAaoLGnoYy27
```

```
$ solana config set -u l
$ spl-token accounts --owner AyZz8kpXG3MKMkaXFS3yiEHbzKf98NwwrAaoLGnoYy27
```

## whirlpool
Accounts related to BONO/USDC(ts=64) have been downloaded to ``tests/test_validator_accounts``.

We can start solana-test-validator with BONO/USDC(ts=64) whirlpool.

We can download the latest state here:

https://everlastingsong.github.io/account-microscope/#/whirlpool/whirlpool/DBJ5hywaJQKfjyt8Ekng4t6KB1gvqnYFdcJoTppCNikt

- clone whirlpool
  - on: append filename prefix
  - on: with WhirlpoolsConfig
  - on: with FeeTier
  - on: with TickArray (**all**)
  - on: with VaultTokenAccount
  - on: with MintAccount
  - on: with Position

## build
```
$ cargo build-sbf
```

## test
### start test validator
```
$ cargo build-sbf
$ cd tests
$ ./run_test_validator.sh
```

### run test
after start test validator
```
$ cd tests
$ yarn
$ ts-node invoke_read_bono_price.ts 
```

output should be
```
rpc: http://localhost:8899
wallet: AyZz8kpXG3MKMkaXFS3yiEHbzKf98NwwrAaoLGnoYy27
decimalsA (BONO): 9
decimalsB (USDC): 6
sqrtPriceX64 346097641012710561
uiPrice 0.352012 USDC/BONO
status: { context: { slot: 49 }, value: { err: null } }
txInfo(log): [
  'Program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj invoke [1]',
  'Program log: process_instruction: 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj: 1 accounts, data=[1, 0, 148, 53, 119, 0, 0, 0, 0]',
  'Program log: Whirlpool account sqrt_price_x64: 346097641012710561',
  'Program log: Whirlpool account ui_price: 0.352012',
  'Program log: BONO amount in u64: 2000000000',
  'Program log: BONO amount in USDC: 0.704024',
  'Program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj consumed 28136 of 200000 compute units',
  'Program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj success'
]
```