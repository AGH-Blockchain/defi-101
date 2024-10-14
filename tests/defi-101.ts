import { assertEquals } from "jsr:@std/assert";
import { delay } from "jsr:@std/async";
import { Defi101 } from "../target/types/defi_101.ts";
import {
  AnchorProvider,
  Program,
  setProvider,
  workspace,
} from "npm:@coral-xyz/anchor";
import { BN } from "npm:bn.js";
import { prepare, ata, TOKEN_PROGRAM } from "./utils.ts";

setProvider(AnchorProvider.env());
const program = workspace.Defi101 as Program<Defi101>;
const connection = program.provider.connection!;

Deno.test("Is initialized!", async (t) => {
  const { keypairs, mints } = await prepare(program, 5, 2);

  const [usd, abc] = mints;
  const [alice, bob, whale] = keypairs;

  await t.step("Initialize program", async () => {
    const depositAmount = new BN(1e11);
    const signature = await program.methods
      .deposit(depositAmount)
      .accounts({
        signer: whale.publicKey,
        mintA: usd,
        mintB: abc,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([whale])
      .rpc();
    console.log("Your transaction signature", signature);

    const balance_whale_after = await connection.getTokenAccountBalance(
      ata(usd, whale)
    );

    assertEquals(balance_whale_after.value.amount, "900000000000");
  });

  // Leave time for cleanup
  await delay(1000);
});
