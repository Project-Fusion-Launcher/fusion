use super::manifest::Manifest;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct DownloadPlan {
    manifest: Option<Manifest>,
    pub download_tasks: VecDeque<DownloadTask>,
    pub write_tasks: VecDeque<WriteTask>,
}

impl DownloadPlan {
    pub fn new(manifest: Manifest) -> Self {
        let capacity = manifest.cdl.elements.len();
        Self {
            download_tasks: VecDeque::with_capacity(capacity),
            write_tasks: VecDeque::with_capacity(capacity),
            manifest: Some(manifest),
        }
    }

    pub fn compute(&mut self) {
        let manifest = self.manifest.take().unwrap();

        let mut chunk_ref_counts: HashMap<_, u32> =
            HashMap::with_capacity(manifest.cdl.elements.len());
        manifest.fml.elements.iter().for_each(|file| {
            file.chunk_parts.iter().for_each(|chunk| {
                *chunk_ref_counts.entry(chunk.guid).or_insert(0) += 1;
            });
        });

        let mut download_map: HashMap<_, _> = manifest
            .cdl
            .elements
            .into_iter()
            .map(|element| {
                (
                    element.guid,
                    DownloadTask {
                        chunk_guid: element.guid,
                        chunk_path: element.path(),
                    },
                )
            })
            .collect();

        for file in manifest.fml.elements {
            self.write_tasks.push_back(WriteTask::Open {
                filename: file.filename,
            });

            for chunk_part in file.chunk_parts {
                if let Some(task) = download_map.remove(&chunk_part.guid) {
                    self.download_tasks.push_back(task);
                }

                let count = chunk_ref_counts
                    .get_mut(&chunk_part.guid)
                    .expect("Missing chunk GUID in reference count");

                let is_last_use = *count == 1;

                self.write_tasks.push_back(WriteTask::Write {
                    chunk_guid: chunk_part.guid,
                    chunk_offset: chunk_part.offset as usize,
                    size: chunk_part.size as usize,
                    remove_cache: is_last_use,
                });

                *count -= 1;
            }
            self.write_tasks
                .push_back(WriteTask::Close { sha1: file.sha1 });
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
