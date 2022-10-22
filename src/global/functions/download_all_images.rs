use crate::global::structs::Item;
use futures::future::join_all;
use std::{error::Error, fs, io::Cursor, thread};
use tokio::runtime::Runtime;

pub struct DownloadRequest {
    pub url: String,
    pub id: String,
}

impl From<&Item> for Option<DownloadRequest> {
    fn from(item: &Item) -> Self {
        Some(match item {
            Item::MiniVideo(minivideo) => DownloadRequest {
                url: minivideo.thumbnail_url.clone(),
                id: minivideo.id.clone(),
            },
            Item::MiniPlaylist(miniplaylist) => DownloadRequest {
                url: miniplaylist.thumbnail_url.clone(),
                id: miniplaylist.id.clone(),
            },
            Item::MiniChannel(minichannel) => DownloadRequest {
                url: minichannel.thumbnail_url.clone(),
                id: minichannel.id.clone(),
            },
            Item::FullVideo(fullvideo) => DownloadRequest {
                url: fullvideo.thumbnail_url.clone(),
                id: fullvideo.id.clone(),
            },
            Item::FullPlaylist(fullplaylist) => DownloadRequest {
                url: fullplaylist.thumbnail_url.clone(),
                id: fullplaylist.id.clone(),
            },
            Item::Unknown(_) => return None,
        })
    }
}

/// Function to download all thumbnails (or just any files) to `~/.cache/thumbnails` with  no file exitension (cuz its not needed)
pub fn download_all_images(downloads: Vec<Option<DownloadRequest>>) {
    thread::spawn(move || {
        let rt: Runtime = tokio::runtime::Runtime::new().unwrap();

        let _ = rt.block_on(download_all_images_async(
            downloads.into_iter().flatten().collect(),
        ));
    });
}

// Create a `download_single()`s and join them into 1 future
// So all downloads will happen at the same time without waiting
async fn download_all_images_async(downloads: Vec<DownloadRequest>) {
    let mut actions = Vec::new();
    let mut path = home::home_dir().expect("Cannot get your home directory");
    path.push(".cache/youtube-tui/thumbnails/");

    for download in downloads.into_iter() {
        let mut path = path.clone();
        fs::create_dir_all(&path).unwrap();
        path.push(download.id);

        if !path.exists() {
            actions.push(download_single(
                download.url,
                path.to_owned().into_os_string().into_string().unwrap(),
            ));
        }

        path.pop();
    }

    join_all(actions).await;
}

// Download a single file
async fn download_single(url: String, path: String) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Ok(());
    }

    let mut file = std::fs::File::create(path)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
