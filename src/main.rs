use std::fs::File;
use std::io::Read;
use std::env;
use std::string::String;
extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

struct PNG {
    header: [u8; 8],
    chunks: Vec<Chunk>
}

#[derive (Copy, Clone, Debug)]
enum ChunkType {
    IHDR,
    PLTE,
    IDAT,
    IEND,
    tRNS,
    cHRM,
    gAMA,
    iCCP,
    sBIT,
    sRGB,
    iTXt,
    tEXt,
    zTXt,
    bKGD,
    hIST,
    pHYs,
    sPLT,
    tIME,
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
                Option::from(ChunkType::IHDR)
            }
            "PLTE" => {
                Option::from(ChunkType::PLTE)
            }
            "IDAT" => {
                Option::from(ChunkType::IDAT)
            }
            "IEND" => {
                Option::from(ChunkType::IEND)
            }
            "tRNS" => {
                Option::from(ChunkType::tRNS)
            }
            "cHRM" => {
                Option::from(ChunkType::cHRM)
            }
            "gAMA" => {
                Option::from(ChunkType::gAMA)
            }
            "iCCP" => {
                Option::from(ChunkType::iCCP)
            }
            "sBIT" => {
                Option::from(ChunkType::sBIT)
            }
            "sRGB" => {
                Option::from(ChunkType::sRGB)
            }
            "iTXt" => {
                Option::from(ChunkType::iTXt)
            }
            "tEXt" => {
                Option::from(ChunkType::tEXt)
            }
            "zTXt" => {
                Option::from(ChunkType::zTXt)
            }
            "bKGD" => {
                Option::from(ChunkType::bKGD)
            }
            "hIST" => {
                Option::from(ChunkType::hIST)
            }
            "pHYs" => {
                Option::from(ChunkType::pHYs)
            }
            "sPLT" => {
                Option::from(ChunkType::sPLT)
            }
            "tIME" => {
                Option::from(ChunkType::tIME)
            }
            _ => Option::None
        }
    }
}

struct Chunk {
    length: u32,
    chunk_type: u32,
    chunk_data: Vec<u8>,
    crc: u32
}

impl Chunk {
    fn from_buf(buf: &mut Vec<u8>) -> Chunk {
        let length = PNG::drain_u32(buf);
        println!("length: {}", length); 
        let chunk_type = PNG::drain_u32(buf);
        println!("chunk type: {:x}, {:?}", chunk_type, ChunkType::parse(chunk_type));
        let chunk_data = buf.drain(..length as usize).collect();
        let crc = PNG::drain_u32(buf);
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

    fn drain_u32(buf: &mut Vec<u8>) -> u32 {
        let word: Vec<u8> = buf.drain(..4).collect();
        (&word[..]).read_u32::<BigEndian>().unwrap()
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
