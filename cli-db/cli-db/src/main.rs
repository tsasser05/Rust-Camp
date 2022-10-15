use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::env;

#[derive(Debug)]
struct Arguments {
    firstName: String,
    lastName: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        print_usage();
        eprintln!("Wrong number of arguments.");
        std::process::exit(1);
    } // if

    Arguments {
        firstName: args[0].clone(),
        lastName: args[1].clone(),
    }
}

fn print_usage() {
    eprintln!("Add name to database)");
    eprintln!("Usage:  cargo run \"FIRSTNAME\" \"LASTNAME\"");
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    let firstName = args.firstName.clone();
    let lastName = args.lastName.clone();

    let conn = Connection::open("users.db").unwrap();
    conn.execute(
        "CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    firstName TEXT NOT NULL,
    lastName TEXT NOT NULL
  )",
        NO_PARAMS,
    )
    .unwrap();

    conn.execute(
        "INSERT INTO users (firstName, lastName) VALUES (?1, ?2)",
        &[&firstName, &lastName],
    )
    .unwrap();
} // main()
