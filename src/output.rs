use serde::{Serialize, Deserialize};
use std::{path::Path, fs::OpenOptions, io::Write};

use crate::error::{Error, self};
use colored::*;

#[derive(Serialize, Deserialize)]
struct AIData {
    image: Vec<u8>,
    letter: [u8; 26]
}

pub fn modify_json(images: Vec<Vec<u8>>, letter: [u8; 26]) -> Result<(), Error> {
    let mut json_arr: Vec<AIData> = vec![];
    let mut file = OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)
    .write(true)
    .open("./output/output.json")
    .unwrap();

    for image in images {
        let data = AIData {
            image,
            letter
        };
        json_arr.push(data);
        
    }
    let json = serde_json::to_string(&json_arr).unwrap();
    let file_bytes_res = json.as_bytes();

    let file_write_res = file.write(file_bytes_res);
    if file_write_res.is_err() {
        return Err(error::new_error("Could not write to output.json file".red().to_string()))
    }
    Ok(())
}

pub fn setup_output() -> Result<(), Error> {
    let path = Path::new("./output/output.json");
    let path_exists = path.exists();
    if path_exists {
        let file_removed_res = std::fs::remove_file(path);
        if file_removed_res.is_err() {
            return Err(error::new_error("Could not delete existing output.txt file.".red().to_string()));
        }
    }
    Ok(())
}