import {
  AnchorProvider,
  Program,
  setProvider,
  workspace,
} from "npm:@coral-xyz/anchor";
import { Defi101 } from "../target/types/defi_101.ts";
import { clusterApiUrl, Connection, PublicKey } from "npm:@solana/web3.js";

export function getProgram() {
  Deno.env.set("ANCHOR_PROVIDER_URL", clusterApiUrl("devnet"));
  Deno.env.set("ANCHOR_WALLET", "/home/mateo/.config/solana/id.json");
  const connection = new Connection(clusterApiUrl("devnet"));
  const provider = new AnchorProvider(connection, AnchorProvider.env().wallet);
  setProvider(provider);
  const program = workspace.Defi101 as Program<Defi101>;
  return program;
}

export const mintA = new PublicKey(
  "3JrxDnuphq6XUBz4CHMokmjB2jf51YRttzdRUg6K2Jqu"
);
export const mintB = new PublicKey(
  "8cBN4iFrWD3eeW4AJtjcWiVz2VS1x9HchWWskCJqPsdJ"
);
export const mintLp = new PublicKey(
  "BfgtfKJiQZeEN4deAtpXPEuGryMDMJaJMtgz6NcxKXyM"
);
