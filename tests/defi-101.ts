import { assertEquals } from "jsr:@std/assert";
import { delay } from "jsr:@std/async";
import { Defi101 } from "../target/types/defi_101.ts";
import {
  AnchorProvider,
  Program,
  setProvider,
  workspace,
} from "npm:@coral-xyz/anchor";

setProvider(AnchorProvider.env());
const program = workspace.Defi101 as Program<Defi101>;

await Deno.test("Is initialized!", async () => {
  // Add your test here.
  const tx = await program.methods.initialize().rpc();
  console.log("Your transaction signature", tx);

  // Leave time for cleanup
  await delay(1000);
});
