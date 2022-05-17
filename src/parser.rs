use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::cli::Cli;

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug)]
pub struct Png {
    metadata: [u8; 8],
    chunks: Vec<Chunk>,
}

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-layout
#[derive(Debug)]
struct Chunk {
    chunk_type: ChunkType,
    chunk_length: usize,
    chunk_data: Vec<u8>,
    chunk_crc: [u8; 4],
}

// Chunk Types
// http://www.libpng.org/pub/png/spec/1.2/PNG-Chunks.html
// Used to tell the parser what the data is used for.
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum ChunkType {
    IHDR,
    PLTE,
    IDAT,
    IEND,

    tRNS,
    gAMA,
    cHRM,
    sRGB,
    iCCP,

    tEXt,
    zTXt,
    iTXt,

    bKGD,
    pHYs,
    sBIT,
    sPLT,
    hIST,
    tIME,
}

impl From<Cli> for Png {
    // Iterate over the whole file, emmiting PNG at end.
    fn from(cli: Cli) -> Png {
        let path = match cli.file_path {
            Some(file_path) => file_path,
            None => {
                panic!("Error, can't read path from CLI");
            }
        };

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(..) => {
                panic!("Error, can't read file");
            }
        };

        let mut png_metadata: [u8; 8] = [0; 8];
        match file.read_exact(&mut png_metadata) {
            Ok(x) => x,
            Err(..) => {
                panic!("Error, PNG signature not found")
            }
        }

        if !(png_metadata == [137, 80, 78, 71, 13, 10, 26, 10]) {
            panic!("Error, the PNG File Signature is incorrect")
        }

        println!("{:?}", png_metadata);

        let mut png = Png {
            metadata: png_metadata,
            chunks: Vec::new(),
        };

        let mut chunk_type: [u8; 4] = [0; 4];
        let mut chunk_length: [u8; 4] = [0; 4];
        // let mut chunk_data: Vec<u8> = Vec::new(); - Length created at run-time, so we put it in the loop
        let mut chunk_crc: [u8; 4] = [0; 4];

        // TODO: Better Error Management
        loop {
            match file.read_exact(&mut chunk_length) {
                Err(..) => (),
                _ => (),
            }

            // PNG Uses Big-Edian
            // https://www.w3.org/TR/2003/REC-PNG-20031110/#7Integers-and-byte-order
            let chunk_length_usize = u32::from_be_bytes(chunk_length).try_into().unwrap();

            match file.read_exact(&mut chunk_type) {
                Err(..) => break,
                _ => (),
            }
            let mut chunk_data = vec![0; chunk_length_usize];

            match file.read_exact(&mut chunk_data) {
                Err(..) => (),
                _ => (),
            }

            match file.read_exact(&mut chunk_crc) {
                Err(..) => (),
                _ => (),
            }

            png.chunks.push(Chunk {
                chunk_type: ChunkType::from(chunk_type),
                chunk_length: chunk_length_usize,
                chunk_data,
                chunk_crc,
            });
        }

        return png;
    }
}

impl From<[u8; 4]> for ChunkType {
    // Identify & Parse the chunktype
    fn from(chunk_identifier: [u8; 4]) -> ChunkType {
        match chunk_identifier {
            [73, 69, 78, 68] => ChunkType::IEND,
            [73, 68, 65, 84] => ChunkType::IDAT,
            [73, 72, 68, 82] => ChunkType::IHDR,
            [80, 76, 84, 69] => ChunkType::PLTE,

            [116, 82, 78, 83] => ChunkType::tRNS,
            [103, 65, 77, 65] => ChunkType::gAMA,
            [99, 72, 82, 77] => ChunkType::cHRM,
            [115, 82, 71, 66] => ChunkType::sRGB,
            [98, 75, 71, 68] => ChunkType::bKGD,
            [112, 72, 89, 115] => ChunkType::pHYs,
            [115, 66, 73, 84] => ChunkType::sBIT,
            [115, 80, 76, 84] => ChunkType::sPLT,
            [104, 73, 83, 84] => ChunkType::hIST,
            [116, 73, 77, 69] => ChunkType::tIME,

            _ => {
                println!("Unkown Chunk Type");
                ChunkType::tIME
            }
        }
    }
}

// impl Png {
//     pub fn from_cli(input: Cli) -> () {
//         let path = match input.path {
//             Some(input_path) => input_path,
//             None => {
//                 panic!("File Path Not Given");
//             }
//         };

//         let output = match input.output {
//             Some(input_output) => input_output,
//             None => {
//                 panic!("Output Type Not Given");
//             }
//         };

//         let mut f = File::open(path).unwrap();

//         let mut metadata: [u8; 8] = [0; 8];
//         let mut color_type: u8 = 0;

//         f.read_exact(&mut metadata).expect("error");

//         println!("PNG METADATA: {:?} | Magic Numbers", metadata);
//         loop {
//             let mut chunk_length: [u8; 4] = [0; 4];
//             match f.read_exact(&mut chunk_length) {
//                 Err(..) => break,
//                 _ => (),
//             };

//             let mut chunk_type: [u8; 4] = [0; 4];
//             f.read_exact(&mut chunk_type).expect("error");

//             let mut chunk_data = vec![0; u32::from_be_bytes(chunk_length).try_into().unwrap()];
//             f.read_exact(&mut chunk_data).expect("error");

//             let mut crc: [u8; 4] = [0; 4];
//             f.read_exact(&mut crc).expect("error");

//             match ChunkTypes::from_bytes(chunk_type) {
//                 ChunkTypes::IEND => {
//                     println!("\nIEND CHUNK: \n (NO DATA)");
//                 }
//                 ChunkTypes::IDAT => {
//                     println!("\nIDAT CHUNK:");
//                     Color::from_color_type(color_type.clone(), chunk_data);
//                 }
//                 ChunkTypes::IHDR => {
//                     let mut data = chunk_data.as_slice();

//                     let mut byte_width: [u8; 4] = [0; 4];
//                     data.read_exact(&mut byte_width).unwrap();

//                     let mut byte_height: [u8; 4] = [0; 4];
//                     data.read_exact(&mut byte_height).unwrap();

//                     let mut byte_bit_depth: [u8; 1] = [0; 1];
//                     data.read_exact(&mut byte_bit_depth).unwrap();

//                     let mut byte_color_type: [u8; 1] = [0; 1];
//                     data.read_exact(&mut byte_color_type).unwrap();

//                     let mut byte_compression: [u8; 1] = [0; 1];
//                     data.read_exact(&mut byte_compression).unwrap();

//                     let mut byte_filter_method: [u8; 1] = [0; 1];
//                     data.read_exact(&mut byte_filter_method).unwrap();

//                     let mut byte_interlase_method: [u8; 1] = [0; 1];
//                     data.read_exact(&mut byte_interlase_method).unwrap();

//                     let width: usize = u32::from_be_bytes(byte_width).try_into().unwrap();
//                     let height: usize = u32::from_be_bytes(byte_height).try_into().unwrap();
//                     let bit_depth: usize = u8::from_be_bytes(byte_bit_depth).try_into().unwrap();
//                     color_type = u8::from_be_bytes(byte_color_type).try_into().unwrap();
//                     let compression: usize =
//                         u8::from_be_bytes(byte_compression).try_into().unwrap();
//                     let filter_method: usize =
//                         u8::from_be_bytes(byte_filter_method).try_into().unwrap();
//                     let interlase_method: usize =
//                         u8::from_be_bytes(byte_interlase_method).try_into().unwrap();

//                     if output == Output::Descriptive {
//                         println!("\nIHDR CHUNK: \n WIDTH:           {} | Width in Pixels\n HEIGHT:          {} | Height in Pixels\n BIT_DEPTH        {} | Bits Per Sample / Bits per pallet index\n COLOR_TYPE       {} | Allowed Bit Depths. 0 = Greyscale; 2 = RGB; 3 = Pallet Index; 4 = Greyscale + Alpha; 6 = RGB + Alpha\n COMPRESSION      {} | Only \"0\" is defined.\n FILTER_METHOD    {} | Any preprossessing applied before compression. \n INTERLASE_METHOD {} | Type of interlasing. 0 = None; 1 = Adam7",
//                                  &width, &height, &bit_depth, &color_type, &compression, &filter_method, &interlase_method);
//                     } else {
//                         println!("\nIHDR CHUNK: \n WIDTH: {}\n HEIGHT: {}\n BIT_DEPTH {}\n COLOR_TYPE {}\n COMPRESSION {}\n FILTER_METHOD {}\n INTERLASE_METHOD {}",
//                                  &width, &height, &bit_depth, &color_type, &compression, &filter_method, &interlase_method);
//                     }
//                 }
//                 ChunkTypes::PLTE => {
//                     if output == Output::Descriptive {
//                         println!("\nPLTE CHUNK: \n PALLET_DATA: {:?} | This is an indexed list of 255 colors in the image", chunk_data);
//                     } else {
//                         println!("\nPLTE CHUNK: \n PALLET_DATA: {:?}", chunk_data);
//                     }
//                 }
//                 ChunkTypes::tRNS => {
//                     if output == Output::Descriptive {
//                         println!("\ntRNS CHUNK: \n ALPHA_DATA: {:?} | This is an indexed list holding each alpha channel for all 255 colors in PLTE, but only includes alpha", chunk_data);
//                     } else {
//                         println!("\ntRNS CHUNK: \n ALPHA_DATA: {:?}", chunk_data);
//                     }
//                 }
//                 ChunkTypes::gAMA => {
//                     let mut byte_data: [u8; 4] = [0; 4];
//                     chunk_data.as_slice().read_exact(&mut byte_data).unwrap();

//                     if output == Output::Descriptive {
//                         println!(
//                             "\ngAMA CHUNK: \n GAMMA: {} | Holds gamma times 10,000",
//                             u32::from_be_bytes(byte_data)
//                         );
//                     } else {
//                         println!("\ngAMA CHUNK: \n GAMMA: {}", u32::from_be_bytes(byte_data));
//                     }
//                 }
//                 ChunkTypes::cHRM => {
//                     let mut data = chunk_data.as_slice();

//                     let mut whitex: [u8; 4] = [0; 4];
//                     data.read_exact(&mut whitex).unwrap();

//                     let mut whitey: [u8; 4] = [0; 4];
//                     data.read_exact(&mut whitey).unwrap();

//                     let mut redx: [u8; 4] = [0; 4];
//                     data.read_exact(&mut redx).unwrap();

//                     let mut redy: [u8; 4] = [0; 4];
//                     data.read_exact(&mut redy).unwrap();

//                     let mut greenx: [u8; 4] = [0; 4];
//                     data.read_exact(&mut greenx).unwrap();

//                     let mut greeny: [u8; 4] = [0; 4];
//                     data.read_exact(&mut greeny).unwrap();

//                     let mut bluex: [u8; 4] = [0; 4];
//                     data.read_exact(&mut bluex).unwrap();

//                     let mut bluey: [u8; 4] = [0; 4];
//                     data.read_exact(&mut bluey).unwrap();

//                     if output == Output::Descriptive {
//                         println!("\ncHRM CHUNK: \n WHITE POINT ({}, {}) | The point which should always be displayed as white \n RED ({}, {}) | Red chromaticy \n GREEN ({}, {}) | Green chromaticy \n BLUE ({}, {}) | Blue chromaticy", u32::from_be_bytes(whitex), u32::from_be_bytes(whitey), u32::from_be_bytes(redx), u32::from_be_bytes(redy), u32::from_be_bytes(greenx), u32::from_be_bytes(greeny), u32::from_be_bytes(bluex),u32::from_be_bytes(bluey));
//                     } else {
//                         println!("\ncHRM CHUNK: \n WHITE POINT ({}, {}) \n RED ({}, {}) \n GREEN ({}, {}) \n BLUE ({}, {})", u32::from_be_bytes(whitex), u32::from_be_bytes(whitey), u32::from_be_bytes(redx), u32::from_be_bytes(redy), u32::from_be_bytes(greenx), u32::from_be_bytes(greeny), u32::from_be_bytes(bluex),u32::from_be_bytes(bluey));
//                     }
//                 }
//                 ChunkTypes::sRGB => {
//                     let mut srgb: [u8; 1] = [0; 1];
//                     chunk_data.as_slice().read_exact(&mut srgb).unwrap();

//                     if output == Output::Descriptive {
//                         println!("\nsRGB CHUNK: \n RENDERING INTENT: {} | Modes for sRGB, 0=Perseptual;1=Relative;2=Saturation;3=Absolute", u8::from_be_bytes(srgb));
//                     } else {
//                         println!(
//                             "\nsRGB CHUNK: \n RENDERING INTENT: {}",
//                             u8::from_be_bytes(srgb)
//                         );
//                     }
//                 }

//                 ChunkTypes::bKGD => match color_type.clone() {
//                     0 | 4 => {
//                         let mut grey: [u8; 2] = [0; 2];
//                         chunk_data.as_slice().read_exact(&mut grey).unwrap();

//                         println!(
//                             "\nbKGD CHUNK: \n GREYSCALE_BACKGROUND {}",
//                             u16::from_be_bytes(grey)
//                         );
//                     }
//                     2 | 6 => {
//                         let mut data = chunk_data.as_slice();

//                         let mut red: [u8; 2] = [0; 2];
//                         data.read_exact(&mut red).unwrap();

//                         let mut blue: [u8; 2] = [0; 2];
//                         data.read_exact(&mut blue).unwrap();

//                         let mut green: [u8; 2] = [0; 2];
//                         data.read_exact(&mut green).unwrap();

//                         println!(
//                             "\nbKGD CHUNK: \n RGB_BACKGROUND ({}, {} , {})",
//                             u16::from_be_bytes(red),
//                             u16::from_be_bytes(green),
//                             u16::from_be_bytes(blue)
//                         );
//                     }
//                     3 => {
//                         let mut index: [u8; 1] = [0; 1];
//                         chunk_data.as_slice().read_exact(&mut index).unwrap();

//                         println!(
//                             "\nbKGD CHUNK: \n PALLET_BACKGROUND {}",
//                             u8::from_be_bytes(index)
//                         );
//                     }
//                     _ => {}
//                 },
//                 ChunkTypes::pHYs => {
//                     let mut data = chunk_data.as_slice();

//                     let mut sizex: [u8; 4] = [0; 4];
//                     data.read_exact(&mut sizex).unwrap();

//                     let mut sizey: [u8; 4] = [0; 4];
//                     data.read_exact(&mut sizey).unwrap();

//                     let mut unit: [u8; 1] = [0; 1];
//                     data.read_exact(&mut unit).unwrap();

//                     if output == Output::Descriptive {
//                         println!("\npHYs CHUNK: \n PIXEL_PER_UNIT_X {} \n PIXEL_PER_UNIT_Y {} \n UNIT {} | 0=Unknown;1=Meter", u32::from_be_bytes(sizex), u32::from_be_bytes(sizey), u8::from_be_bytes(unit));
//                     } else {
//                         println!("\npHYs CHUNK: \n PIXEL_PER_UNIT_X {} \n PIXEL_PER_UNIT_Y {} \n UNIT {}", u32::from_be_bytes(sizex), u32::from_be_bytes(sizey), u8::from_be_bytes(unit));
//                     }
//                 }
//                 ChunkTypes::sBIT => match color_type.clone() {
//                     0 => {
//                         let mut sig: [u8; 1] = [0; 1];
//                         chunk_data.as_slice().read_exact(&mut sig).unwrap();

//                         println!(
//                             "\nsBIT CHUNK: \n SIGNIFICANT_BITS: {}",
//                             u8::from_be_bytes(sig)
//                         );
//                     }
//                     2 => {
//                         let mut data = chunk_data.as_slice();

//                         let mut red: [u8; 1] = [0; 1];
//                         data.read_exact(&mut red).unwrap();

//                         let mut blue: [u8; 1] = [0; 1];
//                         data.read_exact(&mut blue).unwrap();

//                         let mut green: [u8; 1] = [0; 1];
//                         data.read_exact(&mut green).unwrap();

//                         println!(
//                             "\nsBIT CHUNK: \n SIGNIFICANT_BITS: ({}, {}, {})",
//                             u8::from_be_bytes(red),
//                             u8::from_be_bytes(blue),
//                             u8::from_be_bytes(green)
//                         );
//                     }
//                     4 => {
//                         let mut data = chunk_data.as_slice();

//                         let mut grey: [u8; 1] = [0; 1];
//                         data.read_exact(&mut grey).unwrap();

//                         let mut alpha: [u8; 1] = [0; 1];
//                         data.read_exact(&mut alpha).unwrap();

//                         let mut green: [u8; 1] = [0; 1];
//                         data.read_exact(&mut green).unwrap();

//                         println!(
//                             "\nsBIT CHUNK: \n SIGNIFICANT_BITS: ({}, {})",
//                             u8::from_be_bytes(grey),
//                             u8::from_be_bytes(alpha)
//                         );
//                     }
//                     6 => {
//                         let mut data = chunk_data.as_slice();

//                         let mut red: [u8; 1] = [0; 1];
//                         data.read_exact(&mut red).unwrap();

//                         let mut blue: [u8; 1] = [0; 1];
//                         data.read_exact(&mut blue).unwrap();

//                         let mut green: [u8; 1] = [0; 1];
//                         data.read_exact(&mut green).unwrap();

//                         let mut alpha: [u8; 1] = [0; 1];
//                         data.read_exact(&mut alpha).unwrap();

//                         println!(
//                             "\nsBIT CHUNK: \n SIGNIFICANT_BITS: ({}, {}, {}, {})",
//                             u8::from_be_bytes(red),
//                             u8::from_be_bytes(blue),
//                             u8::from_be_bytes(green),
//                             u8::from_be_bytes(alpha)
//                         );
//                     }

//                     _ => {}
//                 },
//                 ChunkTypes::tIME => {
//                     let mut data = chunk_data.as_slice();

//                     let mut year: [u8; 2] = [0; 2];
//                     data.read_exact(&mut year).unwrap();

//                     let mut month: [u8; 1] = [0; 1];
//                     data.read_exact(&mut month).unwrap();

//                     let mut day: [u8; 1] = [0; 1];
//                     data.read_exact(&mut day).unwrap();

//                     let mut hour: [u8; 1] = [0; 1];
//                     data.read_exact(&mut hour).unwrap();

//                     let mut minute: [u8; 1] = [0; 1];
//                     data.read_exact(&mut minute).unwrap();

//                     let mut second: [u8; 1] = [0; 1];
//                     data.read_exact(&mut second).unwrap();

//                     println!("\ntIME CHUNK: \n YEAR: {} \n MONTH: {} \n DAY: {} \n HOUR: {} \n MINUTE: {} \n SECOND: {}", u16::from_be_bytes(year), u8::from_be_bytes(month), u8::from_be_bytes(day), u8::from_be_bytes(hour), u8::from_be_bytes(minute), u8::from_be_bytes(second));
//                 }
//                 _ => (),
//             }
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// #[allow(non_camel_case_types)]
// #[derive(PartialEq)]
// enum ChunkTypes {
//     IHDR,
//     PLTE,
//     IDAT,
//     IEND,

//     tRNS,
//     gAMA,
//     cHRM,
//     sRGB,
//     bKGD,
//     pHYs,
//     sBIT,
//     sPLT,
//     hIST,
//     tIME,

//     Other,
// }

// impl Color {
//     pub fn from_color_type(color_type: u8, input: Vec<u8>) -> () {
//         match color_type {
//             0 => {
//                 let mut color_vec: Vec<(u8)> = Vec::new();

//                 for x in input {
//                     color_vec.push((x));
//                 }

//                 println!(" GREYSCALE_DATA: {:?}", color_vec);
//             }
//             2 => {
//                 let mut color_vec: Vec<(u8, u8, u8)> = Vec::new();

//                 let mut data = input.as_slice();

//                 loop {
//                     let mut byte_red: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_red) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let mut byte_green: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_green) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let mut byte_blue: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_blue) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let red = u8::from_be_bytes(byte_red);
//                     let green = u8::from_be_bytes(byte_green);
//                     let blue = u8::from_be_bytes(byte_blue);

//                     color_vec.push((red, green, blue))
//                 }

//                 println!(" RGB_DATA: {:?}", color_vec);
//             }
//             3 => {
//                 let mut color_vec: Vec<(u8)> = Vec::new();

//                 for x in input {
//                     color_vec.push((x));
//                 }

//                 println!(" PALLET_INDEXES: {:?}", color_vec);
//             }
//             4 => {
//                 let mut color_vec: Vec<(u8, u8)> = Vec::new();
//                 let mut data = input.as_slice();

//                 loop {
//                     let mut byte_grey: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_grey) {
//                         Err(..) => break (),
//                         _ => (),
//                     }

//                     let mut byte_alpha: [u8; 1] = [0; 1];
//                     data.read_exact(&mut byte_alpha).unwrap();

//                     color_vec.push((u8::from_be_bytes(byte_grey), u8::from_be_bytes(byte_alpha)));
//                 }

//                 println!(" GREYSCALE+ALPHA_DATA: {:?}", color_vec);
//             }
//             6 => {
//                 let mut color_vec: Vec<(u8, u8, u8, u8)> = Vec::new();
//                 let mut data = input.as_slice();

//                 loop {
//                     let mut byte_red: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_red) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let mut byte_green: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_green) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let mut byte_blue: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_blue) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let mut byte_alpha: [u8; 1] = [0; 1];
//                     match data.read_exact(&mut byte_alpha) {
//                         Err(..) => break (),
//                         _ => (),
//                     };

//                     let red = u8::from_be_bytes(byte_red);
//                     let green = u8::from_be_bytes(byte_green);
//                     let blue = u8::from_be_bytes(byte_blue);
//                     let alpha = u8::from_be_bytes(byte_alpha);

//                     color_vec.push((red, green, blue, alpha))
//                 }
//                 println!(" RGB+ALPHA_DATA: {:?}", color_vec);
//             }
//             _ => panic!(),
//         }
//     }
// }

// impl ChunkTypes {
//     pub fn from_bytes(input: [u8; 4]) -> ChunkTypes {
//         match input {
// [73, 69, 78, 68] => ChunkTypes::IEND,
// [73, 68, 65, 84] => ChunkTypes::IDAT,
// [73, 72, 68, 82] => ChunkTypes::IHDR,
// [80, 76, 84, 69] => ChunkTypes::PLTE,

// [116, 82, 78, 83] => ChunkTypes::tRNS,
// [103, 65, 77, 65] => ChunkTypes::gAMA,
// [99, 72, 82, 77] => ChunkTypes::cHRM,
// [115, 82, 71, 66] => ChunkTypes::sRGB,
// [98, 75, 71, 68] => ChunkTypes::bKGD,
// [112, 72, 89, 115] => ChunkTypes::pHYs,
// [115, 66, 73, 84] => ChunkTypes::sBIT,
// [115, 80, 76, 84] => ChunkTypes::sPLT,
// [104, 73, 83, 84] => ChunkTypes::hIST,
// [116, 73, 77, 69] => ChunkTypes::tIME,
// _ => ChunkTypes::Other,
//         }
//     }
// }
