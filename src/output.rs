use serde_json::Result;
use serde::{Serialize, Deserialize};
use std::{path::Path, fs::OpenOptions, io::Write};

#[derive(Serialize, Deserialize)]
struct AIData {
    image: Vec<u8>,
    letter: [u8; 26]
}

pub fn modify_json(images: Vec<Vec<u8>>, letter: [u8; 26]) -> Result<()> {
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
    file.write(serde_json::to_string(&json_arr)?.as_bytes());
    Ok(())
}

pub fn setup_output() {
    let path = Path::new("./output/output.json");
    let path_exists = path.exists();
    if path_exists {
        std::fs::remove_file(path);
    }
}