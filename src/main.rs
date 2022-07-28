mod input;
mod video;
mod output;

fn main() -> Result<(), String> {
    output::setup_output();
    let input_file_path = "input.txt";
    let input_file = std::fs::read_to_string(input_file_path).expect("Could not find config file.");
    let input_lines = input_file.lines();
    input_lines.into_iter().for_each(|line| {
        let split_line: Vec<&str> = line.trim().split(',').collect();
        if split_line.len() == 2 {
            let file = split_line[0].to_string();
            let letter = input::get_arr_from_letter(split_line[1].to_string()).expect("could not parse letter");
            println!("Loading file: {}", file);
            let images = video::split_video(file).expect("could not split video");
            output::modify_json(images, letter).expect("Could not parse video");
        } else {
            println!("Error. Can not parse line: {}", line);
        }
    });
    Ok(())
}
