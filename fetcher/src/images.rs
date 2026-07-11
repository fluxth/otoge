use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use sha2::{Digest, Sha256};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tracing::{error, info, warn};

use otoge::shared::traits::{SongImage, SongMetadata};

const PROGRESS_MIN_TOTAL: usize = 20;
const PROGRESS_STEPS: usize = 10;
// keep small images in memory, roll over to disk once they exceed this size (bytes)
const SPOOL_THRESHOLD: usize = 2 * 1024 * 1024;

struct PendingDownload {
    song_index: usize,
    url: String,
}

struct CompletedDownload {
    filename: String,
    bytes: u64,
}

pub async fn download_images<S, F>(
    client: &reqwest::Client,
    data_path: &Path,
    songs: &mut [S],
    refresh: bool,
    get_image_url: F,
) -> anyhow::Result<()>
where
    S: SongImage + SongMetadata,
    F: Fn(&str) -> String,
{
    let images_dir = data_path.join("images");
    let semaphore = Arc::new(Semaphore::new(10));

    let mut to_download: Vec<PendingDownload> = Vec::new();
    for (index, song) in songs.iter().enumerate() {
        if song.image_id().is_empty() {
            continue;
        }

        if !refresh
            && let Some(filename) = song.image_file()
            && tokio::fs::try_exists(image_path(&images_dir, filename)).await?
        {
            continue;
        }

        to_download.push(PendingDownload {
            song_index: index,
            url: get_image_url(song.image_id()),
        });
    }

    info!(count = to_download.len(), "Downloading images");

    let mut joinset: JoinSet<(usize, anyhow::Result<Option<CompletedDownload>>)> = JoinSet::new();

    for PendingDownload { song_index, url } in to_download {
        let client = client.clone();
        let images_dir = images_dir.clone();
        let sem = Arc::clone(&semaphore);

        joinset.spawn(async move {
            let _permit = sem.acquire_owned().await.unwrap();
            let result = download_single_image(&client, &url, &images_dir).await;
            (song_index, result)
        });
    }

    let total = joinset.len();
    let mut completed: usize = 0;
    let mut total_bytes: u64 = 0;

    while let Some(join_result) = joinset.join_next().await {
        let (song_index, result) = join_result.expect("task panicked");
        match result {
            Ok(Some(CompletedDownload { filename, bytes })) => {
                total_bytes += bytes;
                songs[song_index].set_image_file(Some(filename));
            }
            Ok(None) => {
                let song = &songs[song_index];
                warn!(
                    title = song.title(),
                    artist = song.artist(),
                    image = song.image_id(),
                    "Image not found (404)"
                );
            }
            Err(err) => {
                let song = &songs[song_index];
                error!(
                    error = %err,
                    title = song.title(),
                    artist = song.artist(),
                    image = song.image_id(),
                    "Failed to download image"
                );
            }
        }

        completed += 1;
        if total >= PROGRESS_MIN_TOTAL {
            let bucket = completed * PROGRESS_STEPS / total;
            let prev_bucket = (completed - 1) * PROGRESS_STEPS / total;
            if bucket != prev_bucket {
                info!(
                    completed,
                    total,
                    bytes = total_bytes,
                    progress = bucket * 10,
                    "Downloading images"
                );
            }
        }
    }

    if completed > 0 {
        info!(completed, total, bytes = total_bytes, "Downloaded images");
    }

    Ok(())
}

pub async fn cleanup_orphans(data_path: &Path, referenced: &HashSet<String>) -> anyhow::Result<()> {
    let images_dir = data_path.join("images");

    let mut prefix_entries = tokio::fs::read_dir(&images_dir).await?;
    let mut to_delete = Vec::new();

    while let Some(prefix_entry) = prefix_entries.next_entry().await? {
        if !prefix_entry.file_type().await?.is_dir() {
            continue;
        }

        let mut file_entries = tokio::fs::read_dir(prefix_entry.path()).await?;
        while let Some(file_entry) = file_entries.next_entry().await? {
            let filename = file_entry.file_name().to_string_lossy().into_owned();
            if !referenced.contains(&filename) {
                to_delete.push(file_entry.path());
            }
        }
    }

    for path in &to_delete {
        info!(path = ?path, "Deleting orphan image");
        tokio::fs::remove_file(path).await?;
    }

    if !to_delete.is_empty() {
        info!(count = to_delete.len(), "Deleted orphan images");
    }

    Ok(())
}

async fn download_single_image(
    client: &reqwest::Client,
    url: &str,
    images_dir: &Path,
) -> anyhow::Result<Option<CompletedDownload>> {
    let response = client.get(url).send().await?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let mut response = response.error_for_status()?;

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| anyhow::anyhow!("Missing or invalid Content-Type header"))?
        .to_owned();

    let extension = content_type_to_ext(&content_type)
        .ok_or_else(|| anyhow::anyhow!("Unsupported content type '{}'", content_type))?;

    let mut spooled = tempfile::spooled_tempfile(SPOOL_THRESHOLD);
    let mut hasher = Sha256::new();
    let mut bytes: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        bytes += u64::try_from(chunk.len())?;
        hasher.update(&chunk);
        std::io::Write::write_all(&mut spooled, &chunk)?;
    }

    let hash: String = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    let filename = format!("{}.{}", hash, extension);
    let filepath = image_path(images_dir, &filename);
    let prefix_dir = filepath.parent().unwrap();
    tokio::fs::create_dir_all(prefix_dir).await?;

    if !tokio::fs::try_exists(&filepath).await? {
        std::io::Seek::seek(&mut spooled, std::io::SeekFrom::Start(0))?;
        let mut out = std::fs::File::create(&filepath)?;
        std::io::copy(&mut spooled, &mut out)?;
    }

    Ok(Some(CompletedDownload { filename, bytes }))
}

fn image_path(images_dir: &Path, filename: &str) -> std::path::PathBuf {
    images_dir.join(&filename[..2]).join(filename)
}

fn content_type_to_ext(content_type: &str) -> Option<&'static str> {
    let mime = content_type.split(';').next()?.trim();
    match mime {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        "image/gif" => Some("gif"),
        _ => None,
    }
}
