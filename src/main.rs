#![allow(non_snake_case)] // huh

use std::env;
use std::fs::{File, metadata, OpenOptions};
use std::io::Read;

const PROGRAM_NAME: &'static str = env!("CARGO_PKG_NAME");

// todo: do not depend on "png" crate.
fn main() {
    let mut args = env::args();
    let program_name = match args.next() {
        Some(p) => { p }
        None => unreachable!("No program name found??")
    };
    let input_file_paths = args.collect::<Vec<_>>();
    if input_file_paths.len() == 0 {
        println!("No file paths were provided.");
        println!("Usage: {program_name} <input files...>");
        return;
    }
    for file_path in input_file_paths {
        let mut file = match OpenOptions::new()
            .read(true)
            .open(&file_path) {
            Ok(f) => { f }
            Err(err) => {
                eprintln!("Error opening file: {0}.", &file_path);
                eprintln!("{err:?}");
                continue;
            }
        };

        let metadata = match metadata(&file_path) {
            Ok(m) => { m }
            Err(err) => {
                eprintln!("Error reading file metadata for: {0}.", &file_path);
                eprintln!("{err:?}");
                continue;
            }
        };

        if metadata.len() == 0 {
            println!("File {file_path} was an empty file.");
            continue;
        }

        let mut file_buffer = vec![0u8; metadata.len() as usize];
        match file.read(&mut file_buffer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading file {0} into buffer.", &file_path);
                eprintln!("{err:?}");
                continue;
            }
        };

        let mut file_map = [[0usize; 256]; 256];
        for byte in file_buffer.windows(2) {
            let x = byte[0] as usize;
            let y = byte[1] as usize;

            file_map[y][x] = 0xFFFFFFFF;
        }

        let output_file_path = format!("{file_path}.{}.png", PROGRAM_NAME);
        let mut output_file = match File::create(&output_file_path) {
            Ok(of) => { of }
            Err(err) => {
                eprintln!("Error creating output file {1} for input file {0}.", &file_path, &output_file_path);
                eprintln!("{err:?}");
                continue;
            }
        };
        let data: Vec<_> = file_map.iter().flat_map(|row| row.iter()).map(|x| *x as u8).collect();
        let encoder = png::Encoder::new(&mut output_file, 256, 256);
        let mut writer = match encoder.write_header() {
            Ok(ok) => { ok }
            Err(err) => {
                eprintln!("Error writing to file.");
                eprintln!("{err:?}");
                continue;
            }
        };
        match writer.write_image_data(data.as_slice()) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error writing to file.");
                eprintln!("{err:?}");
                continue;
            }
        };
    }
}

