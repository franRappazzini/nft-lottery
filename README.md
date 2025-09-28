# NFT Lottery

This project is a smart contract for Solana, developed with Anchor, that implements an NFT lottery. It allows you to create lotteries, buy tickets, select winners randomly, and claim prizes.

## Main Structure

The program is located in `/programs/nft-lottery/` and is composed of the following modules:

- **constants.rs**: Defines seeds and constants used to derive accounts and PDAs.
- **errors.rs**: Lists custom dApp errors, such as ticket limit, expired time, winner not selected, etc.
- **instructions/**: Contains the logic for each instruction:
  - `initialize_config.rs`: Initializes global configuration and treasury.
  - `create_lottery.rs`: Creates a new lottery and its NFT collection.
  - `buy_ticket.rs`: Allows users to buy tickets for a lottery.
  - `commit_randomness_account.rs`: Links the Switchboard randomness account.
  - `select_winner.rs`: Selects the winner using Switchboard randomness.
  - `claim_prize.rs`: Allows the winner to claim the prize.
- **states/**: Defines data structures:
  - `GlobalConfig`: Global program configuration (authority, treasury, fee, etc).
  - `Lottery`: State of each lottery (id, collection, prize, winner, tickets, etc).

## How to Run the Project Locally

1. **Install JS dependencies**
   ```bash
   npm install
   ```
2. **Build the Anchor program**
   ```bash
   anchor build
   ```
3. **Set Solana to local environment**
   ```bash
   solana config set --url localhost
   ```
4. **Run the setup scripts**
   - Navigate to the `/setup` directory:
     ```bash
     cd setup
     ```
   - Make the setup scripts executable:
     ```bash
     chmod +x ./setup-local.sh
     chmod +x ./start-validator.sh
     ```
   - First, initialize the local environment:
     ```bash
     ./setup-local.sh
     ```
   - Then, start the local validator:
     ```bash
     ./start-validator.sh
     ```
5. **Run Anchor tests in another terminal (without starting the local validator)**
   ```bash
   anchor test --skip-local-validator
   ```

## Notes

- The project uses Switchboard for randomness.
- The scripts in `/setup` prepare the environment and validator with required accounts and oracles.
- The contract is written in Rust and uses Anchor to facilitate Solana development.

## Relevant Folder Structure

- `/programs/nft-lottery/`: Contract source code.
- `/setup/`: Scripts and files to set up the local environment.
- `/tests/`: Automated tests.

## Author

franRappazzini

---

For technical questions, check the source files in `/programs/nft-lottery/src/` and the scripts in `/setup/`.
