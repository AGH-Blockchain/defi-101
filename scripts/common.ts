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
  "2D9x4K1obxpzceZqmGYGvWs7Qdh3aevzbwiLdUTw5RsW"
);
export const mintB = new PublicKey(
  "AkTZ8gHXj8WweNdw7qtA7Y5VZNvbs85TvePPXLx5T2f6"
);
export const mintLp = new PublicKey(
  "9HQfnbbag15PwcbnWtt9k1gv16382FNpYjebYEy8grMb"
);
