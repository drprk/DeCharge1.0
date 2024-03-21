import { Wallet, web3 } from "@coral-xyz/anchor";
import { createConnection, deChargeProgram, airdrop } from "./utils";
import { getUserPDA } from "./pda";
import { expect } from "chai";

console.log("createUser Test....");

const connection = createConnection();

describe("User", () => {
  // 32 bytes hash of the user's phone number
  let phoneNumberHash = "12345678901234567890123456789012";

  it("success: should create a user", async () => {
    const userWallet = new Wallet(web3.Keypair.generate());
    await airdrop(
      connection,
      userWallet.publicKey,
      web3.LAMPORTS_PER_SOL * 0.1,
    );
    const program = deChargeProgram(userWallet);

    const [userPda, _] = getUserPDA(userWallet.publicKey);

    const tx = await program.methods
      .createUser(phoneNumberHash)
      .accounts({
        user: userWallet.publicKey,
        userPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([userWallet.payer])
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
        maxRetries: 3,
      });

    console.log("created user", tx);

    const userAccount = await program.account.user.fetch(userPda);
    expect(userAccount.phoneNumberHash).equal(phoneNumberHash);
  });

  it("fail: should not create a user with phone <> 32", async () => {
    const userWallet = new Wallet(web3.Keypair.generate());
    await airdrop(
      connection,
      userWallet.publicKey,
      web3.LAMPORTS_PER_SOL * 0.1,
    );
    const program = deChargeProgram(userWallet);

    const [userPda, _] = getUserPDA(userWallet.publicKey);

    try {
      const tx = await program.methods
        .createUser("123456789012345678901234567890123")
        .accounts({
          user: userWallet.publicKey,
          userPda,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([userWallet.payer])
        .rpc({
          skipPreflight: true,
          commitment: "confirmed",
          maxRetries: 3,
        });

      console.log("created user fail", tx);
    } catch (e) {
      expect(e).to.be.an("error");
    }
  });
});
