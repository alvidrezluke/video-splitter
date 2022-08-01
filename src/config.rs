
#[derive(Debug)]
pub struct ConfigData {
    pub input_dir: String,
    pub input_file: String,
    pub output_dir: String,
    pub img_res: (i32, i32)
}

pub fn fetch_config_data() -> ConfigData {
    // Set config data path
    let config_path = "config.txt";

    // Fetch data from file
    let config_file = std::fs::read_to_string(config_path).unwrap();
    let lines = config_file.lines();

    // Initialize default values
    let mut data = ConfigData {
        img_res: (100, 100),
        input_dir: "./input/".to_string(),
        output_dir: "./output/".to_string(),
        input_file: "input.txt".to_string()
    };

    for line in lines {
        let split_line = line.split_once('=');
        if let Some(line_data) = split_line {
            // Match config command
            match line_data.0 {
                "INPUT_DIR" => {
                    let in_dir = line_data.1.trim().to_string();
                    let cleaned_in_dir = in_dir.replace('"', "");
                    data.output_dir = cleaned_in_dir;
                },
                "OUTPUT_DIR" => {
                    let out_dir = line_data.1.trim().to_string();
                    let cleaned_out_dir = out_dir.replace('"', "");
                    data.output_dir = cleaned_out_dir;
                },
                "IMG_RES" => {
                    let tuple_string = line_data.1.trim().to_string();
                    let nums_string = tuple_string.replace('(', "").replace(')', "");
                    let nums = nums_string.split_once(',').expect("Invalid IMG_RES in config.txt");
                    data.img_res = (nums.0.parse().expect("Invalid nums for IMG_SIZE"), nums.1.parse().expect("Invalid nums for IMG_SIZE"));
                },
                "INPUT_FILE" => data.input_file = line_data.1.trim().to_string(),
                _ => {}
            }
        }
    }
    data
}