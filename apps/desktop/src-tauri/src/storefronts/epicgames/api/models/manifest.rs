use crate::{
    common::result::Result,
    storefronts::epicgames::api::{models::utils::read, Guid, MANIFEST_MAGIC},
};
use flate2::bufread::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

#[derive(Debug)]
pub struct Manifest {
    pub header: ManifestHeader,
    pub meta: ManifestMeta,
    pub cdl: ManifestCDL,
    pub fml: ManifestFML,
    pub custom_fields: ManifestCustomFields,
}

impl Manifest {
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);

        let header = ManifestHeader::new(&mut cursor)?;

        let mut decompressed_cursor = if header.compressed() {
            let mut decoder = ZlibDecoder::new(cursor);
            let mut data = Vec::with_capacity(header.data_size as usize);
            decoder.read_to_end(&mut data)?;

            if data.len() != header.data_size as usize {
                return Err("Decompressed size does not match expected size")?;
            }

            let computed_sha1 = Sha1::digest(&data);

            if computed_sha1.as_slice() != header.data_sha1 {
                return Err("SHA1 checksum does not match")?;
            }

            Cursor::new(data)
        } else {
            cursor
        };

        let meta = ManifestMeta::new(&mut decompressed_cursor)?;
        let cdl = ManifestCDL::new(&mut decompressed_cursor, header.version)?;
        let fml = ManifestFML::new(&mut decompressed_cursor)?;
        let custom_fields = ManifestCustomFields::new(&mut decompressed_cursor)?;

        Ok(Self {
            header,
            meta,
            cdl,
            fml,
            custom_fields,
        })
    }

    pub fn download_size(&self) -> u64 {
        self.cdl.total_file_size
    }

    pub fn install_size(&self) -> u64 {
        self.fml.total_size
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
    pub fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
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

    pub fn compressed(&self) -> bool {
        self.stored_as & 0x1 != 0
    }
}

#[derive(Debug)]
pub struct ManifestMeta {
    pub size: u32,
    pub version: u8,
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
    pub uninstall_action_path: Option<String>,
    pub uninstall_action_args: Option<String>,
}

impl ManifestMeta {
    fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let initial_position = cursor.position();

        let size = read(cursor)?;
        let version = read(cursor)?;
        let feature_level = read(cursor)?;
        let is_file_data = read(cursor)?;
        let app_id = read(cursor)?;
        let app_name = read(cursor)?;
        let build_version = read(cursor)?;
        let launch_target = read(cursor)?;
        let launch_command = read(cursor)?;

        let entries: u32 = read(cursor)?;

        let mut prereq_ids = Vec::with_capacity(entries as usize);
        for _ in 0..entries {
            let prereq_id = read(cursor)?;
            prereq_ids.push(prereq_id);
        }

        let prereq_name = read(cursor)?;
        let prereq_path = read(cursor)?;
        let prereq_args = read(cursor)?;

        let build_id = if version >= 1 {
            Some(read(cursor)?)
        } else {
            None
        };

        let (uninstall_action_path, uninstall_action_args) = if version >= 2 {
            let path = read(cursor)?;
            let args = read(cursor)?;
            (Some(args), Some(path))
        } else {
            (None, None)
        };

        if size as u64 != cursor.position() - initial_position {
            eprintln!(
                "Meta size mismatch: expected {}, got {}",
                size,
                cursor.position() - initial_position
            );
            cursor.set_position(size as u64 + initial_position);
        }

        Ok(Self {
            size,
            version,
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

#[derive(Debug)]
pub struct ManifestCDL {
    pub size: u32,
    pub version: u8,
    pub elements: Vec<ChunkInfo>,
    pub total_file_size: u64,
}

impl ManifestCDL {
    fn new(cursor: &mut Cursor<Vec<u8>>, manifest_version: u32) -> Result<Self> {
        let initial_position = cursor.position();

        let size = read(cursor)?;
        let version = read(cursor)?;

        let count: u32 = read(cursor)?;

        let mut elements = Vec::with_capacity(count as usize);
        for _ in 0..count {
            elements.push(ChunkInfo {
                manifest_version,
                guid: read(cursor)?,
                hash: 0,
                sha1: [0; 20],
                group_num: 0,
                window_size: 0,
                file_size: 0,
            });
        }

        for element in &mut elements {
            element.hash = read(cursor)?;
        }

        for element in &mut elements {
            element.sha1 = read(cursor)?;
        }

        for element in &mut elements {
            element.group_num = read(cursor)?;
        }

        for element in &mut elements {
            element.window_size = read(cursor)?;
        }

        let mut total_file_size = 0;

        for element in &mut elements {
            element.file_size = read(cursor)?;
            total_file_size += element.file_size as u64;
        }

        if size as u64 != cursor.position() - initial_position {
            eprintln!(
                "CDL size mismatch: expected {}, got {}",
                size,
                cursor.position() - initial_position
            );
            cursor.set_position(size as u64 + initial_position);
        }

        Ok(Self {
            size,
            version,
            elements,
            total_file_size,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ChunkInfo {
    pub guid: Guid,
    pub hash: u64,
    pub sha1: [u8; 20],
    pub group_num: u8,
    pub window_size: u32,
    pub file_size: i64,
    pub manifest_version: u32,
}

impl ChunkInfo {
    pub fn path(&self) -> String {
        format!(
            "{}/{:02}/{:016X}_{}.chunk",
            self.dir(),
            self.group_num,
            self.hash,
            self.guid_str()
        )
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

    fn guid_str(&self) -> String {
        format!(
            "{:08X}{:08X}{:08X}{:08X}",
            self.guid.0, self.guid.1, self.guid.2, self.guid.3
        )
    }
}

#[derive(Debug)]
pub struct ManifestFML {
    pub size: u32,
    pub version: u8,
    pub elements: Vec<FileManifest>,
    pub total_size: u64,
}

impl ManifestFML {
    fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let initial_position = cursor.position();

        let size = read(cursor)?;
        let version = read(cursor)?;

        let count: u32 = read(cursor)?;

        let mut elements = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let filename = read(cursor)?;
            elements.push(FileManifest {
                filename,
                symlink_target: String::new(),
                sha1: [0; 20],
                flags: 0,
                install_tags: Vec::new(),
                chunk_parts: Vec::new(),
                md5: None,
                mime_type: None,
                sha256: None,
                size: 0,
            });
        }

        for element in &mut elements {
            element.symlink_target = read(cursor)?;
        }

        for element in &mut elements {
            element.sha1 = read(cursor)?;
        }

        for element in &mut elements {
            element.flags = read(cursor)?;
        }

        for element in &mut elements {
            let count: u32 = read(cursor)?;
            for _ in 0..count {
                element.install_tags.push(read(cursor)?);
            }
        }

        for element in &mut elements {
            let count: u32 = read(cursor)?;
            let mut file_offset: u64 = 0;

            for _ in 0..count {
                let initial_position = cursor.position();
                let data_size: u32 = read(cursor)?;

                let guid = read(cursor)?;
                let offset = read(cursor)?;
                let size = read(cursor)?;

                element.chunk_parts.push(ChunkPart {
                    guid,
                    offset,
                    size,
                    file_offset,
                });

                file_offset += size as u64;

                if cursor.position() - initial_position != data_size as u64 {
                    eprintln!(
                        "Chunk part size mismatch: expected {}, got {}",
                        data_size,
                        cursor.position() - initial_position
                    );
                    cursor.set_position(initial_position + data_size as u64);
                }
            }
        }

        if version >= 1 {
            for element in &mut elements {
                let has_md5: u32 = read(cursor)?;

                if has_md5 != 0 {
                    element.md5 = Some(read(cursor)?);
                }
            }

            for element in &mut elements {
                element.mime_type = Some(read(cursor)?);
            }
        }

        if version >= 2 {
            for element in &mut elements {
                element.sha256 = Some(read(cursor)?);
            }
        }

        let mut total_size = 0;

        for element in &mut elements {
            element.size = element
                .chunk_parts
                .iter()
                .map(|part| part.size as u64)
                .sum();
            total_size += element.size;
        }

        if cursor.position() - initial_position != size as u64 {
            eprintln!(
                "FML size mismatch: expected {}, got {}",
                size,
                cursor.position() - initial_position
            );
            cursor.set_position(size as u64 + initial_position);
        }

        Ok(Self {
            size,
            version,
            elements,
            total_size,
        })
    }
}

#[derive(Debug)]
pub struct FileManifest {
    pub filename: String,
    pub symlink_target: String,
    pub sha1: [u8; 20],
    pub flags: u8,
    pub install_tags: Vec<String>,
    pub chunk_parts: Vec<ChunkPart>,
    pub md5: Option<[u8; 16]>,
    pub mime_type: Option<String>,
    pub sha256: Option<[u8; 32]>,
    pub size: u64,
}

#[derive(Debug)]
pub struct ChunkPart {
    pub guid: Guid,
    pub offset: u32,
    pub size: u32,
    pub file_offset: u64,
}

#[derive(Debug)]
pub struct ManifestCustomFields {
    pub size: u32,
    pub version: u8,
    pub elements: HashMap<String, String>,
}

impl ManifestCustomFields {
    pub fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let initial_position = cursor.position();

        let size = read(cursor)?;
        let version = read(cursor)?;

        let count: u32 = read(cursor)?;
        let mut elements = HashMap::with_capacity(count as usize);
        let keys: Vec<String> = (0..count).map(|_| read(cursor)).collect::<Result<_>>()?;

        for key in keys {
            let value = read(cursor)?;
            elements.insert(key, value);
        }

        if cursor.position() - initial_position != size as u64 {
            eprintln!(
                "Custom fields size mismatch: expected {}, got {}",
                size,
                cursor.position() - initial_position
            );
            cursor.set_position(size as u64 + initial_position);
        }

        Ok(Self {
            size,
            version,
            elements,
        })
    }
}
