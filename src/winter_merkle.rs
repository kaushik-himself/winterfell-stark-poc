use std::slice;
use std::mem;
use winterfell::{
    crypto::{Digest, MerkleTree},
    math::{fields::f128::BaseElement, log2, FieldElement, StarkField},
    ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError, HashFunction, FieldExtension
};


use rusqlite::{Connection, Result};

mod rescue;
use rescue::{Hash, Rescue128};

mod prover;
use prover::MerkleProver;

mod air;
use air::{MerkleAir, PublicInputs};

/// Function state is set to 6 field elements or 96 bytes; 4 elements are reserved for rate
/// and 2 elements are reserved for capacity.
pub const HASH_STATE_WIDTH: usize = 6;
pub const RATE_WIDTH: usize = 4;

/// Two elements (32-bytes) are returned as digest.
const DIGEST_SIZE: usize = 2;

/// Minimum cycle length required to describe Rescue permutation.
pub const HASH_CYCLE_LEN: usize = 8;

const TRACE_WIDTH: usize = 7;

pub const NUM_HASH_ROUNDS: usize = 7;

struct Token {
  id: i32,
}

fn get_all_tokens() -> Vec<i32> {
  let conn = Connection::open("nft.db").unwrap();

  let mut stmt = conn.prepare("SELECT id FROM Token").unwrap();
  let res = stmt.query_map([], |row| Ok(Token {id: row.get(0)?})).unwrap();
  let mut tokens: Vec<i32> = Vec::new();
  for r in res {
    tokens.push(r.unwrap().id);
  }
  assert!(tokens.len() % 2 == 0, "Even number of tokens needed. Mint one more");
  tokens.sort();

  tokens
}

pub fn generate_proof(token_id: usize) {
  let tokens = get_all_tokens();
  println!("Generating a merkle tree for {:?} tokens.", tokens.len());
  let base_tokens: Vec<BaseElement> = tokens.iter()
    .map(|t| BaseElement::new(*t as u128)).collect();

  let tree_depth: usize = 15;
  assert!(
    tokens.len() < (2 as usize).pow((tree_depth + 1) as u32),
    "Increase the tree depth");
  
  let index = (token_id - 1)/2;
  let token_index = if token_id % 2 == 0 { token_id - 2 } else { token_id - 1};
  let value = [base_tokens[token_index], base_tokens[token_index + 1]];
  println!("Value {:?}",
    Rescue128::digest(&[base_tokens[index], base_tokens[index + 1]]));
  let merkle_tree = build_merkle_tree(
    tree_depth,
    base_tokens
  );

  println!("Merkle leaf value: {:?}", merkle_tree.leaves()[index]);
  println!(
    "Generating a merkle tree of depth {} with root {:?}",
    tree_depth,
    merkle_tree.root());

  let proof = merkle_tree.prove(index).unwrap();
  println!("Generated a merkle tree proof of length {}", proof.len());
  // Regular proof
  assert!(
    MerkleTree::<Rescue128>::verify(*merkle_tree.root(), index, &proof)
    .is_ok());
  println!("Verified the non-stark merkle proof");
  assert!(
    MerkleTree::<Rescue128>::verify(*merkle_tree.root(), index+1, &proof).is_err());
  println!("Verified failure for an invalid non-stark merkle proof");

    // Define proof options; these will be enough for ~96-bit security level.
  let options = ProofOptions::new(
    28, 8, 0, HashFunction::Blake3_256, FieldExtension::None, 4, 256);

  let prover = MerkleProver::new(options.clone());
  let trace = prover.build_trace(value, &proof, index);
  let stark_proof = prover.prove(trace).unwrap();
  println!("Generated a stark proof for the Merkle Tree, leaf index {}", index);

  let pub_inputs = PublicInputs {
    tree_root: merkle_tree.root().to_elements(),
  };
  
  // Stark proof
  let verification =
    match winterfell::verify::<MerkleAir>(stark_proof, pub_inputs) {
      Ok(_) => println!("Verified the stark proof successfully"),
      Err(error) => panic!("Verification failed with error {:?}", error),
    };
}

fn build_merkle_tree(depth: usize, base_tokens: Vec<BaseElement>) -> MerkleTree<Rescue128> {
    let num_leaves = usize::pow(2, depth as u32);
    let leaf_elements: Vec<BaseElement> = vec![BaseElement::new(0); num_leaves * 2];
    let mut leaves = Vec::new();
    for i in (0..leaf_elements.len()).step_by(2) {
        leaves.push(Hash::new(leaf_elements[i], leaf_elements[i + 1]));
    }

    let mut j = 0;
    for i in (0..base_tokens.len()).step_by(2) {
      leaves[j] = Rescue128::digest(&[base_tokens[i], base_tokens[i+1]]);
      j = j+1;
    }
    
    MerkleTree::new(leaves).unwrap()
}
