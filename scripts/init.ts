import { Keypair } from "npm:@solana/web3.js";
import { createMints, TOKEN_PROGRAM } from "../tests/utils.ts";
import { getProgram } from "./common.ts";

const program = getProgram();

const { mints, createMintInstructions } = await createMints(program, 2);
const [mintA, mintB] = mints;
const mintLp = Keypair.generate();

console.log("Mint A", mintA.publicKey.toBase58());
console.log("Mint B", mintB.publicKey.toBase58());
console.log("Mint LP", mintLp.publicKey.toBase58());

const signature = await program.methods
  .create()
  .accounts({
    signer: program.provider.publicKey,
    mintA: mintA.publicKey,
    mintB: mintB.publicKey,
    mintLp: mintLp.publicKey,
    tokenProgram: TOKEN_PROGRAM,
  })
  .preInstructions(createMintInstructions)
  .signers([mintA, mintB, mintLp])
  .rpc();

console.log(signature);
