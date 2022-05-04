use std::io::{self, Write};
use rusqlite::{Connection, Result};
use std::env;

mod account;
use account::check_balance;
use account::create_account;

mod mint;
use mint::mint;

mod merkle_proof;
use merkle_proof::build_merkle_tree;

mod merkle_proof_smt;
use merkle_proof_smt::construct_smt;

mod winter_merkle;
use winter_merkle::generate_proof;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    init_db()?;
    assert!(
        args.len() > 1,
        "Please pass a command - create_account, mint or check_balance"
    );
    if args[1] == "create_account" {
        create_account()?;
    }

    if args[1] == "mint" {
        assert_eq!(args.len(), 4, "Please pass the account ID and token URI");
        let account_id_str = &args[2];
        let account_id = account_id_str.parse::<i32>().unwrap();
        mint(account_id, &args[3])?;
    }

    if args[1] == "check_balance" {
        assert_eq!(args.len(), 3, "Please pass the account ID");
        let account_id_str = &args[2];
        let account_id = account_id_str.parse::<i32>().unwrap();
        check_balance(account_id)?;
    }

    if args[1] == "gen_mt" {
        build_merkle_tree();
    }

    if args[1] == "gen_smt" {
        construct_smt();
    }

    if args[1] == "prove_verify" {
        assert_eq!(args.len(), 3, "Please pass the token ID to be verified");
        let token_id_str = &args[2];
        let token_id = token_id_str.parse::<usize>().unwrap();
        generate_proof(token_id);
    }

    Ok(())
}

fn init_db() -> Result<()> {
    let conn = Connection::open("nft.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Account (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nfts STRING
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Token (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uri STRING NON NULL,
            owner INTEGER NON NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Proof (
            proof STRING PRIMARY KEY,
            nfts STRING NON NULL
        )",
        [],
    )?;

    let _ = io::stdout().flush();

    Ok(())
}
