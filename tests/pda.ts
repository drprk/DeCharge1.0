import { web3, BN } from "@coral-xyz/anchor";
import { PROGRAM_ID } from "./utils";

export const getUserPDA = (wallet: web3.PublicKey) => {
  // user wallet
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), wallet.toBuffer()],
    PROGRAM_ID,
  );
};

export const getChargerPDA = (wallet: web3.PublicKey) => {
  // charger wallet
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("charger"), wallet.toBuffer()],
    PROGRAM_ID,
  );
};
