use rusqlite::{named_params, Connection, Result};

pub fn mint(to: i32, token_uri: &str) -> Result<()> {
  let conn = Connection::open("nft.db")?;

  // Check if the `to` account exists
  let mut get_account = conn.prepare("SELECT id, nfts FROM Account WHERE id = :id;")?;
  let account_exists = get_account.exists(&[(":id", &to)])?;
  assert!(account_exists, "To account does not exist.");

  // Create a token in the DB.
  conn.execute(
    "INSERT INTO Token (uri, owner) VALUES (:uri, :owner)",
    named_params! {
      ":uri": token_uri,
      ":owner": to,
    },
  )?;
  let token_id = conn.last_insert_rowid();

  println!("Minted a fresh NFT {}", token_id);

  Ok(())
}
