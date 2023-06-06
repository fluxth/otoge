use std::path::{Path, PathBuf};

pub trait Otoge {
    type DataStore;
    type Song;

    fn name() -> &'static str;
    fn data_path(base_path: Option<&Path>) -> PathBuf {
        let path = base_path.unwrap_or(Path::new(""));
        path.join(Self::name())
    }
}

pub trait DataStore {
    fn data_differs(&self, other: &Self) -> bool;
}
