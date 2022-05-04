mod merkle_tree;
use merkle_tree::{MerkleTree, AsBytes};
use serde::{Serialize, Deserialize};
use std::slice;
use std::mem;

#[derive(Serialize, Deserialize)]
pub struct Token {
  id: i32,
  owner: i32,
  token_uri: String,
}

impl AsBytes for Token {
  fn as_bytes(&self) -> &[u8] {
    let p: *const Token = self;
    let p: *const u8 = p as *const u8;
    let s: &[u8] = unsafe {
      slice::from_raw_parts(p, mem::size_of::<Token>())
    };
    return s;
  }
}

pub fn build_merkle_tree() {
  let t1 = Token {
    id: 1,
    owner: 1,
    token_uri: "test1.jpg".to_string(),
  };

  let t2 = Token {
    id: 2,
    owner: 2,
    token_uri: "test2.jpg".to_string(),
  };

  let merkle_tree: MerkleTree = MerkleTree::build(&[t1, t2]);
  println!("{:?}", merkle_tree.root_hash_str());
}
