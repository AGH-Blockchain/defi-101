import { TOKEN_PROGRAM } from "../tests/utils.ts";
import { getProgram } from "./common.ts";
import BN from "npm:bn.js";

const program = getProgram();

const swapAmount = new BN(-1e5);
const signature = await program.methods
  .swap(swapAmount)
  .accounts({
    signer: program.provider.publicKey,
    tokenProgram: TOKEN_PROGRAM,
  })
  .signers([])
  .rpc();

console.log(signature);
