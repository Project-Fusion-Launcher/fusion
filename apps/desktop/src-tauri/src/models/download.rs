pub struct Download {
    pub DownloadFiles: Vec<DownloadFile>,
}

pub struct DownloadFile {
    pub filename: String,
    pub chunks: Vec<DownloadFileChunk>,
    pub hash: DownloadFileHash,
}

pub struct DownloadFileChunk {
    pub hash: DownloadHash,
    pub size: u32,
    pub offset: u64,
}

pub enum DownloadHash {
    Sha1(String),
    Sha256(String),
    Sha512(String),
    None,
}
