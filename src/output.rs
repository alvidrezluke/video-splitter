use image::DynamicImage;
use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};

use crate::error::Error;

#[derive(Serialize, Deserialize)]
struct AIData {
    image: Vec<u8>,
    letter: [u8; 26]
}

pub fn export_images(images: Vec<DynamicImage>, letter: String) -> Result<(), Error> {
    std::fs::create_dir_all(format!("./output/{}", letter)).unwrap();
    let pb = ProgressBar::new(images.len().try_into().unwrap());
    println!("Saving {} frames...", images.len());
    for (index, image) in images.into_iter().enumerate() {
        image.save(format!("./output/{}/{}.png", letter, index)).unwrap();
        pb.inc(1);
        print!("\rETA: {:.0?}", pb.eta());
    }
    print!("\rETA: 0ms    ");
    pb.finish();
    println!("\n");
    Ok(())
}