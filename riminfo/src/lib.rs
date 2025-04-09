use std::path::PathBuf;

pub struct RimInfo {
    pub revision_sha1: String,
    pub target_revision: String,
    pub checksum: String,
    pub ignore: Vec<PathBuf>,
    pub subdir: Vec<PathBuf>,
    pub path: PathBuf,
}

impl RimInfo {
    pub fn is_dirty(&self, checksum: &str) -> bool {
        self.checksum.is_empty() || self.checksum != checksum
    }
}
