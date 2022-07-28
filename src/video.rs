use std::path::Path;

use indicatif::ProgressBar;
use vid2img::{FileSource, StreamError};

pub fn split_video(file: String) -> Result<Vec<Vec<u8>>, &'static str> {
    let file_path = Path::new(&file);
    let mut images: Vec<Vec<u8>> = vec![];

    let frame_source = FileSource::new(file_path, (200, 200)).unwrap();
    let frames: Vec<Result<Option<Vec<u8>>, StreamError>> = frame_source.into_iter().filter(check_frame).collect();
    let pb = ProgressBar::new(frames.len().try_into().unwrap());
    println!("Processing {} frames...", frames.len());
    frames.into_iter().for_each(|frame| {
        if let Ok(Some(png_img_data)) = frame {
            let new_image = grey(png_img_data).expect("Could not convert image to grayscale");
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

fn grey(img: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let image_load = image::load_from_memory(&img);
    if image_load.is_err() {
        return Err("could not load image from memory");
    }
    let image = image_load.expect("If check failed");
    let grey_image = image.grayscale();
    let img_data = grey_image.into_bytes();
    Ok(img_data)
}

fn check_frame(frame: &Result<Option<Vec<u8>>, StreamError>) -> bool {
    if let Ok(Some(_png_img_data)) = frame {
        return true;
    }
    false
}