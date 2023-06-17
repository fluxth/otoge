use std::path::{Path, PathBuf};

pub trait Otoge {
    type DataStore;
    type Song;

    fn name() -> &'static str;
    fn data_path(base_path: Option<&Path>) -> PathBuf {
        let path = base_path.unwrap_or(Path::new(""));
        path.join(Self::name())
    }

    fn music_data_store_path(base_path: Option<&Path>) -> PathBuf {
        Self::data_path(base_path).join("music.toml")
    }
}

pub trait DataStore {
    fn data_differs(&self, other: &Self) -> bool;
}
