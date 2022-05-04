# Winterfell STARK PoC

A PoC demonstrating a Stark prover/verifier for NFT transactions using the [Winterfell library](https://github.com/novifinancial/winterfell) for Stark implementation.

The PoC demonstrates the Validium mode of scaling NFT minting using ZK-STARKs. NFTs are minted and stored in a DB. A proof is generated for a batch of NFT. This proof can be published to an L1 blockchain. The L1 blockchain runs a verifier smart contract which imports the Winterfell AIR and Verifier exposed in this PoC.

## Functionalities

For the sake of simplicity the PoC only demonstrates NFT minting and its proof and verification. However, the same can be applied to any generic transaction.

## Design

The PoC demonstrates a Validium model of scaling.

- The minted tokens are simply stored into a SQLite DB for now.
- A ZK-STARK proof is generated periodically for a batch of minted tokens. This proof can be published to an L1 blockchain.
- The L1 blockchain runs the verifier in its Rust smart contract after importing the verifier and AIR winterfell crates.

## About Winterfell

The winterfell library requires a developer to create an AIR (Arithmetization) of an algorithm. The remaining five steps required to produce STARK proofs - Low degree extension, constraint evaluation, constraint composition and low-degree testing are handled by the library internally.  
The library also compiles to cleanly provide prover, verifier and AIR crates, which can later be imported and used from different programs.

## Implementation Details

The PoC involves 4 steps:

- Create an account and mint NFTs.
- Run an accumulator to add the NFTs to a merkle tree.
- Create a ZK STARK proof of the NFTs existense as a leaf of the merkle tree.
  - Implement an AIR for Merkle proofs.
- Verify the STARK proof.

For the sake of simplicity, the minted NFTs and their metadata are stored in a SQLite DB.

## How does this help with scaling?

When N NFTs are minted, validating the proof of each NFT added to a Merkle Tree takes O(log(N)).
Using Starks we optimize this further and guarantee sub-logarthmic time in validating each proof.

## Limitations

Since this is a validium, the design has data availability issues. This can be fixed by implementing a ZK rollup.

2<sup>15</sup> is the maximum number of NFTs supported by the Merkle Tree.
This is an arbitrary figure hard-coded for convenience and can be changed through a few trivial updates.

Note: Currently the Rescue hashing algorithm is used for generating the Merkle Tree. The Stark proofs - prover, AIR and trace - will need to be updated for using any different hashing algorithm.

## Steps to run

- Make sure that you have cargo/rust setup.

If you are on Mac ensure `rustup target add aarch64-apple-ios aarch64-apple-darwin x86_64-apple-ios x86_64-apple-darwin` is done to add targets.

- Navigate to `$PROJECT_HOME`.
- Run `cargo build`.

If you're on Mac, you need to add architecture appropriately. For ex on M1, you need to `cargo build --target=x86_64-apple-darwin`


- `cargo run create_account`.

You might need to pass target flags like `cargo run --target=x86_64-apple-darwin create_account`. You will need to pass flags in this manner for any other commands listed below as well.

- `cargo run mint <account_id> <uri>`
- Run the above command a few times.
- Generate and verify a stark proof by running:
`cargo run prove_verify <token_id>`.
