import { Connection, GetVersionedTransactionConfig, Keypair, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";
import { Wallet } from "@coral-xyz/anchor";
import { buildWhirlpoolClient, ORCA_WHIRLPOOL_PROGRAM_ID, PriceMath, WhirlpoolContext } from "@orca-so/whirlpools-sdk";
import BN from "bn.js";
import wallet_keypair_bytes from "./test_wallet.json";

async function main() {
  const ONOCOY_CREDIT_PROGRAM_ID = "82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj";
  const BONO_USDC_64_ADDRESS = "DBJ5hywaJQKfjyt8Ekng4t6KB1gvqnYFdcJoTppCNikt";

  const ONOCOY_CREDIT_PROGRAM_IX_SWAP = 0;
  const ONOCOY_CREDIT_PROGRAM_IX_READ_BONO_PRICE = 1;

  // connection
  const connection = new Connection("http://localhost:8899");
  console.log("rpc:", await connection.rpcEndpoint);

  // wallet
  const wallet = new Wallet(Keypair.fromSecretKey(Uint8Array.from(wallet_keypair_bytes)));
  console.log("wallet:", wallet.publicKey.toBase58());

  // whirlpool client
  const ctx = WhirlpoolContext.from(connection, wallet, ORCA_WHIRLPOOL_PROGRAM_ID);
  const client = buildWhirlpoolClient(ctx);

  // fetch Whirlpool account & print price info at client side
  const whirlpoolBonoUsdc = await client.getPool(BONO_USDC_64_ADDRESS);
  const decimalsA = whirlpoolBonoUsdc.getTokenAInfo().decimals;
  const decimalsB = whirlpoolBonoUsdc.getTokenBInfo().decimals;
  console.log("decimalsA (BONO):", decimalsA);
  console.log("decimalsB (USDC):", decimalsB);

  const data = whirlpoolBonoUsdc.getData();
  const sqrtPriceX64 = data.sqrtPrice;
  const uiPrice = PriceMath.sqrtPriceX64ToPrice(sqrtPriceX64, decimalsA, decimalsB);
  console.log("sqrtPriceX64", sqrtPriceX64.toString());
  console.log("uiPrice", uiPrice.toFixed(6), "USDC/BONO");

  // invoke ReadBonoPrice instruction
  // build tx
  const bonoAmountU64 = new BN(2_000_000_000); // 2 BONO in U64
  const tx = new Transaction()
    .add(new TransactionInstruction({
      programId: new PublicKey(ONOCOY_CREDIT_PROGRAM_ID),
      data: Buffer.from([
        ONOCOY_CREDIT_PROGRAM_IX_READ_BONO_PRICE,
        ...bonoAmountU64.toArray("le", 8)
      ]),
      keys: [
        { pubkey: new PublicKey(BONO_USDC_64_ADDRESS), isSigner: false, isWritable: false },
      ],
    }));

  // sign
  const blockhash = await connection.getLatestBlockhashAndContext();
  tx.feePayer = wallet.publicKey;
  tx.recentBlockhash = blockhash.value.blockhash;
  const signedTx = await wallet.signTransaction(tx);

  // send
  const signature = await connection.sendRawTransaction(signedTx.serialize());

  // wait for confirmation
  const status = await connection.confirmTransaction({ signature, ...blockhash.value }, "confirmed");
  console.log("status:", status);

  // get transaction info
  const getTransactionConfig: GetVersionedTransactionConfig = { commitment: "confirmed", maxSupportedTransactionVersion: 0 };
  const txInfo = await connection.getTransaction(signature, getTransactionConfig);
  console.log("txInfo(log):", txInfo.meta.logMessages);
}

main();