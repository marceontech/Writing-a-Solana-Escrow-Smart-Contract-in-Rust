# Writing-a-Solana-Escrow-Smart-Contract-in-Rust

# Explanation:

Buyer deposits funds into an escrow smart contract.
Seller provides the asset (SPL token, NFT, or other assets).
Contract releases funds once predefined conditions are met.

# Main Functions:

create_escrow(): Locks funds from the buyer in escrow.

approve_escrow(): Buyer must approve the transaction.

release_funds(): Transfers funds to the seller once conditions are met.


# Interacting with the Escrow Smart Contract on CLI:

1) Create an escrow contract:

solana program invoke --program-id <PROGRAM_ID> --signer <WALLET> --data create_escrow

2) Approve escrow transaction:

solana program invoke --program-id <PROGRAM_ID> --signer <WALLET> --data approve_escrow

3) Release funds to seller:

solana program invoke --program-id <PROGRAM_ID> --signer <WALLET> --data release_funds






