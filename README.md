# deCharge Program Library

This repository contains the source code for the deCharge Program Library.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Usage](#usage)

## Introduction

The deCharge Solana Program implements the core functionalities for the deCharge dApp, which aims to provide decentralized charging solutions within the Solana ecosystem. This contract facilitates various operations such as creating users, chargers, and managing charging sessions securely on the Solana blockchain.

## Installation

To integrate the deCharge Solana Smart Contract into your Solana project, follow these steps:

1. Ensure you have Rust installed on your system. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/AnishDe12020/dpl.git
    ```

3. Navigate to the root directory and perform `pnpm\npm\yarn install` for testing purpose and `anchor build` to build the program locally.

## Usage

The deCharge program exposes three main entry points:

1. `create_user`: Creates a user with the provided phone number hash.

2. `create_charger`: Creates a new charger provided operator pubkey.
3. `charger_session`: Initiates a charging session with the specified amount.

### Accounts and PDA
User PDA:
```js
web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), userWallet.toBuffer()],
    PROGRAM_ID,
);
```

<br/>
Charger PDA:

```js
web3.PublicKey.findProgramAddressSync(
    [Buffer.from("charger"), chargerWallet.toBuffer()],
    PROGRAM_ID,
);
```
<br/>

### create_user ix
```rs
struct CreateUserAccounts {
    pub user, // payer for the transaction
    pub user_pda, // seeds = [b"user", user_key.buffer()]
    pub system_program,
}

struct CreateUserArgs {
    phone_number_hash: String
}
```

### create_charger ix
```rs
struct CreateChargerAccounts {
    pub payer, // payer for the transaction
    pub charger, // charger pubkey
    pub charger_pda, // seeds = [b"charger", charger.buffer()]
    pub operator, // operator pubkey
    pub nft_mint, // nft mint address
    pub token_program, // token program id
    pub system_program,
}

struct CreateChargerArgs {
    None
}
```

### charger_session ix
```rs
struct ChargerSessionAccounts {
    pub user, // payer for the transaction
    pub user_ata, // user associated token account
    pub user_pda, // seeds = [b"user", user.buffer()]
    pub charger, // charger pubkey
    pub charger_pda, // seeds = [b"charger", charger.buffer()]
    pub mint, // mint address
    pub nft_mint, // nft mint address
    pub nft_mint_owner, // nft mint owner
    pub nft_mint_owner_ata, // nft mint owner associated token account
    pub operator, // operator pubkey
    pub operator_ata, // operator associated token account
    pub token_program, // token program id
    pub system_program,
}

struct ChargerSessionArgs {
    amount: u64
}
```

The tests file is enough to understand the overall frontend integration with the program.