# Winterfell NFT

A PoC demonstrating a Stark prover/verifier for NFT transactions using the [Winterfell library](https://github.com/novifinancial/winterfell) for Stark implementation.

## Functionalities

- [x] NFT Minting
- [ ] NFT Transactions
- [x] Stark prover/verifier for NFTs minted
- [ ] Update proof on NFT transfer

## Implementation Details

Each NFT that is minted is added to a Merkle Tree. The Merkle Tree provides functionality to prove that a particular element exists in the tree.

A Stark proof is generated for each NFT that exists in the Merkle Tree. A Stark verifier can optimally check that the Stark proof is valid and the NFT really does exist in the Merkle Tree.

Any transaction - (both fungible and non-fungible) token minting, transfer - leads to an addition/update of the Merkle Tree. A new proof is generated each time a new leaf is added or an existing leaf is updated in the tree.

A mapping is stored (to be implemented) between each token and its corresponding proof.

For the sake of simplicity, the minted NFTs and their metadata are stored in a SQLite DB.

## How does this help with scaling?

When N NFTs are minted, validating the proof of each NFT added to a Merkle Tree takes O(log(N)).
Using Starks we optimize this further and guarantee sub-logarthmic time in validating each proof.

## Limitations

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
