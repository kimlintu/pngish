/// Structure holding the PNG image data.
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
        let ihdr_bytes = Vec::from(ihdr);

        let ihdr_chunk = PngChunk {
            data_len: u32::try_from(ihdr_bytes.len()).unwrap().to_be_bytes(),
            chunk_type: [b'I', b'H', b'D', b'R'],
            chunk_data: ihdr_bytes,
            crc: 3 // TODO: calculate crc
        };
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

        let idat_chunk = PngChunk {
            data_len: u32::try_from(compressed_img_data.len()).unwrap().to_be_bytes(),
            chunk_type: [b'I', b'D', b'A', b'T'],
            chunk_data: compressed_img_data,
            crc: 3 // TODO: calculate crc
        };
        picture_bytes.extend_from_slice(&Vec::from(idat_chunk));

        // The IEND chunk marks EOF
        let iend_chunk = PngChunk {
            data_len: [0, 0, 0, 0],
            chunk_type: [b'I', b'E', b'N', b'D'],
            chunk_data: Vec::new(),
            crc: 3 // TODO: calculate crc
        };
        picture_bytes.extend_from_slice(&Vec::from(iend_chunk));

        picture_bytes
    }
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
