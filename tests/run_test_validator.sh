#!/bin/bash

solana-test-validator \
  --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s test_validator_programs/metaplex.metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s.so \
  --bpf-program whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc test_validator_programs/whirlpool.whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc.so \
  --bpf-program 82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj ../target/deploy/onocoy_credit.so \
  --account-dir test_validator_accounts \
  --reset 
