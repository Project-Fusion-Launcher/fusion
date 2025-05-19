use byteorder::{LittleEndian, ReadBytesExt};
use flate2::bufread::ZlibDecoder;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{Cursor, Read, Seek, SeekFrom},
};

#[derive(Serialize, Debug)]
pub struct Manifest {
    pub header_size: u32,
    pub size_uncompressed: u32,
    pub size_compressed: u32,
    pub sha_hash: [u8; 20],
    pub stored_as: u8,
    pub version: u32,
    pub meta: ManifestMeta,
    pub chunk_data_list: ManifestCDL,
    pub file_manifest_list: ManifestFML,
    pub custom_fields: ManifestCustomFields,
}

impl Manifest {
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        let mut cursor = Cursor::new(data);

        let mut magic = [0u8; 4];
        cursor
            .read_exact(&mut magic)
            .map_err(|_| "Failed to read magic bytes")?;
        if magic != [0x0C, 0xC0, 0xBE, 0x44] {
            return Err("Invalid data: header does not match 0x44BEC00C");
        }

        let header_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read header size")?;
        let size_uncompressed = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read uncompressed size")?;
        let size_compressed = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read compressed size")?;
        let mut sha_hash = [0u8; 20];
        cursor
            .read_exact(&mut sha_hash)
            .map_err(|_| "Failed to read SHA hash")?;
        let stored_as = cursor.read_u8().map_err(|_| "Failed to read stored as")?;
        let version = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read version")?;

        if header_size != cursor.position() as u32 {
            cursor.set_position(header_size as u64);
        }

        let mut compressed_data = Vec::new();
        cursor
            .read_to_end(&mut compressed_data)
            .map_err(|_| "Failed to read compressed data")?;

        let mut zlib_decoder = ZlibDecoder::new(compressed_data.as_slice());
        let mut uncompressed_data = Vec::new();
        zlib_decoder
            .read_to_end(&mut uncompressed_data)
            .map_err(|_| "Decompression failed")?;

        if uncompressed_data.len() != size_uncompressed as usize {
            return Err("Decompressed size does not match expected size");
        }

        let mut hasher = Sha1::new();
        hasher.update(&uncompressed_data);
        let hash = hasher.finalize();

        if hash.as_slice() != sha_hash {
            return Err("SHA hash does not match expected hash");
        }

        let mut cursor = Cursor::new(&uncompressed_data);

        let meta = ManifestMeta::from_cursor(&mut cursor)?;
        let chunk_data_list = ManifestCDL::from_cursor(&mut cursor, version)?;
        let file_manifest_list = ManifestFML::from_cursor(&mut cursor)?;
        let custom_fields = ManifestCustomFields::from_cursor(&mut cursor)?;

        let mut remaining_data = Vec::new();
        cursor
            .read_to_end(&mut remaining_data)
            .map_err(|_| "Failed to read remaining data")?;
        if !remaining_data.is_empty() {
            println!(
                "Warning: remaining data after parsing manifest: {} bytes",
                remaining_data.len()
            );
        }

        Ok(Manifest {
            header_size,
            size_uncompressed,
            size_compressed,
            sha_hash,
            stored_as,
            version,
            meta,
            chunk_data_list,
            file_manifest_list,
            custom_fields,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct ManifestMeta {
    pub meta_size: u32,
    pub data_version: u8,
    pub feature_level: u32,
    pub is_file_data: u8,
    pub app_id: u32,
    pub app_name: String,
    pub build_version: String,
    pub launch_target: String,
    pub launch_command: String,
    pub prereq_ids: Vec<String>,
    pub prereq_name: String,
    pub prereq_path: String,
    pub prereq_args: String,
    pub build_id: Option<String>,
    pub uninstall_action_args: Option<String>,
    pub uninstall_action_path: Option<String>,
}

impl ManifestMeta {
    fn from_cursor(cursor: &mut Cursor<&Vec<u8>>) -> Result<Self, &'static str> {
        let meta_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read meta size")?;
        let data_version = cursor
            .read_u8()
            .map_err(|_| "Failed to read data version")?;
        let feature_level = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read feature level")?;
        let is_file_data = cursor
            .read_u8()
            .map_err(|_| "Failed to read is file data")?;
        let app_id = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read app id")?;

        let app_name = read_fstring(cursor).map_err(|_| "Failed to read app name")?;
        let build_version = read_fstring(cursor).map_err(|_| "Failed to read build version")?;
        let launch_target = read_fstring(cursor).map_err(|_| "Failed to read launch target")?;
        let launch_command = read_fstring(cursor).map_err(|_| "Failed to read launch command")?;

        let prereq_entries = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read prereq entries")?;

        let mut prereq_ids = Vec::new();

        for _ in 0..prereq_entries {
            let prereq_id = read_fstring(cursor).map_err(|_| "Failed to read prereq id")?;
            prereq_ids.push(prereq_id);
        }

        let prereq_name = read_fstring(cursor).map_err(|_| "Failed to read prereq name")?;
        let prereq_path = read_fstring(cursor).map_err(|_| "Failed to read prereq path")?;
        let prereq_args = read_fstring(cursor).map_err(|_| "Failed to read prereq args")?;

        let build_id = if data_version >= 1 {
            Some(read_fstring(cursor).map_err(|_| "Failed to read build id")?)
        } else {
            None
        };
        let uninstall_action_args = if data_version >= 2 {
            Some(read_fstring(cursor).map_err(|_| "Failed to read uninstall action args")?)
        } else {
            None
        };
        let uninstall_action_path = if data_version >= 2 {
            Some(read_fstring(cursor).map_err(|_| "Failed to read uninstall action path")?)
        } else {
            None
        };

        if meta_size != cursor.position() as u32 {
            println!(
                "Warning: meta size mismatch. Expected: {}, Actual: {}",
                meta_size,
                cursor.position()
            );
            cursor.set_position(meta_size as u64);
        }

        Ok(ManifestMeta {
            meta_size,
            data_version,
            feature_level,
            is_file_data,
            app_id,
            app_name,
            build_version,
            launch_target,
            launch_command,
            prereq_ids,
            prereq_name,
            prereq_path,
            prereq_args,
            build_id,
            uninstall_action_args,
            uninstall_action_path,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct ManifestCDL {
    pub cdl_size: u32,
    pub cdl_version: u8,
    pub count: u32,
    pub chunks: Vec<Chunk>,
}

impl ManifestCDL {
    fn from_cursor(
        cursor: &mut Cursor<&Vec<u8>>,
        manifest_version: u32,
    ) -> Result<Self, &'static str> {
        let initial_position = cursor.position();

        let cdl_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read CDL size")?;
        let cdl_version = cursor.read_u8().map_err(|_| "Failed to read CDL version")?;
        let count = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read count")?;

        let mut chunks = Vec::new();

        for _ in 0..count {
            let guid1 = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID1")?;
            let guid2 = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID2")?;
            let guid3 = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID3")?;
            let guid4 = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read GUID4")?;

            chunks.push(Chunk {
                manifest_version,
                guid: (guid1, guid2, guid3, guid4),
                hash: 0,
                sha_hash: [0; 20],
                group_num: 0,
                window_size: 0,
                file_size: 0,
            });
        }

        for chunk in &mut chunks {
            let hash = cursor
                .read_u64::<LittleEndian>()
                .map_err(|_| "Failed to read hash")?;
            chunk.hash = hash;
        }

        for chunk in &mut chunks {
            let mut sha_hash = [0u8; 20];
            cursor
                .read_exact(&mut sha_hash)
                .map_err(|_| "Failed to read SHA hash")?;
            chunk.sha_hash = sha_hash;
        }

        for chunk in &mut chunks {
            let group_num = cursor
                .read_u8()
                .map_err(|_| "Failed to read group number")?;
            chunk.group_num = group_num;
        }

        for chunk in &mut chunks {
            let window_size = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read window size")?;
            chunk.window_size = window_size;
        }

        for chunk in &mut chunks {
            let file_size = cursor
                .read_u64::<LittleEndian>()
                .map_err(|_| "Failed to read file size")?;
            chunk.file_size = file_size;
        }

        if cdl_size as u64 != cursor.position() - initial_position {
            println!(
                "Warning: cdl size mismatch. Expected: {}, Actual: {}",
                cdl_size,
                cursor.position() - initial_position
            );
            cursor.set_position(cdl_size as u64 + initial_position);
        }

        Ok(ManifestCDL {
            cdl_size,
            cdl_version,
            count,
            chunks,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct Chunk {
    pub guid: (u32, u32, u32, u32),
    pub hash: u64,
    pub sha_hash: [u8; 20],
    pub group_num: u8,
    pub window_size: u32,
    pub file_size: u64,
    pub manifest_version: u32,
}

impl Chunk {
    pub fn path(&self) -> String {
        format!("{}/", self.dir())
    }

    fn dir(&self) -> &'static str {
        if self.manifest_version >= 15 {
            "ChunksV4"
        } else if self.manifest_version >= 6 {
            "ChunksV3"
        } else if self.manifest_version >= 3 {
            "ChunksV2"
        } else {
            "Chunks"
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ManifestFML {
    pub fml_size: u32,
    pub fml_version: u8,
    pub count: u32,
    pub elements: Vec<FileManifest>,
}

impl ManifestFML {
    fn from_cursor(cursor: &mut Cursor<&Vec<u8>>) -> Result<Self, &'static str> {
        let initial_position = cursor.position();

        let fml_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read FML size")?;
        let fml_version = cursor.read_u8().map_err(|_| "Failed to read FML version")?;
        let count = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read count")?;

        let mut elements = Vec::new();

        for _ in 0..count {
            let filename = read_fstring(cursor).map_err(|_| "Failed to read filename")?;
            elements.push(FileManifest {
                filename,
                symlink_target: String::new(),
                hash: [0; 20],
                flags: 0,
                install_tags: Vec::new(),
                chunk_parts: Vec::new(),
                hash_md5: None,
                mime_type: None,
                hash_sha256: None,
                file_size: 0,
            });
        }

        for element in &mut elements {
            let symlink_target =
                read_fstring(cursor).map_err(|_| "Failed to read symlink target")?;
            element.symlink_target = symlink_target;
        }

        for element in &mut elements {
            let mut hash = [0u8; 20];
            cursor
                .read_exact(&mut hash)
                .map_err(|_| "Failed to read hash")?;
            element.hash = hash;
        }

        for element in &mut elements {
            let flags = cursor.read_u8().map_err(|_| "Failed to read flags")?;
            element.flags = flags;
        }

        for element in &mut elements {
            let install_tags_count = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read install tags count")?;
            for _ in 0..install_tags_count {
                let install_tag = read_fstring(cursor).map_err(|_| "Failed to read install tag")?;
                element.install_tags.push(install_tag);
            }
        }

        for element in &mut elements {
            let chunk_parts = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| "Failed to read chunk parts")?;
            let mut file_offset: u64 = 0;
            for _ in 0..chunk_parts {
                let initial_position = cursor.position();
                let chunk_part_size = cursor
                    .read_u32::<LittleEndian>()
                    .map_err(|_| "Failed to read chunk part size")?
                    as u64;
                let guid = cursor
                    .read_u128::<LittleEndian>()
                    .map_err(|_| "Failed to read GUID")?;
                let offset = cursor
                    .read_u32::<LittleEndian>()
                    .map_err(|_| "Failed to read offset")?;
                let size = cursor
                    .read_u32::<LittleEndian>()
                    .map_err(|_| "Failed to read size")?;

                element.chunk_parts.push(ChunkPart {
                    guid,
                    offset,
                    size,
                    file_offset,
                });

                file_offset += size as u64;

                if cursor.position() - initial_position - chunk_part_size > 0 {
                    println!(
                        "Warning: chunk part size mismatch. Expected: {}, Actual: {}",
                        size,
                        cursor.position() - initial_position - chunk_part_size
                    );
                    cursor.set_position(cursor.position() + chunk_part_size);
                }
            }
        }

        if fml_version >= 1 {
            for element in &mut elements {
                let has_md5 = cursor
                    .read_u32::<LittleEndian>()
                    .map_err(|_| "Failed to read MD5 presence flag")?;

                if has_md5 != 0 {
                    let mut hash_md5 = [0u8; 16];
                    cursor
                        .read_exact(&mut hash_md5)
                        .map_err(|_| "Failed to read MD5 hash")?;
                    element.hash_md5 = Some(hash_md5);
                }
            }

            for element in &mut elements {
                let mime_type = read_fstring(cursor).unwrap_or_default();
                element.mime_type = Some(mime_type);
            }
        }

        if fml_version >= 2 {
            for element in &mut elements {
                let mut hash_sha256 = [0u8; 32];
                cursor.read_exact(&mut hash_sha256).unwrap_or_default();
                element.hash_sha256 = Some(hash_sha256);
            }
        }

        for element in &mut elements {
            element.file_size = element
                .chunk_parts
                .iter()
                .map(|chunk_part| chunk_part.size as u64)
                .sum();
        }

        if cursor.position() - initial_position != fml_size as u64 {
            println!(
                "Warning: fml size mismatch. Expected: {}, Actual: {}",
                fml_size,
                cursor.position() - initial_position
            );
            cursor.set_position(fml_size as u64 + initial_position);
        }

        Ok(ManifestFML {
            fml_size,
            fml_version,
            count,
            elements,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct FileManifest {
    pub filename: String,
    pub symlink_target: String,
    pub hash: [u8; 20],
    pub flags: u8,
    pub install_tags: Vec<String>,
    pub chunk_parts: Vec<ChunkPart>,
    pub hash_md5: Option<[u8; 16]>,
    pub mime_type: Option<String>,
    pub hash_sha256: Option<[u8; 32]>,
    pub file_size: u64,
}

#[derive(Serialize, Debug)]
pub struct ChunkPart {
    pub guid: u128,
    pub offset: u32,
    pub size: u32,
    pub file_offset: u64,
}

#[derive(Serialize, Debug)]
pub struct ManifestCustomFields {
    pub custom_fields_size: u32,
    pub custom_fields_version: u8,
    pub count: u32,
    pub fields: HashMap<String, String>,
}

impl ManifestCustomFields {
    fn from_cursor(cursor: &mut Cursor<&Vec<u8>>) -> Result<Self, &'static str> {
        let initial_position = cursor.position();

        let custom_fields_size = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read custom fields size")?;
        let custom_fields_version = cursor
            .read_u8()
            .map_err(|_| "Failed to read custom fields version")?;
        let count = cursor
            .read_u32::<LittleEndian>()
            .map_err(|_| "Failed to read custom fields count")?;

        let mut keys = Vec::new();
        let mut values = Vec::new();

        let mut fields = HashMap::new();

        for _ in 0..count {
            let key = read_fstring(cursor).map_err(|_| "Failed to read custom field key")?;
            keys.push(key);
        }

        for _ in 0..count {
            let value = read_fstring(cursor).map_err(|_| "Failed to read custom field value")?;
            values.push(value);
        }

        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            fields.insert(key, value);
        }

        if cursor.position() - initial_position != custom_fields_size as u64 {
            println!(
                "Warning: custom fields size mismatch. Expected: {}, Actual: {}",
                custom_fields_size,
                cursor.position() - initial_position
            );
            cursor.set_position(initial_position + custom_fields_size as u64);
        }

        Ok(ManifestCustomFields {
            custom_fields_size,
            custom_fields_version,
            count,
            fields,
        })
    }
}

fn read_fstring<R: Read + Seek>(reader: &mut R) -> Result<String, &'static str> {
    let length = reader
        .read_i32::<LittleEndian>()
        .map_err(|_| "Failed to read string length")?;

    match length.cmp(&0) {
        Ordering::Less => {
            let byte_length = (-length * 2) as usize;
            let mut buffer = vec![0u8; byte_length - 2];
            reader
                .read_exact(&mut buffer)
                .map_err(|_| "Failed to read UTF-16 string data")?;
            reader
                .seek(SeekFrom::Current(2))
                .map_err(|_| "Failed to skip UTF-16 null terminator")?;

            let utf16: Vec<u16> = buffer
                .chunks(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();

            String::from_utf16(&utf16).map_err(|_| "Failed to decode UTF-16 string")
        }
        Ordering::Greater => {
            let mut buffer = vec![0u8; (length - 1) as usize];
            reader
                .read_exact(&mut buffer)
                .map_err(|_| "Failed to read UTF-8 string data")?;
            reader
                .seek(SeekFrom::Current(1))
                .map_err(|_| "Failed to skip UTF-8 null terminator")?;
            String::from_utf8(buffer).map_err(|_| "Failed to decode UTF-8 string")
        }
        Ordering::Equal => Ok(String::new()),
    }
}
