use super::utils::read;
use crate::{common::result::Result, storefronts::epicgames::api::CHUNK_MAGIC};
use flate2::bufread::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Chunk {
    pub header: ChunkHeader,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn new(data: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let header = ChunkHeader::new(&mut cursor)?;

        let data = if header.compressed() {
            let mut decoder = ZlibDecoder::new(cursor);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;

            if let Some(uncompressed_size) = header.data_uncompressed_size {
                if decompressed.len() != uncompressed_size as usize {
                    return Err("Uncompressed data size does not match header size")?;
                }
            }

            if let Some(sha1) = header.data_sha1 {
                let computed_sha1 = Sha1::digest(&decompressed);

                if computed_sha1.as_slice() != sha1 {
                    return Err("SHA1 checksum does not match")?;
                }
            }

            decompressed
        } else {
            cursor.into_inner()
        };

        Ok(Self { header, data })
    }
}

#[derive(Debug)]
pub struct ChunkHeader {
    pub version: u32,
    pub size: u32,
    pub compressed_size: u32,
    pub guid: (u32, u32, u32, u32),
    pub hash: u64,
    pub stored_as: u8,
    pub data_sha1: Option<[u8; 20]>,
    pub data_hash_type: Option<u8>,
    pub data_uncompressed_size: Option<u32>,
}

impl ChunkHeader {
    pub fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let initial_position = cursor.position();

        let magic: [u8; 4] = read(cursor)?;
        if magic != CHUNK_MAGIC {
            return Err("Invalid chunk magic".into());
        }

        let version = read(cursor)?;
        let size = read(cursor)?;
        let compressed_size = read(cursor)?;
        let guid = read(cursor)?;
        let hash = read(cursor)?;
        let stored_as = read(cursor)?;

        let (data_sha1, data_hash_type) = if version >= 2 {
            (Some(read(cursor)?), Some(read(cursor)?))
        } else {
            (None, None)
        };

        let data_uncompressed_size = if version >= 3 {
            Some(read(cursor)?)
        } else {
            None
        };

        if size as u64 != cursor.position() - initial_position {
            eprintln!(
                "Chunk header size mismatch: expected {}, got {}",
                size,
                cursor.position()
            );
            cursor.set_position(size as u64 + initial_position);
        }

        Ok(Self {
            version,
            size,
            compressed_size,
            guid,
            hash,
            stored_as,
            data_sha1,
            data_hash_type,
            data_uncompressed_size,
        })
    }

    pub fn compressed(&self) -> bool {
        self.stored_as & 0x1 != 0
    }
}
