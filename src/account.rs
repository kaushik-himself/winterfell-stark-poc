use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

struct Account {
  id: i32,
  nfts: String,
}

pub fn create_account() -> Result<()> {
  let conn = Connection::open("nft.db")?;

  let nft_arr: Vec<String> = [].to_vec();
  let nfts = serde_json::to_string(&nft_arr).unwrap();
  conn.execute("INSERT INTO Account (nfts) values (?1)", &[&nfts])?;
  println!("Created account with id {}", conn.last_insert_rowid());

  Ok(())
}

pub fn check_balance(account_id: i32) -> Result<()> {
  let conn = Connection::open("nft.db")?;

  let mut get_account = conn.prepare("SELECT id, nfts FROM Account WHERE id = :id;")?;
  let account = get_account.query_row(&[(":id", &account_id)], |row| {
    Ok(Account {
      id: row.get(0)?,
      nfts: row.get(1)?,
    })
  })?;
  let nft_arr: Vec<String> = serde_json::from_str(&account.nfts).unwrap();

  println!("{:?}", nft_arr);

  Ok(())
}
