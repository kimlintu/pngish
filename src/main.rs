use std::error::Error;

struct PngImage {
    signature: [u8; 8],
    data: Vec<u8>,
}

struct PngChunk {
    data_len: [u8; 4],
    chunk_type: [u8; 4],
    chunk_data: Vec<u8>,
    crc: u32,
}
impl From<PngChunk> for Vec<u8> {
    fn from(mut chunk: PngChunk) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(std::mem::size_of::<PngChunk>());
        v.extend_from_slice(&chunk.data_len);
        v.extend_from_slice(&chunk.chunk_type);
        v.append(&mut chunk.chunk_data);
        v.extend_from_slice(&chunk.crc.to_be_bytes());

        v
    }
}

struct IHDRChunk {
    width: [u8; 4],
    height: [u8; 4],
    bit_depth: u8,
    color_type: u8,
    compress_method: u8,
    filter_method: u8,
    interlace_method: u8,
}
impl From<IHDRChunk> for Vec<u8> {
    fn from(chunk: IHDRChunk) -> Vec<u8> {
        let mut v = Vec::with_capacity(std::mem::size_of::<IHDRChunk>());
        v.extend_from_slice(&chunk.width);
        v.extend_from_slice(&chunk.height);
        v.push(chunk.bit_depth); 
        v.push(chunk.color_type);
        v.push(chunk.compress_method);
        v.push(chunk.filter_method);
        v.push(chunk.interlace_method);

        v
    }
}

impl From<TestPainting> for Vec<u8> {
    fn from(painting: TestPainting) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let ihdr = IHDRChunk {
            width: painting.width.to_be_bytes(),
            height: painting.height.to_be_bytes(),
            bit_depth: 8,
            color_type: 2,
            compress_method: 0,
            filter_method: 0,
            interlace_method: 0,
        };
        let ihdr_bytes = Vec::from(ihdr);

        let ihdr_chunk = PngChunk {
            data_len: u32::try_from(ihdr_bytes.len()).unwrap().to_be_bytes(),
            chunk_type: [b'I', b'H', b'D', b'R'],
            chunk_data: ihdr_bytes,
            crc: 3 // TODO: calculate crc
        };
        bytes.extend_from_slice(&Vec::from(ihdr_chunk));

        let mut pixels: Vec<u8> = Vec::new();
        let nr_of_pixels: usize = (painting.width * painting.height).try_into().unwrap();

        for i in 0..nr_of_pixels {
            match (i % usize::try_from(painting.width).unwrap()) {
                0 => {
                    pixels.push(0);
                    pixels.push(painting.pixels[i].red);
                    pixels.push(painting.pixels[i].green);
                    pixels.push(painting.pixels[i].blue);
                },
                _ => {
                    pixels.push(painting.pixels[i].red);
                    pixels.push(painting.pixels[i].green);
                    pixels.push(painting.pixels[i].blue);
                }
            }
        }
        use miniz_oxide::deflate;
        let mut compressed = deflate::compress_to_vec(&pixels, 6);

        let idat_chunk = PngChunk {
            data_len: u32::try_from(compressed.len()).unwrap().to_be_bytes(),
            chunk_type: [b'I', b'D', b'A', b'T'],
            chunk_data: compressed,
            crc: 3 // TODO: calculate crc
        };

        bytes.extend_from_slice(&Vec::from(idat_chunk));

        let iend_chunk = PngChunk {
            data_len: [0, 0, 0, 0],
            chunk_type: [b'I', b'E', b'N', b'D'],
            chunk_data: Vec::new(),
            crc: 3 // TODO: calculate crc
        };
        bytes.extend_from_slice(&Vec::from(iend_chunk));

        bytes
    }
}

/*
   impl From<TestPainting> for Vec<PngChunk> {
   fn from(painting: TestPainting) -> Vec<PngChunk> {
   let mut chunks: Vec<PngChunk> = Vec::new();

   let ihdr_chunk = PngChunk {
   data_len: u32::try_from(std::mem::size_of::<IHDRChunk>()).unwrap(), // data_len = nr. of bytes in chunk_data field
   chunk_type: [b'R', b'D', b'H', b'I'],
   chunk_data: ihdr,
   crc: 3,
   };
   chunks.push(ihdr_chunk);

// encode and push all IDAT chunks
// here we process the painting data (pixels)
// - divide the data into chunks (might not be necessary, first iteration we will just have
// one big chunk)
// - compress the data with some zlib library
// - the IDAT chunk now contains the compressed data
// - push the chunk
//
use miniz_oxide::deflate;
use miniz_oxide::inflate;

let mut bytes: Vec<u8> = Vec::new();
let nr_of_pixels: usize = (painting.width * painting.height).try_into().unwrap();
for i in 0..nr_of_pixels {
bytes.push(painting.pixels[i].blue);
bytes.push(painting.pixels[i].green);
bytes.push(painting.pixels[i].red);
}
let compressed = deflate::compress_to_vec(&bytes, 3);
let idat = IDATChunk {
data: compressed,
};
let idat_chunk = PngChunk {
data_len: u32::try_from(idat.data.len()).unwrap(), // data_len = nr. of bytes in chunk_data field
chunk_type: [b'T', b'A', b'D', b'I'],
chunk_data: ChunkData::IDAT(idat),
crc: 3,
};
chunks.push(idat_chunk);

// push IEND chunk
let iend_chunk = PngChunk {
data_len: 0,
chunk_type: [b'D', b'N', b'E', b'I'],
chunk_data: ChunkData::IEND(IENDChunk {}),
crc: 3,
};
chunks.push(iend_chunk);

chunks
}
}
*/

impl From<TestPainting> for PngImage {
    fn from(painting: TestPainting) -> PngImage {
        let data: Vec<u8> = Vec::from(painting);

        let try1 = [0x0A, 0x1A, 0x0A, 0x0D, 0x47, 0x4E, 0x50, 0x89];
        let try2 = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        PngImage { 
            signature: try2,
            data,
        }
    }
}

#[derive(Debug)]
struct TestPixel {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct TestPainting {
    pixels: Vec<TestPixel>,
    width: u32,
    height: u32,
}

trait Chunk {}

impl Chunk for IHDRChunk {}

struct IDATChunk {}
impl Chunk for IDATChunk {}

struct IENDChunk {}
impl Chunk for IENDChunk {}

fn main() {
    let side_len: u32 = 256;

    let mut painting = TestPainting { 
        pixels: Vec::new(),
        width: side_len,
        height: side_len,
    };

    for y in 0..side_len {
        for x in 0..side_len {
            let pixel = TestPixel { red: 255, green: x as u8, blue: y as u8 };
            painting.pixels.push(pixel);
        }
    }

    let png = PngImage::from(painting);
    //println!("{:?}", png.data);

    // write png bytes to a file

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("test.png").unwrap();
    file.write_all(&png.signature).unwrap();
    file.write_all(&png.data).unwrap();

    println!("hello");
}
