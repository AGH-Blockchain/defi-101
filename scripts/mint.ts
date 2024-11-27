import { PublicKey, Transaction } from "npm:@solana/web3.js";
import { mintToAccounts } from "../tests/utils.ts";
import { getProgram, mintA, mintB } from "./common.ts";

const program = getProgram();

const accounts = [
  // // Mateusz Zając
  // "8bASXozSVHXQWUtRW8uCyx1U3YGDmzKZ37KyGkcJ5dfP",
  // // Jakub Pośpiech
  // "BToGfrd3GvY1rJWtqQxFHChoEed1YMiJBqp2sCScgcDv",
  // // Adam Frydel
  // "9E6wSeh2orQxij19z1fPZ2eMCE19a4kkiBzsfLrdFKze",
  // // Kacper Kozera
  // "7X2PxXwdFgdS7A2rrGrBzvRVA61zgTKbBnHb7aqDVJv7",
  // // Krzysiek Kaleta
  // "E2HTs9ZU95bRvxo1Ng2qxPgDzjx3H9yP6i7MUxqpbbzK",
  // // Katarzyna Bęben
  // "6F1wQvLqDUM4DFBpRLFFtuoMe5B5scfwHrw4TmgTRGhf",
  // // Kamil Szulc
  // "Fa9PAQ1CG5DhvM7UuLfe6K7Yf69ERPsyUh4xEP61K4SX",
  // // Hipacy Pociej
  // "FBgY75xFAuBUqg9YqxDD7YkpJfob8rg7b5AhHxV3qPh4",
];

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
