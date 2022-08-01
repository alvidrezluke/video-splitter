use image::DynamicImage;
use indicatif::ProgressBar;

use crate::error::Error;

pub fn export_images(images: Vec<DynamicImage>, letter: String, output_dir: String) -> Result<(), Error> {
    // Set output directory
    let output_directory_path = format!("{}/{}",output_dir, letter);

    // Create directory if it does not already exist
    std::fs::create_dir_all(&output_directory_path).unwrap();

    // Get current number of files in directory to add to them
    let current_files = std::fs::read_dir(&output_directory_path).unwrap();
    let mut index = current_files.into_iter().count();

    let pb = ProgressBar::new(images.len().try_into().unwrap());
    println!("Saving {} frames...", images.len());

    // Save each image to correct directory
    for image in images {
        image.save(format!("./output/{}/{}.png", letter, index)).unwrap();
        index += 1;
        pb.inc(1);
        print!("\rETA: {:.0?}", pb.eta());
    }
    print!("\rETA: 0ms    ");
    pb.finish();
    println!("\n");
    Ok(())
}