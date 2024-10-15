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
import { Keypair } from "npm:@solana/web3.js";

setProvider(AnchorProvider.env());
const program = workspace.Defi101 as Program<Defi101>;
const connection = program.provider.connection!;

Deno.test("Is initialized!", async (t) => {
  const { keypairs, mints, vault } = await prepare(program, 5, 2);

  const [usd, abc] = mints;
  const [alice, bob, whale] = keypairs;
  const mintLpKeypair = Keypair.generate();
  const mintLp = mintLpKeypair.publicKey;

  await t.step("Create a pool", async () => {
    const signature = await program.methods
      .create()
      .accounts({
        signer: whale.publicKey,
        mintA: usd,
        mintB: abc,
        mintLp,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([whale, mintLpKeypair])
      .rpc();
    console.log("Create pool transaction signature", signature);
  });

  await t.step("Deposit liquidity", async () => {
    const depositAmount = new BN(1e11);
    const signature = await program.methods
      .deposit(depositAmount)
      .accounts({
        signer: whale.publicKey,
        // vault,
        mintA: usd,
        mintB: abc,
        mintLp,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([whale])
      .rpc();
    console.log("Deposit transaction signature", signature);
    const balance_whale_after = await connection.getTokenAccountBalance(
      ata(usd, whale)
    );
    const balance_whale_lp_after = await connection.getTokenAccountBalance(
      ata(mintLp, whale)
    );
    assertEquals(balance_whale_after.value.amount, "900000000000");
    assertEquals(balance_whale_lp_after.value.amount, "100000000000");
  });

  await t.step("Another deposit", async () => {
    const depositAmount = new BN(2e11);
    const signature = await program.methods
      .deposit(depositAmount)
      .accounts({
        signer: whale.publicKey,
        mintA: usd,
        mintB: abc,
        mintLp,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([whale])
      .rpc();

    console.log("Another deposit transaction signature", signature);
    const balance_whale_after = await connection.getTokenAccountBalance(
      ata(usd, whale)
    );
    const balance_whale_lp_after = await connection.getTokenAccountBalance(
      ata(mintLp, whale)
    );
    assertEquals(balance_whale_after.value.amount, "700000000000");
    assertEquals(balance_whale_lp_after.value.amount, "300000000000");
  });

  // Leave time for cleanup
  await delay(1000);
});
