import {
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  generateKeyPairSigner,
  pipe,
  createTransactionMessage,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  appendTransactionMessageInstructions,
  signTransactionMessageWithSigners,
  sendAndConfirmTransactionFactory,
  getSignatureFromTransaction,
  assertIsTransactionMessageWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  assertIsTransactionWithBlockhashLifetime,
} from "@solana/kit";
import {
    getFlashLoanInstruction, getFlashRepayInstruction,
    getLogBalanceInstruction
} from "./generated/instructions";
import { readFileSync } from "fs";
import { join } from "path";
import { findVaultPda } from "./generated";

const DEVNET_RPC = "https://api.devnet.solana.com";
const DEVNET_WS = "wss://api.devnet.solana.com";

async function main() {
  const rpc = createSolanaRpc(DEVNET_RPC);
  const rpcSubscriptions = createSolanaRpcSubscriptions(DEVNET_WS);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

  // Load signer keypair from /keys
  const keyPath = join(__dirname, "../../keys/ANDYPEbY29yk7y93V4DJTxnEqHe9AnSHtMKyXaNd8Dgi.json");
  const secretKey = Uint8Array.from(JSON.parse(readFileSync(keyPath, "utf-8")));
  const payer = await createKeyPairSignerFromBytes(secretKey);
  console.log("Payer:", payer.address);

  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const [vault] = await findVaultPda();
  
  const loanBigIX = getFlashLoanInstruction({ amount: 8_000_000_000, borrower: payer.address, vault });
  const loanSmallIX = getFlashLoanInstruction({ amount: 1, borrower: payer.address, vault });
  const repayIX = getFlashRepayInstruction({borrower: payer, vault})
  const logIX = getLogBalanceInstruction({account: payer.address})

  const txMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (msg) => setTransactionMessageFeePayerSigner(payer, msg),
    (msg) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, msg),
    (msg) => appendTransactionMessageInstructions([loanBigIX, loanSmallIX, repayIX], msg),
  );

  const tx = await signTransactionMessageWithSigners(txMessage);
  const signature = getSignatureFromTransaction(tx);
  console.log("Sending transaction:", signature);

  assertIsTransactionWithBlockhashLifetime(tx);

  await sendAndConfirm(tx, { commitment: "confirmed" });
  console.log("Confirmed!");
}

main().catch(console.error);
