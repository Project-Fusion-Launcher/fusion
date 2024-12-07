use super::game::GameSource;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct DownloadFinished {
    pub id: String,
    pub source: GameSource,
}
