use image::{self, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
// i know DynamicImage::invert is a thing, but idc
fn invert_image_color(image: &DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.pixels() {
        output.put_pixel(x, y, Rgba([!pixel[0], !pixel[1], !pixel[2], pixel[3]]));
    }

    DynamicImage::ImageRgba8(output)
}

fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let input_dir: &Path = Path::new("input");
    let output_dir: &Path = Path::new("output");

    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    let images: Vec<DynamicImage> = input_dir
        .read_dir()
        .expect("Failed to read input directory")
        .filter_map(|entry| entry.ok()) // Ignore entries that failed to read
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "png"))
        .filter_map(|path| image::open(&path).ok()) // Ignore images that failed to open
        .collect();

    images.par_iter().enumerate().for_each(|(i, image)| {
        let inverted: DynamicImage = invert_image_color(image);
        let filename: String = format!("{}/image_{}.png", output_dir.display(), i);
        if let Err(e) = inverted.save(&filename) {
            eprintln!("Error saving {}: {}", filename, e);
        }
    });

    println!("Time taken: {:?}", start.elapsed());
    println!("Done processing images.");
}
