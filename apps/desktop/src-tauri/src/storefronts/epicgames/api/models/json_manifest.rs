use super::manifest::*;
use crate::common::result::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonManifest {
    pub manifest_file_version: String,
    #[serde(rename = "bIsFileData")]
    #[serde(default)]
    pub is_file_data: bool,
    #[serde(rename = "AppID")]
    pub app_id: String,
    pub app_name_string: String,
    pub build_version_string: String,
    pub launch_exe_string: String,
    pub launch_command: String,
    pub prereq_ids: Vec<String>,
    pub prereq_name: String,
    pub prereq_path: String,
    pub prereq_args: String,
    pub file_manifest_list: Vec<JsonFM>,
    pub chunk_hash_list: HashMap<String, String>,
    pub chunk_sha_list: HashMap<String, String>,
    pub data_group_list: HashMap<String, String>,
    pub chunk_filesize_list: HashMap<String, String>,
    pub custom_fields: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonFM {
    pub filename: String,
    pub file_hash: String,
    #[serde(rename = "bIsUnixExecutable")]
    #[serde(default)]
    pub is_unix_executable: bool,
    #[serde(rename = "bIsCompressed")]
    #[serde(default)]
    pub is_compressed: bool,
    #[serde(rename = "bIsReadOnly")]
    #[serde(default)]
    pub is_read_only: bool,
    pub file_chunk_parts: Vec<JsonChunkPart>,
}

impl JsonFM {
    fn flags(&self) -> u8 {
        let mut flags = 0u8;
        flags |= self.is_read_only as u8;
        flags |= (self.is_compressed as u8) << 1;
        flags |= (self.is_unix_executable as u8) << 2;
        flags
    }
}

impl From<JsonFM> for FileManifest {
    fn from(json_fm: JsonFM) -> Self {
        let mut fm = FileManifest {
            flags: json_fm.flags(),
            filename: json_fm.filename,
            symlink_target: String::new(),
            sha1: hex_to_sha1(&json_fm.file_hash).unwrap_or_default(),
            install_tags: vec![],
            chunk_parts: Vec::with_capacity(json_fm.file_chunk_parts.len()),
            md5: None,
            mime_type: None,
            sha256: None,
            size: 0,
        };

        let mut offset = 0;

        for part in json_fm.file_chunk_parts {
            let size = blob_to_num(&part.size) as u32;
            fm.chunk_parts.push(ChunkPart {
                guid: hex_to_guid(&part.guid).unwrap_or_default(),
                offset: blob_to_num(&part.offset) as u32,
                size,
                file_offset: offset,
            });

            fm.size += size as u64;
            offset += size as u64;
        }

        fm
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonChunkPart {
    pub guid: String,
    pub offset: String,
    pub size: String,
}

impl From<JsonManifest> for Manifest {
    fn from(json_manifest: JsonManifest) -> Self {
        let header = ManifestHeader {
            size: 0,
            data_size: 0,
            data_size_compressed: 0,
            data_sha1: [0; 20],
            stored_as: 0,
            version: blob_to_num(&json_manifest.manifest_file_version) as u32,
        };

        let meta = ManifestMeta {
            size: 0,
            version: 0,
            feature_level: header.version,
            is_file_data: json_manifest.is_file_data.into(),
            app_id: blob_to_num(&json_manifest.app_id) as u32,
            app_name: json_manifest.app_name_string,
            build_version: json_manifest.build_version_string,
            launch_target: json_manifest.launch_exe_string,
            launch_command: json_manifest.launch_command,
            prereq_ids: json_manifest.prereq_ids,
            prereq_name: json_manifest.prereq_name,
            prereq_path: json_manifest.prereq_path,
            prereq_args: json_manifest.prereq_args,
            build_id: None,
            uninstall_action_path: None,
            uninstall_action_args: None,
        };

        let mut cdl = ManifestCDL {
            size: 0,
            version: 0,
            elements: Vec::with_capacity(json_manifest.chunk_hash_list.len()),
        };

        for (guid, hash) in json_manifest.chunk_hash_list {
            let hash = blob_to_num(&hash);
            let sha1 = json_manifest
                .chunk_sha_list
                .get(&guid)
                .and_then(|s| hex_to_sha1(s).ok());
            let file_size = json_manifest
                .chunk_filesize_list
                .get(&guid)
                .map(|s| blob_to_num(s) as i64);
            let group = json_manifest
                .data_group_list
                .get(&guid)
                .map(|s| blob_to_num(s) as u8);

            cdl.elements.push(ChunkInfo {
                guid: hex_to_guid(&guid).unwrap_or_default(),
                hash,
                sha1: sha1.unwrap_or_default(),
                file_size: file_size.unwrap_or_default(),
                group_num: group.unwrap_or_default(),
                window_size: 1024 * 1024,
                manifest_version: header.version,
            });
        }

        let fml = ManifestFML {
            size: 0,
            version: 0,
            elements: json_manifest
                .file_manifest_list
                .into_iter()
                .map(|fm| fm.into())
                .collect(),
        };

        let custom_fields = ManifestCustomFields {
            size: 0,
            version: 0,
            elements: json_manifest.custom_fields,
        };

        Manifest {
            header,
            meta,
            cdl,
            fml,
            custom_fields,
        }
    }
}

fn blob_to_num(in_str: &str) -> u64 {
    let mut num = 0u64;
    let mut shift = 0;

    for chunk in in_str.as_bytes().chunks(3) {
        if let Ok(part) = std::str::from_utf8(chunk) {
            if let Ok(value) = part.parse::<u64>() {
                num += value << shift;
                shift += 8;
            }
        }
    }

    num
}

fn hex_to_guid(hex_guid: &str) -> Result<(u32, u32, u32, u32)> {
    let bytes = const_hex::decode(hex_guid).map_err(|_| "Invalid hex GUID format")?;

    if bytes.len() != 16 {
        return Err("GUID must be 16 bytes (32 hex chars) long".into());
    }

    let part1 = u32::from_be_bytes(
        bytes[0..4]
            .try_into()
            .map_err(|_| "Failed to convert bytes to u32")?,
    );
    let part2 = u32::from_be_bytes(
        bytes[4..8]
            .try_into()
            .map_err(|_| "Failed to convert bytes to u32")?,
    );
    let part3 = u32::from_be_bytes(
        bytes[8..12]
            .try_into()
            .map_err(|_| "Failed to convert bytes to u32")?,
    );
    let part4 = u32::from_be_bytes(
        bytes[12..16]
            .try_into()
            .map_err(|_| "Failed to convert bytes to u32")?,
    );

    Ok((part1, part2, part3, part4))
}

fn hex_to_sha1(hex_str: &str) -> Result<[u8; 20]> {
    let bytes = const_hex::decode(hex_str).map_err(|_| "Invalid SHA1 hex string format")?;
    if bytes.len() != 20 {
        return Err("SHA1 hex string must be 40 characters long".into());
    }
    let arr: [u8; 20] = bytes.try_into().expect("slice with incorrect length");
    Ok(arr)
}
