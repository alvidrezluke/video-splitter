use std::path::Path;

use image::DynamicImage;
use indicatif::ProgressBar;
use vid2img::{FileSource, StreamError};

use crate::error::{Error, self};
use colored::*;

pub fn split_video(file: String) -> Result<Vec<DynamicImage>, Error> {
    let file_path = Path::new(&file);
    let mut images: Vec<DynamicImage> = vec![];

    // Open video
    let frame_source = FileSource::new(file_path, (100, 100)).unwrap();

    // Split video into frames and collect into vec
    let frames: Vec<Result<Option<Vec<u8>>, StreamError>> = frame_source.into_iter().filter(check_frame).collect();

    let pb = ProgressBar::new(frames.len().try_into().unwrap());
    println!("Processing {} frames...", frames.len());
    
    // Iterate through each image
    frames.into_iter().for_each(|frame| {
        // Check frame data exists
        if let Ok(Some(png_img_data)) = frame {
            
            // Greyscale image
            let new_image = grey(png_img_data).unwrap();

            // Add image to output
            images.push(new_image);
            pb.inc(1);
            print!("\rETA: {:.0?}", pb.eta());
        }
    });
    print!("\rETA: 0ms    ");
    pb.finish();
    println!("\n");
    Ok(images)
}

// Turn image to grey
fn grey(img: Vec<u8>) -> Result<DynamicImage, Error> {
    // Change image to image instead of vector of bytes
    let image_load = image::load_from_memory(&img);
    if image_load.is_err() {
        return Err(error::new_error("Could not load image from memory".red().to_string()));
    }
    let image = image_load.expect("If check failed");
    let grey_image = image.grayscale();
    Ok(grey_image)
}

// Ensure that the frame contains image data
fn check_frame(frame: &Result<Option<Vec<u8>>, StreamError>) -> bool {
    if let Ok(Some(_png_img_data)) = frame {
        return true;
    }
    false
}