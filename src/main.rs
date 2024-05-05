use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            let page_size = u16::from_be_bytes([header[16], header[17]]);
            println!("[Sqlite]: Running");

            println!("database page size: {}", page_size);

            file.seek(std::io::SeekFrom::Start(0))?;
            let mut body = [0; 4096];
            file.read_exact(&mut body)?;

            println!("number of tables: {}", u16::from_be_bytes([body[100 + 3], body[100 + 4]]));
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
