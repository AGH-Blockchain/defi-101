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
  "9AHfAgRqNyXMG2KL6p7YsaMaQxQKL3WGUiCaKP6S1JMm"
);
export const mintB = new PublicKey(
  "FQJ6PgPwK9qjVS72aecwrNuSEHzAPhYt7ZLKKb6Pradi"
);
export const mintLp = new PublicKey(
  "FPXxcFPeSFPei4JuWiN6ELcQygCrsQ26T3f9bqUuo1Ax"
);
