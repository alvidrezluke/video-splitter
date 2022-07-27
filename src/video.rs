use std::path::Path;
use vid2img::FileSource;

pub fn split_video(file: String) -> Result<Vec<Vec<u8>>, &'static str> {
    let file_path = Path::new(&file);
    let mut images: Vec<Vec<u8>> = vec![];

    let frame_source = FileSource::new(file_path, (1280, 720)).unwrap();
    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            let new_image = grey(png_img_data)?;
            images.push(new_image);
        }
    }

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
