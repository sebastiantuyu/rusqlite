use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
use std::usize;

#[derive(Debug)]
struct DbHeader {
    page_size: usize,
}

#[derive(Debug)]
struct Db {
    file: File,
    pointer: usize,
    number_of_cells: usize,
    page: Vec<u8>,
    header: DbHeader,
    tables: Vec<String>
}
const HEADER_SIZE: usize = 100;

impl Db {
    fn bind(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let header = Db::parse_header(&mut file);

        let mut page: Vec<u8> = vec![0; header.page_size];
        _ = file.seek(std::io::SeekFrom::Start(0));
        _ = file.read_exact(&mut page);

        let db = Db {
            page: page.clone(),
            file,
            header,
            pointer: 0,
            number_of_cells: u16::from_be_bytes([
                page[HEADER_SIZE + 3],
                page[HEADER_SIZE + 4]
            ]) as usize,
            tables: vec![]
        };
        return db;
    }

    fn parse_header(file: &mut File) -> DbHeader {
        let mut header = [0; HEADER_SIZE];
        _ = file.read_exact(&mut header);
        let page_size = u16::from_be_bytes([header[16], header[17]]) as usize;

        return DbHeader {
            page_size
        }
    }

    fn info(&mut self) {
        println!("database page size {}", &self.header.page_size);
        println!("number of tables {}", &self.number_of_cells);
    }

    fn parse_page(&mut self) -> &Vec<String>{
        let mut page: Vec<u8> = vec![0; self.header.page_size];
        _ = self.file.seek(std::io::SeekFrom::Start(0));
        _ = self.file.read_exact(&mut page);

        match u8::from_be_bytes([page[HEADER_SIZE]]) {
            0x02 => println!("InteriorIndex"),
            0x05 => println!("InteriorTable"),
            0x0a => {
                println!("LeafIndex");
                self.pointer += 8;
            },
            0x0d => {
                self.pointer += 8;
                println!("LeafTable")
            },
            _ => {
            }
        }

        self.pointer += HEADER_SIZE;

        for i in 0..self.number_of_cells {
            let c = i as usize * 2;
            let c_pos = u16::from_be_bytes([
                page[self.pointer + c],
                page[self.pointer + c + 1]
            ]) as usize;

            self.read_file(c_pos);
        }

        return &self.tables
    }

    fn read_file(&mut self, mut pointer: usize) {
        let _v = read_varint(&self.page, &mut pointer);
        let _row_id = read_varint(&self.page, &mut pointer);

        let previous_pos = pointer;
        let header_size = read_varint(&self.page, &mut pointer);

        let mut remaining_header = header_size - (pointer - previous_pos);
        let mut data_types = vec![];
        while remaining_header > 0 {
            let previous_pos = pointer;
            let data_type = read_varint(&self.page, &mut pointer);
            data_types.push(data_type);
            remaining_header -= pointer - previous_pos;
        }

        let mut row_data: Vec<String> = Vec::new();
        for data_type in data_types {
            let data_size: usize;
            if data_type >= 12 && data_type % 2 == 0 {
                data_size = (data_type - 12) / 2;
            } else if data_type >= 13 && data_type % 2 == 1 {
                data_size = (data_type - 13) / 2;
            } else {
                data_size = data_type
            }
            let record = &self.page[pointer..pointer + data_size];
            row_data.push(String::from_utf8_lossy(record).to_string());
            pointer += data_size;
        }

        if row_data[2] != "sqlite_sequence" {
            self.tables.push(row_data[2].to_string());
        }
    }
}


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

fn read_varint(page: &Vec<u8>, pointer: &mut usize) -> usize {
    // Basically read data until the
    // MSB is 0, meaning that no more data will follow
    let mask = 0b01111111;
    let mut result = 0;
    loop {
        let current_value = page[*pointer] & mask;
        result = (result << 7) | current_value as usize;
        // literally looking for something like 0b00000000
        if (page[*pointer] >> 7) & 1 == 0 {
            break;
        }
        *pointer += 1;
    }
    *pointer += 1;
    result
}