use tui::{
    layout::Rect,
    style::Style,
    widgets::{Paragraph, Wrap},
};
use tui_additions::framework::FrameworkItem;
use viuer::{print_from_file, Config};

use crate::{
    config::{AppearanceConfig, MainConfig},
    global::item::Item,
};

// an item info displays info of any `Item`s
#[derive(Clone)]
pub struct ItemInfo {
    pub item: Option<Item>,
}

impl FrameworkItem for ItemInfo {
    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
        popup_render: bool,
        _info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let item = if let Some(item) = &self.item {
            item
        } else {
            return;
        };

        let main_config = framework.data.global.get::<MainConfig>().unwrap();
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

        // The scroll (space above) text info will be the height of the image, but if the image fail to display, the scroll will be 0
        let scroll = if main_config.images.display() {
            let thumbnail_path = home::home_dir()
                .unwrap()
                .join(".cache/youtube-tui/thumbnails/")
                .join(item.thumbnail_id());
            if thumbnail_path.exists() {
                let config = Config {
                    width: Some(area.width as u32),
                    x: area.x,
                    y: area.y as i16,
                    use_sixel: main_config.images.use_sixels(),
                    ..Default::default()
                };
                if let Ok((_, height)) = print_from_file(thumbnail_path, &config) {
                    height as u16
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };

        // Each "span" contains a string and a Style, and they are one line max each
        // A "text" is used for descriptions in video/playlist and channels, and starts a new line if the old one runs out
        let (spans, text) = match item {
            Item::MiniVideo(minivideo) => {
                let mut out = (
                    vec![
                        (
                            String::from("[Video]"),
                            Style::default().fg(appearance.colors.item_info.tag),
                        ),
                        (
                            minivideo.title.clone(),
                            Style::default().fg(appearance.colors.item_info.title),
                        ),
                        (
                            format!("Uploaded by {}", minivideo.channel),
                            Style::default().fg(appearance.colors.item_info.author),
                        ),
                    ],
                    if let Some(description) = &minivideo.description {
                        Some((
                            description.clone(),
                            Style::default().fg(appearance.colors.item_info.description),
                        ))
                    } else {
                        None
                    },
                );
                if let Some(views) = &minivideo.views {
                    out.0.push((
                        format!("{} views", views),
                        Style::default().fg(appearance.colors.item_info.viewcount),
                    ));
                }
                out.0.extend(vec![
                    (
                        format!("Length: {}", minivideo.length),
                        Style::default().fg(appearance.colors.item_info.length),
                    ),
                    (
                        format!("Published {}", minivideo.published),
                        Style::default().fg(appearance.colors.item_info.published),
                    )
                ].into_iter());
                out
            }
        };

        // puts each "span" in its own line
        let mut y = if scroll >= area.height { 0 } else { scroll } + area.y;
        let bottom = area.bottom();

        for (text, style) in spans.into_iter().take((bottom - y) as usize) {
            let paragraph = Paragraph::new(text).style(style);
            frame.render_widget(
                paragraph,
                Rect {
                    y,
                    height: 1,
                    ..area
                },
            );
            y += 1;
        }

        if y > bottom || text.is_none() {
            return;
        }

        // renders the "text"
        let (text, style) = text.unwrap();
        if text.len() == 0 {
            return;
        }
        let paragraph = Paragraph::new(format!("Description:\n{}", text)).style(style).wrap(Wrap { trim: true });
        frame.render_widget(
            paragraph,
            Rect {
                y,
                height: bottom - y,
                ..area
            },
        );
    }

    fn selectable(&self) -> bool {
        true
    }
}

impl Default for ItemInfo {
    fn default() -> Self {
        Self { item: None }
    }
}
