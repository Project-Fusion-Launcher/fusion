use crate::{
    common::result::Result,
    storefronts::epicgames::api::{models::utils::read, MANIFEST_MAGIC},
};
use flate2::bufread::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Manifest {
    pub header: ManifestHeader,
    //pub meta: ManifestMeta,
    //pub chunk_data_list: ManifestCDL,
    //pub file_manifest_list: ManifestFML,
    //pub custom_fields: ManifestCustomFields,
}

impl Manifest {
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);

        let header = ManifestHeader::from_cursor(&mut cursor)?;

        let mut decompressed_cursor = {
            let mut decoder = ZlibDecoder::new(cursor);
            let mut data = Vec::new();
            decoder.read_to_end(&mut data)?;

            if data.len() != header.data_size as usize {
                return Err("Decompressed size does not match expected size")?;
            }

            let mut hasher = Sha1::new();
            hasher.update(&data);
            let sha1 = hasher.finalize();

            if sha1[..] != header.data_sha1 {
                return Err("SHA1 checksum does not match")?;
            }

            Cursor::new(data)
        };

        Ok(Self {
            header,
            //meta: ManifestMeta::default(),
            //chunk_data_list: ManifestCDL::default(),
            //file_manifest_list: ManifestFML::default(),
            //custom_fields: ManifestCustomFields::default(),
        })
    }
}

#[derive(Debug)]
pub struct ManifestHeader {
    pub size: u32,
    pub data_size: u32,
    pub data_size_compressed: u32,
    pub data_sha1: [u8; 20],
    pub stored_as: u8,
    pub version: u32,
}

impl ManifestHeader {
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let initial_position = cursor.position();

        let magic: [u8; 4] = read(cursor)?;
        if magic != MANIFEST_MAGIC {
            return Err("Invalid manifest magic".into());
        }

        let size = read(cursor)?;
        let data_size = read(cursor)?;
        let data_size_compressed = read(cursor)?;
        let data_sha1 = read(cursor)?;
        let stored_as = read(cursor)?;
        let version = read(cursor)?;

        if size as u64 != cursor.position() - initial_position {
            eprintln!(
                "Header size mismatch: expected {}, got {}",
                size,
                cursor.position()
            );
            cursor.set_position(size as u64 + initial_position);
        }

        Ok(Self {
            size,
            data_size,
            data_size_compressed,
            data_sha1,
            stored_as,
            version,
        })
    }
}
