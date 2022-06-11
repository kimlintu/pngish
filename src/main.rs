use pngish::{Picture, RGBPixel, PngImage};

fn main() {
	let width = 256;
	let height = 256;

    let mut break_i = width;

	let mut pixels = Vec::new();
    for _ in 0..height {
        for _ in 0..break_i {
            let pixel = RGBPixel { red: 255, green: 0, blue: 0 };
            pixels.push(pixel);
        }

        for _ in break_i..width {
            let pixel = RGBPixel { red: 0, green: 255, blue: 0 };
            pixels.push(pixel);
        }

        break_i -= 1;
    }

    let picture = Picture { 
        pixels,
        width,
        height,
    };

    let png = PngImage::new(&picture);

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("examples/example.png").unwrap();
    file.write_all(&png.signature).unwrap();
    file.write_all(&png.data).unwrap();

    //TODO: incorrect bytes in compression header
    //TODO: crc
}
