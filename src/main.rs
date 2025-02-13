use image::{self, GenericImageView, ImageBuffer, Rgba};

fn main() {
    let image = image::open("image.png").unwrap();
    let (width, height) = image.dimensions();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let start = std::time::Instant::now();

    for (x, y, pixel) in image.pixels() {
        let new_pixel = Rgba([
            !pixel[0],
            !pixel[1],
            !pixel[2],
            pixel[3],
        ]);

        output.put_pixel(x, y, new_pixel);
    }

    output.save("output.png").expect("error writing pixel");
    println!("Time taken: {:?}", start.elapsed());
    println!("Done writing image");
}