// TODO: Might make sense to make structs for each chunk type, allowing for me to more easily share & display the data when parsing

use std::fs::File;
use std::io::Read;

use crate::cli::Cli;

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
// https://www.w3.org/TR/2003/REC-PNG-20031110/#11Chunks
#[derive(Debug)]
pub struct Png {
    metadata: [u8; 8],
    chunks: Vec<Chunk>,
    cli: Cli,
}

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-layout
#[derive(Debug)]
struct Chunk {
    chunk_type: ChunkType,
    chunk_length: usize,
    chunk_data: Vec<u8>,
    chunk_crc: [u8; 4],
}

// Gonna be a disgusting mess imo
impl std::fmt::Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // What to parse before hand: We must know what the colordata in the IHDR chunk is to allow us to understand what the IDAT Chunk actually holds as data.
        // Otherwise, we don't know if it's gresycale, alpha, 3 channel, etc.
        // Also tells us if it's pallet or not, so we don't need to go looking beforehand.

        let mut color_type: [u8; 1] = [0; 1];
        let mut bit_depth: [u8; 1] = [0; 1];

        for chunk in &self.chunks {
            match chunk.chunk_type {
                ChunkType::IHDR => {
                    let mut width: [u8; 4] = [0; 4];
                    let mut height: [u8; 4] = [0; 4];
                    let mut compression_method: [u8; 1] = [0; 1];
                    let mut filter_method: [u8; 1] = [0; 1];
                    let mut interlace_method: [u8; 1] = [0; 1];

                    chunk.chunk_data.as_slice().read_exact(&mut width).unwrap();
                    chunk.chunk_data.as_slice().read_exact(&mut height).unwrap();
                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut bit_depth)
                        .unwrap();
                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut color_type)
                        .unwrap();
                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut compression_method)
                        .unwrap();
                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut filter_method)
                        .unwrap();
                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut interlace_method)
                        .unwrap();

                    if self.cli.display_options.descriptive == Some(true) {
                        write!(f, "IHDR Chunk:\n width: {:?}\n height: {:?}\n compression method: {:?}\n filter_method {:?}\n interlace method: {:?}\n",
                            u32::from_be_bytes(width), u32::from_be_bytes(height), u8::from_be_bytes(compression_method), u8::from_be_bytes(filter_method), u8::from_be_bytes(interlace_method)).unwrap();
                    }
                }
                ChunkType::IEND => {
                    if self.cli.display_options.descriptive == Some(true) {
                        write!(f, "IEND Chunk:\n (no data)").unwrap();
                    }
                }
                ChunkType::PLTE => {
                    // Scheme: Index, Red, Green, Blue
                    let mut pallet: Vec<(u8, u8, u8)> = Vec::new();

                    for _ in chunk.chunk_data.iter() {
                        let mut colors: [u8; 3] = [0, 0, 0];

                        // Break once the Pallet Chunk ends
                        match chunk.chunk_data.as_slice().read_exact(&mut colors) {
                            Ok(..) => {}
                            Err(..) => {
                                break;
                            }
                        }

                        pallet.push((colors[0], colors[1], colors[2]));
                    }

                    if self.cli.display_options.descriptive == Some(true) {
                        write!(f, "PLTE Chunk:\n").unwrap();
                        for (index, colors) in pallet.iter().enumerate() {
                            write!(f, " {}: ", index).unwrap();
                            write!(f, "{:?}\n", colors).unwrap();
                        }
                    }
                }
                ChunkType::gAMA => {
                    let mut image_gama: [u8; 4] = [0; 4];

                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut image_gama)
                        .unwrap();

                    if self.cli.display_options.descriptive == Some(true) {
                        write!(f, "gAMA Chunk:\n gama: {}", u32::from_be_bytes(image_gama))
                            .unwrap();
                    }
                }
                ChunkType::sRGB => {
                    let mut rendering_intent: [u8; 1] = [0];

                    chunk
                        .chunk_data
                        .as_slice()
                        .read_exact(&mut rendering_intent)
                        .unwrap();

                    if self.cli.display_options.descriptive == Some(true) {
                        write!(
                            f,
                            "sRGB Chunk:\n rendering intent: {:?}",
                            u8::from_be_bytes(rendering_intent)
                        )
                        .unwrap();
                    }
                }
                ChunkType::tIME => {
                    let mut year: [u8; 2] = [0; 2];
                    let mut month: [u8; 1] = [0; 1];
                    let mut day: [u8; 1] = [0; 1];
                    let mut hour: [u8; 1] = [0; 1];
                    let mut minute: [u8; 1] = [0; 1];
                    let mut second: [u8; 1] = [0; 1];
                    chunk.chunk_data.as_slice().read_exact(&mut year).unwrap();
                    chunk.chunk_data.as_slice().read_exact(&mut month).unwrap();
                    chunk.chunk_data.as_slice().read_exact(&mut day).unwrap();
                    chunk.chunk_data.as_slice().read_exact(&mut hour).unwrap();
                    chunk.chunk_data.as_slice().read_exact(&mut minute).unwrap();
                    chunk.chunk_data.as_slice().read_exact(&mut second).unwrap();

                    if self.cli.display_options.descriptive == Some(true) {
                        write!(f,
                            "tIME Chunk:\n Year: {}\n Month: {}\n Day: {}\n Hour: {}\n Minute: {}\n Second: {}", 
                            u16::from_be_bytes(year), u8::from_be_bytes(month), u8::from_be_bytes(day), u8::from_be_bytes(hour), u8::from_be_bytes(minute), u8::from_be_bytes(second)
                        )
                        .unwrap();
                    }
                }
                _ => {
                    println!("{:?}", &chunk);
                }
            }
        }
        Ok(())
    }
}

trait Parse {
    fn parse(chunk: Chunk) -> Self;
}

// struct IHDR {
//     width: u32,
//     height: u32,
//     bit_depth: u8,
//     color_type: u8,
//     compression_method: u8,
//     filter_method: u8,
//     interlace_method: u8,
// }

// struct PLTE {
//     // R,G,B
//     pallet: Vec<(u8, u8, u8)>,
// }

// struct IDAT {
//     // Depends
// }

// // No Data
// struct IEND {}

// struct tRNS {
//     //Color type 0:
//     grey_sample_value: u16,

//     // Type 2:
//     red_sample_value: u16,
//     blue_sample_value: u16,
//     green_sample_value: u16,

//     // Type 3
//     pallet: Vec<u8>, // Type 4 & 6 are ignored as alpha is saved in IDAT
// }

// struct cHRM {
//     white_x: u32,
//     white_y: u32,

//     red_x: u32,
//     red_y: u32,

//     green_x: u32,
//     green_y: u32,

//     blue_x: u32,
//     blue_y: u32,
// }

// struct gAMA {
//     image_gama: u32,
// }

// struct iCCP {
//     profile_name: String, // Bytes 1-79

//     //null_separator: u8,
//     compression_method: u8,
//     compressed_profile: Vec<u8>,
// }

// struct sBIT {
//     // Type 0
//     significant_greyscale_bit: u8,

//     // Type 2 & 3
//     significant_red_bits: u8,
//     significant_green_bits: u8,
//     significant_blue_bits: u8,

//     significant_greyscale_bits: u8,
//     significant_alpha_bits: u8,

//     significant_red_bits: u8,
//     significant_green_bits: u8,
//     significant_blue_bits: u8,
//     significant_alpha_bits: u8,
// }

// struct sRGB {
//     rendering_intent: u8,
// }

// struct bKGD {
//     // Type 0
//     greyscale: u16,
//     // Type 2 & 6
//     red: u16,
//     green: u16,
//     blue: u16,
//     // Type 3
//     palette_index: u8,
// }

// struct hIST {
//     histogram: Vec<u16>,
// }

// struct pHYs {
//     pixels_per_x: u32,
//     pixels_per_y: u32,
//     unit_specifier: u8,
// }

// struct sPLT {
//     palette_name: String, // First 1-79 bytes
//     // null_seperator: u8,
//     sample_depth: u8,
//     red: u8, // Can be 16?
//     green: u8,
//     blue: u8,
//     alpha: u8,
//     frequency: u8,
// }

// struct tIME {
//     year: u16,
//     month: u8,
//     day: u8,
//     hour: u8,
//     minute: u8,
//     second: u8,
// }

// Chunk Types
// http://www.libpng.org/pub/png/spec/1.2/PNG-Chunks.html
// Used to tell the parser what the data is used for.
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum ChunkType {
    // Critical chunks
    // https://www.w3.org/TR/2003/REC-PNG-20031110/#11Critical-chunks
    IHDR, // 73, 72, 68, 62
    PLTE, // 80, 76, 84, 69
    IDAT, // 73, 68, 65, 84
    IEND, // 73, 69, 78, 68

    // Ancillary chunks
    // https://www.w3.org/TR/2003/REC-PNG-20031110/#11Ancillary-chunks
    tRNS, // 116, 82, 78, 83
    gAMA, // 103, 65, 77, 65
    cHRM, // 99, 72, 82, 77
    sRGB, // 115, 82, 71, 66
    iCCP, // 105, 67, 67, 80

    tEXt, // 116, 69, 88, 116
    zTXt, // 122, 84, 88, 116
    iTXt, // 105, 84, 88, 116

    bKGD, // 98, 75, 71, 68
    pHYs, // 112, 72, 89, 115
    sBIT, // 115, 66, 73, 84
    sPLT, // 115, 80, 76, 84
    hIST, // 104, 73, 83, 84
    tIME, // 116, 73, 77, 69
}

impl From<Cli> for Png {
    // Iterate over the whole file, emmiting PNG at end.
    fn from(cli: Cli) -> Png {
        let path = match cli.file_path.clone() {
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
            cli,
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
            [73, 72, 68, 82] => ChunkType::IHDR,
            [80, 76, 84, 69] => ChunkType::PLTE,
            [73, 68, 65, 84] => ChunkType::IDAT,
            [73, 69, 78, 68] => ChunkType::IEND,

            [116, 82, 78, 83] => ChunkType::tRNS,
            [103, 65, 77, 65] => ChunkType::gAMA,
            [99, 72, 82, 77] => ChunkType::cHRM,
            [115, 82, 71, 66] => ChunkType::sRGB,
            [105, 67, 67, 80] => ChunkType::iCCP,

            [116, 69, 88, 116] => ChunkType::tEXt,
            [122, 84, 88, 116] => ChunkType::zTXt,
            [105, 84, 88, 116] => ChunkType::iTXt,

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
