use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::bufread::ZlibDecoder;
use sha1::{Digest, Sha1};

static CHUNK_MAGIC: [u8; 4] = [0xA2, 0x3A, 0xFE, 0xB1];

#[derive(Debug)]
pub enum HashType {
    RollingHash = 1,
    ShaHash = 2,
    RollingShaHash = 3,
}

#[derive(Debug)]
pub struct Chunk {
    pub header_version: u32,
    pub header_size: u32,
    pub compressed_size: u32,
    pub guid: (u32, u32, u32, u32),
    pub hash: u64,
    pub stored_as: u8,
    pub sha_hash: Option<[u8; 20]>,
    pub hash_type: Option<HashType>,
    pub uncompressed_size: Option<u32>,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        let mut cursor = Cursor::new(data);

        let mut magic = [0u8; 4];
        cursor
            .read_exact(&mut magic)
            .map_err(|_| "Failed to read magic bytes")?;
        if magic != CHUNK_MAGIC {
            return Err("Invalid data: header does not match 0x44BEC00C");
        }

        let header_version = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read header version")?;

        let header_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read header size")?;

        let compressed_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read compressed size")?;

        let guid = (
            cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID part 1")?,
            cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID part 2")?,
            cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID part 3")?,
            cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID part 4")?,
        );

        let hash = cursor
            .read_u64::<LittleEndian>()
            .map_err(|_| "Failed to read hash")?;

        let stored_as = cursor.read_u8().map_err(|_| "Failed to read stored as")?;

        let sha_hash = if header_version >= 2 {
            let mut sha_hash = [0u8; 20];
            cursor
                .read_exact(&mut sha_hash)
                .map_err(|_| "Failed to read SHA hash")?;
            Some(sha_hash)
        } else {
            None
        };

        let hash_type = if header_version >= 2 {
            let hash_type_byte = cursor.read_u8().map_err(|_| "Failed to read hash type")?;
            let hash_type_enum = match hash_type_byte {
                1 => HashType::RollingHash,
                2 => HashType::ShaHash,
                3 => HashType::RollingShaHash,
                _ => return Err("Unknown hash type"),
            };
            Some(hash_type_enum)
        } else {
            None
        };

        let uncompressed_size = if header_version >= 3 {
            Some(
                cursor
                    .read_u32::<LittleEndian>()
                    .map_err(|_| "Failed to read uncompressed size")?,
            )
        } else {
            None
        };

        if cursor.position() != header_size as u64 {
            println!(
                "Warning: Header size mismatch. Expected {}, got {}",
                header_size,
                cursor.position()
            );
            cursor.set_position(header_size as u64);
        }

        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .map_err(|_| "Failed to read chunk data")?;

        if stored_as & 0x1 != 0 {
            let compressed = std::mem::take(&mut data);
            let mut decoder = ZlibDecoder::new(&*compressed);
            decoder
                .read_to_end(&mut data)
                .map_err(|_| "Failed to decompress chunk data")?;
        }

        let mut hasher = Sha1::new();
        hasher.update(&data);
        let result = hasher.finalize();
        println!("SHA1 hash: {:x}", result);

        Ok(Chunk {
            header_version,
            header_size,
            compressed_size,
            guid,
            hash,
            stored_as,
            sha_hash,
            hash_type,
            uncompressed_size,
            data,
        })
    }
}
