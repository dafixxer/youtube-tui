use crate::global::functions::*;
use invidious::{channel::Channel, hidden::*, universal::Playlist as FullPlaylist, video::Video};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::Errors;

/// Items are things like a single video/playlist and channel
// they are displayed by the item info widget in `iteminfo.rs`
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Item {
    // minivideos are all videos that appeared without actually clicking into the video
    MiniVideo(MiniVideoItem),
    MiniPlaylist(MiniPlaylistItem),
    MiniChannel(MiniChannelItem),
    FullVideo(FullVideoItem),
    FullPlaylist(FullPlaylistItem),
    FullChannel(FullChannelItem),
    Page(bool), // true: next false: prev
    Unknown(SearchItemTransition),
}

/// stores information of a previewed video
#[derive(Clone, Serialize, Deserialize)]
pub struct MiniVideoItem {
    pub title: String,
    pub id: String,
    pub thumbnail_url: String,
    pub length: String,
    pub views: Option<String>,
    pub channel: String,
    pub channel_id: String,
    pub published: Option<String>,
    pub description: Option<String>,
}

unsafe impl Send for MiniVideoItem {}

/// stores information of a previewed playlist
#[derive(Clone, Serialize, Deserialize)]
pub struct MiniPlaylistItem {
    pub title: String,
    pub id: String,
    pub channel: String,
    pub channel_id: String,
    pub video_count: u32,
    pub thumbnail_url: String,
}

/// stores information of a previewed channel
#[derive(Clone, Serialize, Deserialize)]
pub struct MiniChannelItem {
    pub name: String,
    pub id: String,
    pub thumbnail_url: String,
    pub sub_count: u32,
    pub sub_count_text: String,
    pub video_count: u32,
    pub description: String,
}

/// stores information of a viewed video
#[derive(Clone, Serialize, Deserialize)]
pub struct FullVideoItem {
    pub title: String,
    pub id: String,
    pub thumbnail_url: String,
    pub length: String,
    pub views: String,
    pub channel: String,
    pub channel_id: String,
    pub sub_count: String,
    pub published: String,
    pub description: String,
    pub likes: String,
    // pub dislikes: Option<String>, TODO
    pub genre: String,
}

/// stores information of a viewed playlist
#[derive(Clone, Serialize, Deserialize)]
pub struct FullPlaylistItem {
    pub title: String,
    pub id: String,
    pub channel: String,
    pub channel_id: String,
    pub video_count: u32,
    pub description: String,
    pub views: String,
    pub thumbnail_url: String,
    pub videos: Vec<Item>,
}

/// stores information of a viewed channel
#[derive(Clone, Serialize, Deserialize)]
pub struct FullChannelItem {
    pub name: String,
    pub id: String,
    pub thumbnail_url: String,
    pub sub_count: u32,
    pub sub_count_text: String,
    pub total_views: String,
    pub created: String,
    pub autogenerated: bool,
    pub description: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MiniVideo(video) => &video.title,
            Self::MiniPlaylist(playlist) => &playlist.title,
            Self::MiniChannel(channel) => &channel.name,
            Self::FullVideo(video) => &video.title,
            Self::FullPlaylist(playlist) => &playlist.title,
            Self::FullChannel(channel) => &channel.name,
            Self::Page(b) => {
                if *b {
                    "Next page"
                } else {
                    "Previous page"
                }
            }
            Self::Unknown(_) => "Unknown item",
        })
    }
}

impl Item {
    /// returns the id of an item, `None` is self is Unknown
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::MiniVideo(MiniVideoItem { id, .. })
            | Self::MiniChannel(MiniChannelItem { id, .. })
            | Self::MiniPlaylist(MiniPlaylistItem { id, .. })
            | Self::FullVideo(FullVideoItem { id, .. })
            | Self::FullChannel(FullChannelItem { id, .. })
            | Self::FullPlaylist(FullPlaylistItem { id, .. }) => Some(id),
            Self::Unknown(_) | Self::Page(_) => None,
        }
    }

    /// check if self is Self::Unknown
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

impl Item {
    /// stript self into a FullVideoItem
    pub fn fullvideo(&self) -> Result<&FullVideoItem, Errors> {
        match self {
            Self::FullVideo(fullvideo) => Ok(fullvideo),
            _ => Err(Errors::StrError("not a full video")),
        }
    }

    /// stript self into a MiniVideoItem
    pub fn minivideo(&self) -> Result<&MiniVideoItem, Errors> {
        match self {
            Self::MiniVideo(minivideo) => Ok(minivideo),
            _ => Err(Errors::StrError("not a mini video")),
        }
    }

    /// stript self into a MiniPlaylistItem
    pub fn miniplaylist(&self) -> Result<&MiniPlaylistItem, Errors> {
        match self {
            Self::MiniPlaylist(miniplaylist) => Ok(miniplaylist),
            _ => Err(Errors::StrError("not a mini playlist")),
        }
    }

    /// stript self into a FullPlaylistItem
    pub fn fullplaylist(&self) -> Result<&FullPlaylistItem, Errors> {
        match self {
            Self::FullPlaylist(fullplaylist) => Ok(fullplaylist),
            _ => Err(Errors::StrError("not a full playlist")),
        }
    }

    /// stript self into a MiniChannelItem
    pub fn minichannel(&self) -> Result<&MiniChannelItem, Errors> {
        match self {
            Self::MiniChannel(minichannel) => Ok(minichannel),
            _ => Err(Errors::StrError("not a mini channel")),
        }
    }

    pub fn fullchannel(&self) -> Result<&FullChannelItem, Errors> {
        match self {
            Self::FullChannel(fullchannel) => Ok(fullchannel),
            _ => Err(Errors::StrError("not a full channel")),
        }
    }
}

impl Item {
    /// stript self into a FullVideoItem
    pub fn into_fullvideo(self) -> Result<FullVideoItem, Errors> {
        match self {
            Self::FullVideo(fullvideo) => Ok(fullvideo),
            _ => Err(Errors::StrError("not a full video")),
        }
    }

    /// stript self into a MiniVideoItem
    pub fn into_minivideo(self) -> Result<MiniVideoItem, Errors> {
        match self {
            Self::MiniVideo(minivideo) => Ok(minivideo),
            _ => Err(Errors::StrError("not a mini video")),
        }
    }

    /// stript self into a MiniPlaylistItem
    pub fn into_miniplaylist(self) -> Result<MiniPlaylistItem, Errors> {
        match self {
            Self::MiniPlaylist(miniplaylist) => Ok(miniplaylist),
            _ => Err(Errors::StrError("not a mini playlist")),
        }
    }

    /// stript self into a FullPlaylistItem
    pub fn into_fullplaylist(self) -> Result<FullPlaylistItem, Errors> {
        match self {
            Self::FullPlaylist(fullplaylist) => Ok(fullplaylist),
            _ => Err(Errors::StrError("not a full playlist")),
        }
    }

    /// stript self into a MiniChannelItem
    pub fn into_minichannel(self) -> Result<MiniChannelItem, Errors> {
        match self {
            Self::MiniChannel(minichannel) => Ok(minichannel),
            _ => Err(Errors::StrError("not a mini channel")),
        }
    }

    pub fn into_fullchannel(self) -> Result<FullChannelItem, Errors> {
        match self {
            Self::FullChannel(fullchannel) => Ok(fullchannel),
            _ => Err(Errors::StrError("not a full channel")),
        }
    }
}

impl Item {
    /// returns the file save path for thumbnails (which is by their id)
    /// returns `invalid` if the item does not have a thumbnail
    pub fn thumbnail_id(&self) -> &str {
        match self {
            Self::MiniVideo(video) => &video.id,
            Self::MiniPlaylist(playlist) => &playlist.id,
            Self::MiniChannel(channel) => &channel.id,
            Self::FullVideo(video) => &video.id,
            Self::FullPlaylist(playlist) => &playlist.id,
            Self::FullChannel(channel) => &channel.id,
            Self::Unknown(_) | Self::Page(_) => "invalid",
        }
    }

    /// parse `TrendingVideo` into `Self`
    pub fn from_trending_video(original: TrendingVideo, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbmails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: Some(viewcount_text(original.views)),
            channel: original.author,
            channel_id: original.author_id,
            published: Some(format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            )),
            description: Some(original.description),
        })
    }

    /// parse `PopularItem` into `Self`
    pub fn from_popular_item(original: PopularItem, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: Some(viewcount_text(original.views)),
            channel: original.author,
            channel_id: original.author_id,
            published: Some(format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            )),
            description: None,
        })
    }

    /// parse `Playlist` into `Self`
    pub fn from_mini_playlist(original: Playlist) -> Self {
        Self::MiniPlaylist(MiniPlaylistItem {
            title: original.title,
            id: original.id,
            channel: original.author,
            channel_id: original.author_id,
            video_count: original.video_count,
            thumbnail_url: original.thumbnail,
        })
    }

    /// parse `SearchItem` into `Self`
    pub fn from_search_item(original: SearchItem, image_index: usize) -> Self {
        match original {
            SearchItem::Video {
                title,
                id,
                author,
                author_id,
                length,
                thumbnails,
                description,
                views,
                published,
                published_text,
                ..
            } => Self::MiniVideo(MiniVideoItem {
                title,
                id,
                thumbnail_url: thumbnails[image_index].url.clone(),
                length: secs_display_string(length as u32),
                views: Some(viewcount_text(views)),
                channel: author,
                channel_id: author_id,
                published: Some(format!("{} [{}]", published_text, date_text(published))),
                description: Some(description),
            }),
            SearchItem::Playlist {
                title,
                id,
                author,
                author_id,
                video_count,
                thumbnail,
                ..
            } => Self::MiniPlaylist(MiniPlaylistItem {
                title,
                id,
                channel: author,
                channel_id: author_id,
                video_count,
                thumbnail_url: thumbnail,
            }),
            SearchItem::Channel {
                name,
                id,
                thumbnails,
                subscribers,
                video_count,
                description,
                ..
            } => Self::MiniChannel(MiniChannelItem {
                name,
                id,
                thumbnail_url: format!("https://{}", thumbnails[image_index].url),
                video_count,
                sub_count: subscribers,
                sub_count_text: viewcount_text(subscribers as u64),
                description,
            }),
            SearchItem::Unknown(searchitem_transitional) => Self::Unknown(searchitem_transitional),
        }
    }

    /// parse `Video` into `Self`
    pub fn from_full_video(original: Video, image_index: usize) -> Self {
        Self::FullVideo(FullVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: viewcount_text(original.views),
            channel: original.author,
            channel_id: original.author_id,
            sub_count: original.sub_count_text,
            published: format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            ),
            description: original.description,
            likes: viewcount_text(original.likes as u64),
            genre: original.genre,
        })
    }

    /// parse `FullPlaylist` into `Self`
    pub fn from_full_playlist(original: FullPlaylist, image_index: usize) -> Self {
        Self::FullPlaylist(FullPlaylistItem {
            title: original.title,
            id: original.id,
            channel: original.author,
            channel_id: original.author_id,
            video_count: original.video_count,
            description: original.description,
            views: viewcount_text(original.views),
            thumbnail_url: original.thumbnail,
            videos: original
                .videos
                .into_iter()
                .map(|video| Self::from_playlist_item(video, image_index))
                .collect(),
        })
    }

    /// parse `PlaylistItem` into `Self`
    pub fn from_playlist_item(original: PlaylistItem, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: None,
            channel: original.author,
            channel_id: original.author_id,
            published: None,
            description: None,
        })
    }

    /// parse `Channel` into `Self`
    pub fn from_full_channel(original: Channel, image_index: usize) -> Self {
        Self::FullChannel(FullChannelItem {
            name: original.name,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            sub_count: original.sub_count,
            sub_count_text: viewcount_text(original.sub_count as u64),
            total_views: viewcount_text(original.total_views),
            created: date_text(original.joined),
            autogenerated: original.auto_generated,
            description: original.description,
        })
    }

    /// parse `ChannelVideo` into `Self`
    pub fn from_channel_video(original: ChannelVideo, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: Some(viewcount_text(original.view_count)),
            channel: original.author,
            channel_id: original.author_id,
            published: Some(format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            )),
            description: Some(original.description),
        })
    }

    /// parse `Playlist` into `Self`
    pub fn from_channel_playlist(original: Playlist) -> Self {
        Self::MiniPlaylist(MiniPlaylistItem {
            title: original.title,
            id: original.id,
            channel: original.author,
            channel_id: original.author_id,
            video_count: original.video_count,
            thumbnail_url: original.thumbnail,
        })
    }
}
