import { Wallet, web3 } from "@coral-xyz/anchor";
import {
  createConnection,
  deChargeProgram,
  airdrop,
  ownerKeypair,
} from "./utils";
import { getChargerPDA, getUserPDA } from "./pda";
import { expect } from "chai";

console.log("createCharger Test....");

const connection = createConnection();

describe("Charger", () => {
  const nftMint = new web3.PublicKey(
    "AzbXdnsdYR8kiJr76bX2vvKLuSy7gS6e5fRsRvPKUfYL",
  );
  const tokenProgram = new web3.PublicKey(
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
  );

  it("success: should create a charger", async () => {
    const ownerWallet = new Wallet(ownerKeypair);
    const operatorWallet = new Wallet(web3.Keypair.generate());
    const chargerKey = web3.Keypair.generate();

    await airdrop(
      connection,
      ownerWallet.publicKey,
      web3.LAMPORTS_PER_SOL * 0.1,
    );

    const program = deChargeProgram(ownerWallet);

    const [chargerPda, _] = getChargerPDA(chargerKey.publicKey);
    console.log("chargerPda", chargerPda.toBase58());

    const tx = await program.methods
      .createCharger()
      .accounts({
        operator: operatorWallet.publicKey,
        payer: ownerWallet.publicKey,
        charger: chargerKey.publicKey,
        nftMint,
        chargerPda,
        tokenProgram,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([ownerWallet.payer])
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
        maxRetries: 3,
      });

    console.log("created charger", tx);

    // const chargerAccount = await program.account.charger.fetch(chargerPda);
    // expect(chargerAccount.allTimeRevenue).equal(0);
  });

  // it("fail: should not create a user with phone <> 32", async () => {
  //   const userWallet = new Wallet(web3.Keypair.generate());
  //   await airdrop(
  //     connection,
  //     userWallet.publicKey,
  //     web3.LAMPORTS_PER_SOL * 0.1,
  //   );
  //   const program = deChargeProgram(userWallet);

  //   const [userPda, _] = getUserPDA(userWallet.publicKey);

  //   try {
  //     const tx = await program.methods
  //       .createUser("123456789012345678901234567890123")
  //       .accounts({
  //         user: userWallet.publicKey,
  //         userPda,
  //         systemProgram: web3.SystemProgram.programId,
  //       })
  //       .signers([userWallet.payer])
  //       .rpc({
  //         skipPreflight: true,
  //         commitment: "confirmed",
  //         maxRetries: 3,
  //       });

  //     console.log("created user fail", tx);
  //   } catch (e) {
  //     expect(e).to.be.an("error");
  //   }
  // });
});
