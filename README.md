# pngish
This is a very primitive PNG encoder written in Rust, and was mainly created to learn more about the language.

At its current state it's possible to encode RGB data into a PNG image, see example below. No filtering or fancy conversions is happening here, it just takes a vector of "RGB pixels" and writes them directly to an image.

# Example
Import the required structs: 
```
use pngish::{Picture, RGBPixel, PngImage};
```

First we can create a vector of *RGBPixel*s, an example:
```
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
```

Then a *Picture* struct can be created with our pixels:
```
let picture = Picture { 
	pixels,
	width,
	height,
};
```

The *Picture* struct is then used to generate the PNG image data:
```
let png = PngImage::new(&picture);
```

Which then can be written to a file:
```
use std::fs::File;
use std::io::Write;
let mut file = File::create("examples/example.png").unwrap();
file.write_all(&png.signature).unwrap();
file.write_all(&png.data).unwrap();
```

Run the program with `cargo run` from the root folder and resulting image in examples/ should be:
![Example PNG](/examples/example.png)
