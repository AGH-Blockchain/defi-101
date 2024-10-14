import type { Program } from "npm:@coral-xyz/anchor";
import {
  PublicKey,
  TransactionInstruction,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction,
  Keypair,
} from "npm:@solana/web3.js";
import { makeKeypairs } from "npm:@solana-developers/helpers";
import type { Defi101 } from "../target/types/defi_101.ts";
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from "npm:@solana/spl-token";

export const TOKEN_PROGRAM = TOKEN_2022_PROGRAM_ID;

export function createKeypairs(program: Program<Defi101>, count: number) {
  const keypairs = makeKeypairs(count);
  const transferInstructions = keypairs.map((keypair) =>
    SystemProgram.transfer({
      fromPubkey: program.provider.publicKey!,
      toPubkey: keypair.publicKey,
      lamports: 10 * LAMPORTS_PER_SOL,
    })
  );

  return { keypairs, transferInstructions };
}

export async function createMints(program: Program<Defi101>, count: number) {
  const provider = program.provider!;
  const wallet = provider.publicKey!;
  const mints = makeKeypairs(count);

  const minimumLamports = await getMinimumBalanceForRentExemptMint(
    program.provider.connection
  );
  const createMintInstructions: Array<TransactionInstruction> = mints.map(
    (mint) =>
      SystemProgram.createAccount({
        fromPubkey: wallet,
        newAccountPubkey: mint.publicKey,
        lamports: minimumLamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM,
      })
  );

  const initializeMint2Instructions = mints.map((m) =>
    createInitializeMint2Instruction(
      m.publicKey,
      6,
      program.provider.publicKey!,
      null,
      TOKEN_PROGRAM
    )
  );

  return {
    mints: mints,
    createMintInstructions: [
      ...createMintInstructions,
      ...initializeMint2Instructions,
    ],
  };
}

export function mintToAccounts(
  program: Program<Defi101>,
  toMint: Array<[PublicKey, PublicKey, number]>
) {
  const mintAuthority = program.provider.publicKey!;

  const mintTokensInstructions: Array<TransactionInstruction> = toMint.flatMap(
    ([authority, mint, amount]) => {
      const ata = getAssociatedTokenAddressSync(
        mint,
        authority,
        false,
        TOKEN_PROGRAM
      );
      return [
        createAssociatedTokenAccountInstruction(
          mintAuthority,
          ata,
          authority,
          mint,
          TOKEN_PROGRAM
        ),
        createMintToInstruction(
          mint,
          ata,
          mintAuthority,
          amount,
          [],
          TOKEN_PROGRAM
        ),
      ];
    }
  );

  return mintTokensInstructions;
}

export async function prepare(
  program: Program<Defi101>,
  keysCount: number,
  mintsCount: number
) {
  const { keypairs, transferInstructions } = await createKeypairs(
    program,
    keysCount
  );
  const { mints, createMintInstructions } = await createMints(
    program,
    mintsCount
  );
  const mintTokensInstructions = await mintToAccounts(program, [
    [keypairs[0].publicKey, mints[0].publicKey, 1e9],
    [keypairs[1].publicKey, mints[1].publicKey, 1e9],
    [keypairs[2].publicKey, mints[0].publicKey, 1e12],
    [keypairs[2].publicKey, mints[1].publicKey, 1e12],
  ]);

  const tx = new Transaction();
  tx.add(
    ...[
      ...transferInstructions,
      ...createMintInstructions,
      ...mintTokensInstructions,
    ]
  );

  const _transactionSignature = await program.provider.sendAndConfirm!(tx, [
    ...mints,
  ]);

  // await confirmTransaction(program.provider.connection, transactionSignature);

  return { keypairs, mints: mints.map((m) => m.publicKey) };
}

export function ata(mint: PublicKey, keypair: Keypair) {
  return getAssociatedTokenAddressSync(
    mint,
    keypair.publicKey,
    false,
    TOKEN_PROGRAM
  );
}
