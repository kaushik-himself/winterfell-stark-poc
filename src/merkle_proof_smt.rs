use sparse_merkle_tree::{
    blake2b::Blake2bHasher, default_store::DefaultStore,
    error::Error, MerkleProof,
    SparseMerkleTree, traits::Value, H256
};
use blake2b_rs::{Blake2b, Blake2bBuilder};
use std::slice;
use std::mem;

// define SMT
type SMT = SparseMerkleTree<Blake2bHasher, Token, DefaultStore<Token>>;

// define SMT value
#[derive(Default, Clone, Copy)]
pub struct Token {
  id: i32,
  owner: i32,
}

impl Token {
  fn as_bytes(&self) -> &[u8] {
    let p: *const Token = self;
    let p: *const u8 = p as *const u8;
    let s: &[u8] = unsafe {
      slice::from_raw_parts(p, mem::size_of::<Token>())
    };
    return s;
  }
}

impl Value for Token {
   fn to_h256(&self) -> H256 {
       let mut buf = [0u8; 32];
       let mut hasher = new_blake2b();
       hasher.update(self.as_bytes());
       hasher.finalize(&mut buf);
       buf.into()
   }
   fn zero() -> Self {
       Default::default()
   }
}

// helper function
fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32).personal(b"SMT").build()
}

pub fn construct_smt() {
  let t1 = Token {
    id: 1,
    owner: 1,
  };

  let t2 = Token {
    id: 2,
    owner: 2,
  };

    let mut tree = SMT::default();
    let tokens = vec![t1, t2];
    for (i, token) in tokens.iter().enumerate()
    {
        let key: H256 = {
            let mut buf = [0u8; 32];
            let mut hasher = new_blake2b();
            hasher.update(&(i as u32).to_le_bytes());
            hasher.finalize(&mut buf);
            buf.into()
        };
        // insert key value into tree
        tree.update(key, *token).expect("update");
    }

    println!("SMT root is {:?} ", tree.root());
}