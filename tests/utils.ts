import { AnchorProvider, Program, Wallet, web3 } from "@coral-xyz/anchor";
import { Dpl, IDL } from "../target/types/dpl";
import { readFileSync } from "fs";
import path from "path";

export const PROGRAM_ID = new web3.PublicKey(
  "MfQ5MtGrou6TxQuBSGAFiQMuPTUWe7Y7kscbswUw31c",
);

export function createConnection(devnet = true) {
  if (devnet) {
    return new web3.Connection("https://api.devnet.solana.com", "confirmed");
  } else {
    return new web3.Connection("http://localhost:8899", "confirmed");
  }
}

export const deChargeProgram = (wallet: Wallet): Program<Dpl> => {
  return new Program(
    IDL,
    PROGRAM_ID,
    new AnchorProvider(createConnection(), wallet, {
      commitment: "confirmed",
    }),
  ) as unknown as Program<Dpl>;
};

const mainKeypair = web3.Keypair.fromSecretKey(
  Buffer.from(
    JSON.parse(
      readFileSync(path.join(__dirname, "./main_keypair.json"), "utf-8"),
    ),
  ),
);

export const ownerKeypair = web3.Keypair.fromSecretKey(
  Buffer.from(
    JSON.parse(
      readFileSync(
        path.join(
          __dirname,
          "./ownVAWtaFaiPGPjwAHnu5j71VHfGraELuLxwrReR4Jb.json",
        ),
        "utf-8",
      ),
    ),
  ),
);

export async function airdrop(
  connection: web3.Connection,
  to: web3.PublicKey,
  amount?: number,
) {
  // transfer from the default account to the new account
  const tx = new web3.Transaction().add(
    web3.SystemProgram.transfer({
      fromPubkey: new web3.PublicKey(mainKeypair.publicKey),
      toPubkey: to,
      lamports: amount || web3.LAMPORTS_PER_SOL * 0.01,
    }),
  );

  tx.feePayer = mainKeypair.publicKey;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.sign(mainKeypair);

  await connection.sendRawTransaction(tx.serialize());
}

export async function generateFundedKeypair(connection: web3.Connection) {
  const keypair = web3.Keypair.generate();
  await airdrop(connection, keypair.publicKey, web3.LAMPORTS_PER_SOL * 0.1);
  return keypair;
}
