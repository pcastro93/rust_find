use std::path::PathBuf;

#[derive(Debug)]
pub struct DirWithDepth {
    pub dir: PathBuf,
    pub depth: i32,
}

impl DirWithDepth {
    pub fn new(pb: PathBuf, depth: i32) -> Self {
        DirWithDepth {
            dir: pb,
            depth: depth,
        }
    }
}
