mod db;
mod varint_parser;
use anyhow::{bail, Result};
use db::Db;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let mut db = Db::bind(&args[1]);

    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            db.info();
        }
        ".tables" => {
            let k = db.parse_page();
            dbg!(k);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}