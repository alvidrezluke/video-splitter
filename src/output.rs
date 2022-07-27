use serde_json::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct AIData {
    image: Vec<u8>,
    letter: [u8; 26]
}

pub fn modify_json(images: Vec<Vec<u8>>, letter: [u8; 26]) -> Result<()> {
    let mut json_arr: Vec<AIData> = vec![];
    for image in images {
        let data = AIData {
            image,
            letter
        };
        json_arr.push(data);
        
    }
    std::fs::write("./output/output.json", serde_json::to_string(&json_arr)?).expect("Unable to write file");
    Ok(())
}