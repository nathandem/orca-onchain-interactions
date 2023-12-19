import { Connection, GetVersionedTransactionConfig, Keypair, PublicKey, Transaction, TransactionInstruction, SystemProgram } from "@solana/web3.js";
import { Wallet } from "@coral-xyz/anchor";
import { buildWhirlpoolClient, IGNORE_CACHE, ORCA_WHIRLPOOL_PROGRAM_ID, PDAUtil, PriceMath, swapQuoteByInputToken, WhirlpoolContext } from "@orca-so/whirlpools-sdk";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import BN from "bn.js";
import Decimal from "decimal.js";
import wallet_keypair_bytes from "./test_wallet.json";
import { DecimalUtil, Percentage } from "@orca-so/common-sdk";

async function main() {
  const ONOCOY_CREDIT_PROGRAM_ID = "82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj";
  const BONO_USDC_64_ADDRESS = "DBJ5hywaJQKfjyt8Ekng4t6KB1gvqnYFdcJoTppCNikt";

  const ONOCOY_CREDIT_PROGRAM_PDA_SEED = "CREDIT_SIGNING_PDA";

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
  const bonoMint = whirlpoolBonoUsdc.getTokenAInfo().mint;
  const usdcMint = whirlpoolBonoUsdc.getTokenBInfo().mint;
  const decimalsA = whirlpoolBonoUsdc.getTokenAInfo().decimals;
  const decimalsB = whirlpoolBonoUsdc.getTokenBInfo().decimals;
  console.log("decimalsA (BONO):", decimalsA);
  console.log("decimalsB (USDC):", decimalsB);

  // get quote: 1 USDC for BONO
  const quote = await swapQuoteByInputToken(
    whirlpoolBonoUsdc,
    usdcMint,
    DecimalUtil.toBN(new Decimal(1), decimalsB), // 1 USDC
    Percentage.fromFraction(1, 100), // 1% slippage
    ctx.program.programId,
    ctx.fetcher,
    IGNORE_CACHE,    
  );
  console.log("quote.amount:", quote.amount.toString());
  console.log("quote.otherAmountThreshold:", quote.otherAmountThreshold.toString());
  console.log("quote.tickArray0:", quote.tickArray0.toString());
  console.log("quote.tickArray1:", quote.tickArray1.toString());
  console.log("quote.tickArray2:", quote.tickArray2.toString());

  // prepare pubkeys
  const walletUsdcAta = getAssociatedTokenAddressSync(usdcMint, wallet.publicKey);
  const [programPda, _bump] = PublicKey.findProgramAddressSync([Buffer.from(ONOCOY_CREDIT_PROGRAM_PDA_SEED)], new PublicKey(ONOCOY_CREDIT_PROGRAM_ID));
  const programPdaBonoAta = getAssociatedTokenAddressSync(bonoMint, programPda, true);
  const oracle = PDAUtil.getOracle(ctx.program.programId, whirlpoolBonoUsdc.getAddress()).publicKey;

  // invoke ReadBonoPrice instruction
  // build tx
  const tx = new Transaction()
    .add(new TransactionInstruction({
      programId: new PublicKey(ONOCOY_CREDIT_PROGRAM_ID),
      data: Buffer.from([
        ONOCOY_CREDIT_PROGRAM_IX_SWAP,
        ...quote.amount.toArray("le", 8),
        ...quote.otherAmountThreshold.toArray("le", 8),
      ]),
      keys: [
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: ctx.program.programId, isSigner: false, isWritable: false }, // whirlpool program
        { pubkey: bonoMint, isSigner: false, isWritable: false },
        { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: walletUsdcAta, isSigner: false, isWritable: true },
        { pubkey: programPda, isSigner: false, isWritable: false },
        { pubkey: programPdaBonoAta, isSigner: false, isWritable: true },
        { pubkey: whirlpoolBonoUsdc.getAddress(), isSigner: false, isWritable: true },
        { pubkey: whirlpoolBonoUsdc.getTokenVaultAInfo().address, isSigner: false, isWritable: true },
        { pubkey: whirlpoolBonoUsdc.getTokenVaultBInfo().address, isSigner: false, isWritable: true },
        { pubkey: quote.tickArray0, isSigner: false, isWritable: true },
        { pubkey: quote.tickArray1, isSigner: false, isWritable: true },
        { pubkey: quote.tickArray2, isSigner: false, isWritable: true },
        { pubkey: oracle, isSigner: false, isWritable: false },
      ],
    }));

  // sign
  const blockhash = await connection.getLatestBlockhashAndContext();
  tx.feePayer = wallet.publicKey;
  tx.recentBlockhash = blockhash.value.blockhash;
  const signedTx = await wallet.signTransaction(tx);

  // send
  const signature = await connection.sendRawTransaction(signedTx.serialize(), { skipPreflight: true });

  // wait for confirmation
  const status = await connection.confirmTransaction({ signature, ...blockhash.value }, "confirmed");
  console.log("status:", status);

  // get transaction info
  const getTransactionConfig: GetVersionedTransactionConfig = { commitment: "confirmed", maxSupportedTransactionVersion: 0 };
  const txInfo = await connection.getTransaction(signature, getTransactionConfig);
  console.log("txInfo(log):", txInfo?.meta?.logMessages);

  // check program BONO ATA balance
  const programBonoAtaBalance = await connection.getTokenAccountBalance(programPdaBonoAta, "confirmed");
  console.log("programBonoAtaBalance:", programBonoAtaBalance.value.uiAmountString);
}

main();