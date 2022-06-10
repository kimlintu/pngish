use std::error::Error;
use pngish::{Picture, RGBPixel, PngImage};

fn main() {
    let side_len: u32 = 256;

    let mut painting = Picture { 
        pixels: Vec::new(),
        width: side_len,
        height: side_len,
    };

    for y in 0..side_len {
        for x in 0..side_len {
            let pixel = RGBPixel { red: 255, green: x as u8, blue: y as u8 };
            painting.pixels.push(pixel);
        }
    }

    let png = PngImage::new(&painting);
    //println!("{:?}", png.data);

    // write png bytes to a file

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("test.png").unwrap();
    file.write_all(&png.signature).unwrap();
    file.write_all(&png.data).unwrap();

    println!("hello");
}
