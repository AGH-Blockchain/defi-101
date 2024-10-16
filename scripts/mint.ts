import { PublicKey, Transaction } from "npm:@solana/web3.js";
import { mintToAccounts } from "../tests/utils.ts";
import { getProgram, mintA, mintB } from "./common.ts";

const program = getProgram();

const accounts = ["f3mkEGUmbqsLBc5W87MvZwXTs346cHhNxmkcvFxzS4F"];
const amount = 1e9;

const mintInstructions = accounts.flatMap((account) => {
  return [mintA, mintB].flatMap((token) => {
    return mintToAccounts(program, [
      [new PublicKey(account), new PublicKey(token), amount],
    ]);
  });
});

const tx = new Transaction().add(...mintInstructions);

const signature = await program.provider.sendAndConfirm!(tx, []);

console.log(signature);
