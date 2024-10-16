import { PublicKey } from "npm:@solana/web3.js";
import { TOKEN_PROGRAM } from "../tests/utils.ts";
import { getProgram, mintA, mintB, mintLp } from "./common.ts";
import BN from "npm:bn.js";

const program = getProgram();

const depositAmount = new BN(1e7);
const signature = await program.methods
  .deposit(depositAmount)
  .accounts({
    signer: program.provider.publicKey,
    // vault,
    mintA,
    mintB,
    mintLp,
    tokenProgram: TOKEN_PROGRAM,
  })
  .signers([])
  .rpc();

console.log(signature);
