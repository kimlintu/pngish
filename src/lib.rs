// Source: http://www.libpng.org/pub/png/spec/1.2/PNG-CRCAppendix.html
fn gen_crc_table() -> Vec<u32> {
    let mut table = Vec::with_capacity(256);

    let what: u32 = 0xEDB88320;
    let mut c: u32;

    for n in 0..256 {
        c = u32::try_from(n).unwrap();
        for _ in 0..8 {
            match c & 1 {
                1 => {
                    c = what ^ (c >> 1);
                }
                _ => {
                    c = c >> 1;
                }
            }
        }
        table.push(c);
    }

    table
}

// Source: http://www.libpng.org/pub/png/spec/1.2/PNG-CRCAppendix.html
fn gen_crc(crc: u32, buf: &[u8]) -> u32 {
    let mut c = crc;

    let table = gen_crc_table();

    for n in 0..buf.len() {
        c = table[usize::try_from((c ^ u32::try_from(buf[n]).unwrap()) & 0xFF).unwrap()] ^ (c >> 8);
    }

    c ^ 0xFFFFFFFF
}

pub struct PngImage {
    pub signature: [u8; 8],
    pub data: Vec<u8>,
}

impl PngImage {
    /// Creates a PngImage holding the PNG image data based on the data provided.
    pub fn new(picture: &Picture) -> PngImage {
        PngImage { 
            signature: [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            data: Vec::from(picture),
        }
    }
}

impl From<&Picture> for Vec<u8> {
    fn from(picture: &Picture) -> Vec<u8> {
        let mut picture_bytes: Vec<u8> = Vec::new();

        // IHDR chunk always come first
        let ihdr = IHDRChunk {
            width: picture.width.to_be_bytes(),
            height: picture.height.to_be_bytes(),
            bit_depth: 8,
            color_type: 2,
            compress_method: 0,
            filter_method: 0,
            interlace_method: 0,
        };
        let ihdr_chunk = PngChunk::new_as_bytes(String::from("IHDR"), Vec::from(ihdr));
        picture_bytes.extend_from_slice(&Vec::from(ihdr_chunk));

        // Our IDAT chunks that will contain compressed image data
        let mut pixels: Vec<u8> = Vec::with_capacity(usize::try_from(picture.height * (1 + picture.width)).unwrap());
        let mut pixel_i = 0;
        for _ in 0..picture.height {
            // Each scanline is preceeded by a 'filter type' byte. 
            // Currently we just use 0, which means no filter is applied.
            pixels.push(0);

            for _ in 0..picture.width {
                pixels.push(picture.pixels[pixel_i].red);
                pixels.push(picture.pixels[pixel_i].green);
                pixels.push(picture.pixels[pixel_i].blue);
                pixel_i += 1;
            }
        }
        let compressed_img_data = miniz_oxide::deflate::compress_to_vec(&pixels, 6);
        let idat_chunk = PngChunk::new_as_bytes(String::from("IDAT"), compressed_img_data);
        picture_bytes.extend_from_slice(&idat_chunk);

        // The IEND chunk marks the EOF
        let iend_chunk = PngChunk::new_as_bytes(String::from("IEND"), Vec::new()); 
        picture_bytes.extend_from_slice(&iend_chunk);

        picture_bytes
    }
}

struct PngChunk {
    data_len: [u8; 4],
    chunk_type: [u8; 4],
    chunk_data: Vec<u8>,
    crc: u32,
}

impl PngChunk {
    fn new_as_bytes(chunk_type: String, mut chunk_data: Vec<u8>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let data_len = chunk_data.len();
        bytes.extend_from_slice(&u32::try_from(data_len).unwrap().to_be_bytes());

        let s = &chunk_type;
        let chunk_type = chunk_type.as_bytes();
        bytes.extend_from_slice(chunk_type);

        bytes.append(&mut chunk_data);

        // The CRC is calculated based on the previous data in the chunk, excluding the data length
        // (first 4 bytes).
        let crc = gen_crc(0xFFFFFFFF, &bytes[4..bytes.len()]);
        println!("crc for {} is {:#x}", s, crc);
        bytes.extend_from_slice(&u32::try_from(crc).unwrap().to_be_bytes());

        bytes
    }
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

/// The original picture data that is to be transformed into a PNG image.
pub struct Picture {
    pub pixels: Vec<RGBPixel>,
    pub width: u32, 
    pub height: u32, 
}

/// A pixel with RGB fields.
pub struct RGBPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
