use colored::*;

mod config;
mod video;
mod output;
mod error;

fn main() -> Result<(), error::Error> {
    let config_data = config::fetch_config_data();
    let input_file_res = std::fs::read_to_string(config_data.input_file);
    if input_file_res.is_err() {
        return Err(error::new_error("Could not read input.txt".red().to_string()))
    }
    let input_file = input_file_res.unwrap();
    let input_lines = input_file.lines();

    input_lines.into_iter().for_each(|line: &str|{
        let split_line: Vec<&str> = line.trim().split(',').collect();
        if split_line.len() == 2 {
            let file = config_data.input_dir.to_string() + split_line[0];
            let letter = split_line[1].to_string();
            println!("Loading file: {}", file);
            let images = video::split_video(file).unwrap();
            output::export_images(images, letter, config_data.output_dir.clone()).expect("Could not parse video");
        } else {
            println!("Error. Can not parse line: {}", line);
        }
    });

    Ok(())
}
