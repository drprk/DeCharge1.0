# DeCharge
DePIN for EV Charging

DeCharge Program Library README

Table of Contents 

1. Introduction
2. Installation
3. Usage
4. API Integrations

   a. Create_user
   
   b. Create_charger

   c. charger_session

   d. API Integration

6. Concept and Vision
7. Technical Architecture
8. Business Model
9. Product and How It Works
10. Problem Statement
11. Solutions and Benefits
12. Security Considerations
13. Testing and Development
14. Contributing
15. License
16. About Us


This walkthrough will cover the essence of deCharge from technical specifics to its business model and market solutions, aimed at a broad audience including developers, investors, and potential users.

---

## Introduction

DeCharge stands at the forefront of blockchain and EV charging innovation, aiming to solve critical issues facing electric vehicle (EV) owners today. As the adoption of EVs accelerates globally, the demand for accessible, reliable, and efficient charging infrastructure has never been higher. DeCharge introduces a groundbreaking solution by leveraging the Solana blockchain to create a decentralized network of smart, GPS-enabled charging stations. This initiative not only facilitates easier access to charging options but also introduces a novel approach to ownership and transactions within the ecosystem, providing a seamless experience for EV owners, venue hosts, and investors.

## Installation

For developers and contributors looking to explore, modify, or integrate the deCharge Solana Smart Contract, the installation process is straightforward:

1. **Rust Installation:** Essential for compiling and building Solana programs. Detailed installation instructions are available at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

2. **Repository Cloning:**
   ```sh
   git clone https://github.com/AnishDe12020/dpl.git
   ```
   This step makes the deCharge program codebase accessible for further actions.

3. **Dependency Installation:** Navigate to the cloned directory and execute `pnpm install`, `npm install`, or `yarn install` to set up the necessary dependencies for the project.

4. **Building the Program:** Compile the smart contract using `anchor build` to ensure it's ready for deployment and testing on the Solana blockchain.

## Usage

DeCharge’s program library is crafted to enable key functionalities within the dApp, facilitating the registration of users and chargers, and managing charging sessions. This user-centric approach ensures that the interactions within the ecosystem are streamlined and secure, embodying the core principles of accessibility and efficiency that deCharge champions.

## API Integrations

### `create_user`

- **Overview:** Registers users in the deCharge network, using a hash of their phone number for privacy.
- **Technical Insight:** This function demonstrates deCharge’s commitment to user privacy and security by requiring only a hashed version of the phone number, minimizing personal data exposure on the blockchain.

### `create_charger`

- **Overview:** Facilitates the addition of new charging stations, broadening the network's infrastructure.
- **Technical Insight:** It underscores the scalability of deCharge, with each charger uniquely identified and managed through the blockchain, enhancing the ecosystem's robustness and accessibility.

### `charger_session`

- **Overview:** Manages the charging session details, including payment and session initiation.
- **Technical Insight:** This function is a testament to deCharge’s innovative approach to transaction management and revenue distribution, ensuring transparency and efficiency through blockchain technology.


## Usage

The deCharge program exposes three main entry points:

1. `create_user`: Creates a user with the provided phone number hash.

2. `create_charger`: Creates a new charger provided operator pubkey.
3. `charger_session`: Initiates a charging session with the specified amount.

### Accounts and PDA

Program ID:
```md
8uwxL2etDeowSko5FXiZLfUJCU9qwFQsmsqrLKjQpFV4
```

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
## Concept and Vision

DeCharge envisions a future where the adoption of electric vehicles is unfettered by the limitations of current charging infrastructure. The project is poised to address the challenges of accessibility, reliability, and transaction transparency through a decentralized network of charging stations. This network is not only a technological leap forward but also a step towards sustainable transportation, leveraging blockchain to ensure a seamless, secure, and user-friendly experience for all stakeholders involved.

## Technical Architecture

The foundation of deCharge is built upon the Solana blockchain, chosen for its high throughput, low transaction costs, and robust security features. The technical architecture integrates smart contracts for managing users, chargers, and charging sessions, incorporating NFTs for unique charger identification and ownership, and a tokenized reward and payment system. This sophisticated yet elegant blockchain implementation enables deCharge to offer a scalable, secure, and efficient platform for EV charging solutions.

## Business Model

### Revenue Streams

- **Charging Service Fees:** Users pay a service fee for accessing the charging stations, creating a steady revenue stream for station hosts and NFT owners.
- **NFT Marketplace:** The trading of charger NFTs in a secondary market allows for dynamic ownership and investment opportunities, generating additional revenue.
- **Staking Incentives:** Users, hosts, and investors can stake deCharge tokens to receive rewards, promoting long-term engagement and platform growth.

### Cost Structure

- **Initial Setup:** Includes the deployment of smart charging stations and the development of the blockchain-integrated platform.
- **Operational Expenses:** Ongoing costs associated with platform maintenance, development, and marketing efforts to expand the deCharge ecosystem.

## Product and How It Works

DeCharge revolutionizes the EV charging experience by offering a user-friendly app that connects EV owners with an extensive network of blockchain-powered charging stations. Users can easily locate, book, and pay for charging sessions through the app, with the added benefit of earning rewards in deCharge tokens. This seamless integration of blockchain technology enhances the overall utility and accessibility of EV charging solutions, addressing the critical needs of the EV community.

## Problem Statement

The rapid adoption of electric vehicles has outpaced the development of supportive charging infrastructure, creating a gap in accessibility, reliability, and transparency in charging services. This disparity poses a significant barrier to the continued growth of sustainable transportation, necessitating an innovative solution that can address these challenges head-on.

## Solutions and Benefits

DeCharge addresses the aforementioned challenges through its decentralized network of smart charging stations, offering:

- **Improved Accessibility:** A GPS-enabled app that simplifies locating and booking charging stations.
- **Decentralized Ownership:** Incorporates NFTs for charger co-ownership, democratizing investment opportunities.
- **Transparent Transactions:** Leverages blockchain for clear, secure transactions and rewards distribution, fostering trust and engagement within the ecosystem.

## Information architecture and product flow

![Untitled-2024-01-07-1704](https://github.com/drprk/DeCharge/assets/88657058/e9ff9d6c-bba6-4b93-91d3-c919b3065a91)


## Security Considerations

DeCharge prioritizes security, implementing state-of-the-art encryption and smart contract audits to protect users' funds and data. The platform's adherence to best security practices ensures a safe and reliable environment for all transactions and interactions within the ecosystem.

## Testing and Development

A comprehensive testing framework underscores deCharge's commitment to quality and reliability. Unit and integration tests ensure the robustness of smart contracts and APK interactions, while user acceptance testing guarantees the platform's usability and intuitiveness. This meticulous approach to development and testing is critical for fostering user trust and satisfaction.

## Contributing

DeCharge invites contributions from developers, blockchain enthusiasts, and visionaries eager to make an impact in the realms of sustainable transportation and blockchain technology. The project offers a collaborative environment where contributions in code, ideas, and community support are highly valued, driving the platform toward continuous improvement and innovation.

## License

The deCharge Program Library is made available under the MIT License, encouraging open collaboration and the free use, modification, and distribution of the software, aligning with the project's ethos of openness and community-driven development.

## About Us

The deCharge team comprises individuals passionate about leveraging technology for environmental sustainability. With expertise spanning blockchain development, software engineering, and renewable energy, the team is united by the goal of accelerating the adoption of electric vehicles through innovative charging solutions, contributing to a cleaner, greener future.

---

This detailed overview of the deCharge Program Library README provides a holistic view of the project, from its technical underpinnings to its business model, and how it seeks to address the current challenges in the EV charging market.

