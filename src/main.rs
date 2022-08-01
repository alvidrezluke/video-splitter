use colored::*;

mod video;
mod output;
mod error;

fn main() -> Result<(), error::Error> {

    let input_file_path = "input.txt";
    let input_file_res = std::fs::read_to_string(input_file_path);
    if input_file_res.is_err() {
        return Err(error::new_error("Could not read input.txt".red().to_string()))
    }
    let input_file = input_file_res.unwrap();
    let input_lines = input_file.lines();

    input_lines.into_iter().for_each(|line: &str|{
        let split_line: Vec<&str> = line.trim().split(',').collect();
        if split_line.len() == 2 {
            let file = split_line[0].to_string();
            let letter = split_line[1].to_string();
            println!("Loading file: {}", file);
            let images = video::split_video(file).unwrap();
            output::export_images(images, letter).expect("Could not parse video");
        } else {
            println!("Error. Can not parse line: {}", line);
        }
    });

    Ok(())
}
