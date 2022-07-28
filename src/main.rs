mod input;
mod video;
mod output;

fn main() -> Result<(), String> {
    let args = input::get_args().expect("Invalid args");
    let file = args.0;
    let letter = args.1;
    let images = video::split_video(file).expect("could not split video");
    output::modify_json(images, letter).expect("Could not parse video");
    Ok(())
}
