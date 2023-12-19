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

```
$ ts-node invoke_swap.ts
```

output should be
```
rpc: http://localhost:8899
wallet: AyZz8kpXG3MKMkaXFS3yiEHbzKf98NwwrAaoLGnoYy27
decimalsA (BONO): 9
decimalsB (USDC): 6
quote.amount: 1000000
quote.otherAmountThreshold: 2798074270
quote.tickArray0: AnxYYj2gaKbGYqtHTJuLZVff6yZBnG4qVzywHshAaHYp
quote.tickArray1: 7oqxtmUZuTGoCNEzfF7pRSYYrEQe294nMnT5Fas1Amy3
quote.tickArray2: FmtmwHixM8g1hAvo49N2xW6AEBaoGsMcVcyB9NkQvJDj
status: { context: { slot: 13 }, value: { err: null } }
txInfo(log): [
  'Program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj invoke [1]',
  'Program log: process_instruction: 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj: 16 accounts, data=[0, 64, 66, 15, 0, 0, 0, 0, 0, 158, 57, 199, 166, 0, 0, 0, 0]',
  'Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [2]',
  'Program log: CreateIdempotent',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]',
  'Program log: Instruction: GetAccountDataSize',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1595 of 152603 compute units',
  'Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success',
  'Program 11111111111111111111111111111111 invoke [3]',
  'Program 11111111111111111111111111111111 success',
  'Program log: Initialize the associated token account',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]',
  'Program log: Instruction: InitializeImmutableOwner',
  'Program log: Please upgrade to SPL Token 2022 for immutable owner support',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 145990 compute units',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]',
  'Program log: Instruction: InitializeAccount3',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4214 of 142106 compute units',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success',
  'Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 27998 of 165586 compute units',
  'Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]',
  'Program log: Instruction: Approve',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2904 of 133955 compute units',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success',
  'Program whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc invoke [2]',
  'Program log: Instruction: Swap',
  'Program log: fee_growth: 2227780281713',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]',
  'Program log: Instruction: Transfer',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4735 of 87018 compute units',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]',
  'Program log: Instruction: Transfer',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 79363 compute units',
  'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success',
  'Program whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc consumed 50974 of 122151 compute units',
  'Program whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc success',
  'Program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj consumed 129618 of 200000 compute units',
  'Program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj success'
]
programBonoAtaBalance: 2.826055013
```