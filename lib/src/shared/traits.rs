use std::path::{Path, PathBuf};

pub trait SongImage {
    fn image_id(&self) -> &str;
    fn image_file(&self) -> Option<&str>;
    fn set_image_file(&mut self, value: Option<String>);
}

pub trait SongMetadata {
    fn title(&self) -> &str;
    fn artist(&self) -> &str;
}

pub trait Otoge {
    type DataStore;
    type Song;

    fn name() -> &'static str;
    fn image_url(image_id: &str) -> String;
    fn data_path(base_path: Option<&Path>) -> PathBuf {
        let path = base_path.unwrap_or(Path::new(""));
        path.join(Self::name())
    }

    fn music_data_store_path(base_path: Option<&Path>) -> PathBuf {
        Self::data_path(base_path).join("music.toml")
    }
}

pub trait DataStore {
    type Song: SongImage + SongMetadata;

    fn data_differs(&self, other: &Self) -> bool;
    fn songs(&self) -> &[Self::Song];
    fn songs_mut(&mut self) -> &mut Vec<Self::Song>;

    fn clear_image_files(&mut self) {
        for song in self.songs_mut() {
            song.set_image_file(None);
        }
    }
}
