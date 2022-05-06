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

## Implementation Details

The PoC involves 4 steps:

- Create an account and mint NFTs.
- Run an accumulator to add the NFTs to a merkle tree.
- Create a ZK STARK proof of the NFTs existence as a leaf of the merkle tree.
  - Implement an AIR for Merkle proofs.
- Verify the STARK proof with the merkle root as the public input.

For the sake of simplicity, the minted NFTs and their metadata are stored in a SQLite DB.

## How does this help with scaling?

Rather than mint an NFT on an L1 blockchain which has high gas cost, the NFT is minted on an L2. Only the STARK proofs are periodically published to L1, where a smart contract verifies it and stores the proof once the verification is successful.

This enables scale by doing the following:

- ZK STARK proofs are succinct. The amount of space consumed by the proof is polylogarithmic in comparison to the size of the actual data.
- The proofs are sent to the smart contract as a part of calldata. The calldata storage costs a lot lower than storage in the blockchain memory.
- The computation of ZK STARK proofs is highly efficient. This saves gas by reducing the amount of computation needed.

The above three factors lead to higher throughput and lower costs.

## Privacy

The PoC can prove that an NFT has been minted and it exists without revealing any further details about the NFT. This can find application as a membership validator that maintains the member's anonymity. The smart contract verifier takes the root of the merkle tree as an input and a proof of the existence of the NFT as a leaf node of the merkle tree which reveals no details about the NFT itself.

## Limitations

Since this is a validium, the design has data availability issues. This can be fixed by implementing a ZK rollup.

2<sup>15</sup> is the maximum number of NFTs supported by the Merkle Tree.
This is an arbitrary figure hard-coded for convenience and can be changed through a few trivial updates.

## Steps to run

Make sure that you have cargo/rust setup.

If you are on Mac ensure `rustup target add aarch64-apple-ios aarch64-apple-darwin x86_64-apple-ios x86_64-apple-darwin` is done to add targets.

- Navigate to `$PROJECT_HOME`.
- Run `cargo build`.
  - If you're on Mac, you need to add architecture appropriately. For ex on M1, you need to `cargo build --target=x86_64-apple-darwin`
- `cargo run create_account`.
  - You might need to pass target flags like `cargo run --target=x86_64-apple-darwin create_account`. You will need to pass flags in this manner for any other commands listed below as well.
- `cargo run mint <account_id> <uri>`
- Run the above command a few times.
- Generate and verify a stark proof by running:
`cargo run prove_verify <token_id>`.


## About Winterfell

The winterfell library requires a developer to create an AIR (Arithmetization) of an algorithm. The remaining five steps required to produce STARK proofs - Low degree extension, constraint evaluation, constraint composition and low-degree testing are handled by the library internally.  
The library also compiles to cleanly provide prover, verifier and AIR crates, which can later be imported and used from different programs.

### Defining a Winterfell AIR and prover

A Winterfell AIR comprises of the following:

1. Execution trace
2. Transition constraints
3. State constraints
4. Public inputs

Firstly, the program needs to converted into arithmetic form. This can be easily accomplished by using a hashing function. In this PoC, we make use of a merkle tree which consists of the hashed integer values of the tokens to be stored.

The execution trace is an array which stores all the intermediate states of the program's execution.

The transition constraints define the constraints to be evaluated in the intermediate transitions of the program's execution. The constraints are evaluated against the execution trace. In this PoC, the constraint evaluates the computation of the merkle tree - specifically the process of hashing to sibling nodes together using Rescue128 to compute the parent node.

The state constraints are the constraints defined on the initial/final state of the program's execution. In this PoC, the initial state is the leaf node and the final state is the root node for a valid proof.

Finally, the public inputs define the inputs which are publicly available to everyone including the verifier. In this PoC the public input is the root of the merkle tree.
