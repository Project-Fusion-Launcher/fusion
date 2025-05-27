use std::collections::{HashMap, VecDeque};

use super::manifest::Manifest;

#[derive(Debug)]
pub struct DownloadPlan {
    manifest: Option<Manifest>,
    pub download_tasks: VecDeque<DownloadTask>,
    pub write_tasks: VecDeque<WriteTask>,
}

impl DownloadPlan {
    pub fn new(manifest: Manifest) -> Self {
        Self {
            download_tasks: VecDeque::with_capacity(manifest.cdl.elements.len()),
            write_tasks: VecDeque::with_capacity(manifest.cdl.elements.len()),
            manifest: Some(manifest),
        }
    }

    pub fn compute(&mut self) {
        let manifest = self.manifest.take().unwrap();

        let mut chunks_to_download = HashMap::new();
        for element in manifest.cdl.elements {
            chunks_to_download.insert(
                element.guid,
                DownloadTask {
                    chunk_guid: element.guid,
                    chunk_path: element.path(),
                },
            );
        }

        let mut files_per_chunk = HashMap::new();
        for element in manifest.fml.elements.iter() {
            for chunk_part in element.chunk_parts.iter() {
                *files_per_chunk.entry(chunk_part.guid).or_insert(1) += 1_u32;
            }
        }

        for element in manifest.fml.elements {
            self.write_tasks.push_back(WriteTask::Open {
                filename: element.filename,
            });
            for chunk_part in element.chunk_parts {
                if let Some(task) = chunks_to_download.remove(&chunk_part.guid) {
                    self.download_tasks.push_back(task);
                }

                if let Some(remaining_files) = files_per_chunk.get_mut(&chunk_part.guid) {
                    let remove_cache = *remaining_files == 1;

                    self.write_tasks.push_back(WriteTask::Write {
                        chunk_guid: chunk_part.guid,
                        chunk_offset: chunk_part.offset as usize,
                        remove_cache,
                        size: chunk_part.size as usize,
                    });

                    *remaining_files -= 1;
                } else {
                    panic!("Chunk GUID not found in files_per_chunk map");
                }
            }
            self.write_tasks
                .push_back(WriteTask::Close { sha1: element.sha1 });
        }
    }
}

#[derive(Debug)]
pub struct DownloadTask {
    pub chunk_guid: (u32, u32, u32, u32),
    pub chunk_path: String,
}

#[derive(Debug)]
pub enum WriteTask {
    Open {
        filename: String,
    },
    Write {
        chunk_guid: (u32, u32, u32, u32),
        chunk_offset: usize,
        size: usize,
        remove_cache: bool,
    },
    Close {
        sha1: [u8; 20],
    },
}
