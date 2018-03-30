use std::fs::File;
use std::io::Read;
use std::env;
use std::string::String;
extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

fn drain_u32(buf: &mut Vec<u8>) -> u32 {
    let word: Vec<u8> = buf.drain(..4).collect();
    (&word[..]).read_u32::<BigEndian>().unwrap()
}

fn drain_u8(buf: &mut Vec<u8>) -> u8 {
    let byte = buf[0];
    buf.drain(..1);
    byte
}


struct PNG {
    header: [u8; 8],
    chunks: Vec<Chunk>
}

#[derive (Copy, Clone, Debug, PartialEq)]
enum ChunkType {
    IHDR(u32),
    PLTE(u32),
    IDAT(u32),
    IEND(u32),
    tRNS(u32),
    cHRM(u32),
    gAMA(u32),
    iCCP(u32),
    sBIT(u32),
    sRGB(u32),
    iTXt(u32),
    tEXt(u32),
    zTXt(u32),
    bKGD(u32),
    hIST(u32),
    pHYs(u32),
    sPLT(u32),
    tIME(u32),
}

impl ChunkType {
    fn to_string(tag: u32) -> String {
        let mut buf = Vec::new();
        buf.write_u32::<BigEndian>(tag).unwrap();
        String::from_utf8(buf).unwrap()
    }

    fn parse(tag: u32) -> Option<ChunkType> {
        match ChunkType::to_string(tag).as_ref() {
            "IHDR" => {
                Option::from(ChunkType::IHDR(tag))
            }
            "PLTE" => {
                Option::from(ChunkType::PLTE(tag))
            }
            "IDAT" => {
                Option::from(ChunkType::IDAT(tag))
            }
            "IEND" => {
                Option::from(ChunkType::IEND(tag))
            }
            "tRNS" => {
                Option::from(ChunkType::tRNS(tag))
            }
            "cHRM" => {
                Option::from(ChunkType::cHRM(tag))
            }
            "gAMA" => {
                Option::from(ChunkType::gAMA(tag))
            }
            "iCCP" => {
                Option::from(ChunkType::iCCP(tag))
            }
            "sBIT" => {
                Option::from(ChunkType::sBIT(tag))
            }
            "sRGB" => {
                Option::from(ChunkType::sRGB(tag))
            }
            "iTXt" => {
                Option::from(ChunkType::iTXt(tag))
            }
            "tEXt" => {
                Option::from(ChunkType::tEXt(tag))
            }
            "zTXt" => {
                Option::from(ChunkType::zTXt(tag))
            }
            "bKGD" => {
                Option::from(ChunkType::bKGD(tag))
            }
            "hIST" => {
                Option::from(ChunkType::hIST(tag))
            }
            "pHYs" => {
                Option::from(ChunkType::pHYs(tag))
            }
            "sPLT" => {
                Option::from(ChunkType::sPLT(tag))
            }
            "tIME" => {
                Option::from(ChunkType::tIME(tag))
            }
            _ => Option::None
        }
    }
}

struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

#[derive (Copy, Clone, Debug)]
struct DataIHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    colour_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl DataIHDR {
    fn from_buf(buf: &[u8]) -> DataIHDR {
        let mut tmp: Vec<u8> = buf.to_vec();
        let width = drain_u32(&mut tmp);
        let height = drain_u32(&mut tmp);
        let bit_depth: u8 = drain_u8(&mut tmp);
        let colour_type: u8 = drain_u8(&mut tmp);
        let compression_method: u8 = drain_u8(&mut tmp);
        let filter_method: u8 = drain_u8(&mut tmp);
        let interlace_method: u8 = drain_u8(&mut tmp);
        DataIHDR {
            width, height, bit_depth, colour_type, compression_method, filter_method, interlace_method
        }
    }
}

impl Chunk {
    fn from_buf(buf: &mut Vec<u8>) -> Chunk {
        let length = drain_u32(buf);
        let chunk_type = ChunkType::parse(drain_u32(buf)).unwrap();
        let chunk_data: Vec<u8> = buf.drain(..length as usize).collect();
        let crc = drain_u32(buf);
        println!("length: {}, chunk type: {:?}", length, chunk_type); 


        match chunk_type {
            ChunkType::IHDR(_) => {
                println!("IHDR: {:?}", DataIHDR::from_buf(&chunk_data));
            }
            _ => { }
        }
        Chunk { length, chunk_type, chunk_data, crc}
    }
}

impl PNG {

    fn from_file(mut file: File) -> Self {
        let mut buf = Vec::new();
        let mut header: [u8;8] = [0; 8];
        let count = file.read_to_end(&mut buf).expect("Unable to read data");
        println!("count: {:?}", count);

        for i in 0..8 {
            header[i] = buf[i];
        }

        let mut chunks: Vec<Chunk> = Vec::new();

        buf.drain(0..8);
        while !buf.is_empty() {
            chunks.push(Chunk::from_buf(&mut buf));
        }

        PNG {
            header, chunks
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    println!("opening {:?}", filename);

    let file: File = File::open(filename).expect("Unable to open file");

    let png = PNG::from_file(file);

    print!("header:");
    for byte in &png.header[..] {
        print!("{} ", byte);
    }
}
