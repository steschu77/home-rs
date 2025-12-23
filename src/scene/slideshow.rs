use crate::error::{Error, Result};
use crate::scene::{
    Context, Element, Handle, Layout, LayoutId, LayoutItem, Layouter, Picture, Rect, Scene,
    SceneEvent, Text, UserEvent,
};
use crate::util::datetime::Date;
use crate::util::locale::fmt_long;
use crate::v2d::{v2::V2, v4::V4};

// ----------------------------------------------------------------------------
pub struct SlideShowScene {
    photos: Vec<usize>,
    title: String,
    tick_count: usize,
    index: usize,
    photo_index: Option<usize>,
    photo_handle: Option<Handle>,
    text_handle: Option<Handle>,
}

// ----------------------------------------------------------------------------
impl SlideShowScene {
    // ------------------------------------------------------------------------
    pub fn new(photos: Vec<usize>, title: String) -> Result<Self> {
        log::info!("Creating slideshow: {title} with {} photos", photos.len());
        if photos.is_empty() {
            return Err(Error::EmptyPhotos);
        }
        Ok(Self {
            photos,
            title,
            tick_count: 0,
            index: 0,
            photo_index: None,
            photo_handle: None,
            text_handle: None,
        })
    }

    // ------------------------------------------------------------------------
    fn layout(&mut self, ctx: &Context, layouter: &mut Layouter) -> Option<Layout> {
        if Some(self.index) == self.photo_index {
            return None;
        }

        if let Some(handle) = self.photo_handle {
            log::info!("Freeing previous photo handle: {:?}", handle.id);
            layouter.free_handle(handle);
            self.photo_handle = None;
        }

        if let Some(handle) = self.text_handle {
            log::info!("Freeing previous text handle: {:?}", handle.id);
            layouter.free_text(handle);
            self.text_handle = None;
        }

        let id = self.photos[self.index];
        let Some(photo) = ctx.find_photo(id) else {
            log::warn!("Photo id {id} not found in context");
            return Some(Layout::empty());
        };
        let Ok(handle) = layouter.load_photo(photo).inspect_err(|e| {
            log::error!("Failed to load photo {:?}: {}", photo.path, e);
        }) else {
            return Some(Layout::empty());
        };

        self.photo_handle = Some(handle);
        self.photo_index = Some(self.index);

        let src_aspect = handle.aspect_ratio;
        let dst_aspect = layouter.aspect_ratio();

        let (pos, size) = if src_aspect > dst_aspect {
            // source is wider than destination
            let scaled_height = dst_aspect / src_aspect;
            let ofs_y = (1.0 - scaled_height) / 2.0;
            let pos = V2::new([0.0, ofs_y]);
            let size = V2::new([1.0, 1.0 * scaled_height]);
            (pos, size)
        } else {
            // source is taller than destination
            let scaled_width = src_aspect / dst_aspect;
            let ofs_x = (1.0 - scaled_width) / 2.0;
            let pos = V2::new([ofs_x, 0.0]);
            let size = V2::new([scaled_width, 1.0]);
            (pos, size)
        };

        let picture = Picture {
            dst: Rect { pos, size },
            src: Rect {
                pos: V2::new([0.0, 0.0]),
                size: V2::new([1.0, 1.0]),
            },
            opacity: 1.0,
            handle,
        };

        // let text = photo
        //     .meta
        //     .datetime
        //     .map(|dt| fmt_long(&dt.date, ctx.locale.as_ref()))
        //     .unwrap_or_else(|| self.title.clone());

        // get first photo title or use default scene title
        let text = if let Some(titles) = &photo.meta.title {
            titles.first()
        } else {
            None
        }
        .unwrap_or(&self.title)
        .to_string();

        let Ok(handle) = layouter.create_text(&text).inspect_err(|e| {
            log::error!("Failed to create text {text}: {e}");
        }) else {
            return Some(Layout::empty());
        };

        self.text_handle = Some(handle);

        let text = Text {
            dst: Rect {
                pos: V2::new([0.025, 0.025]),
                size: V2::new([0.05, 0.05]),
            },
            color: V4::new([1.0, 1.0, 1.0, 1.0]),
            opacity: 1.0,
            handle,
        };

        let items = vec![
            LayoutItem {
                id: LayoutId(0),
                element: Element::Picture(picture),
                animation_time: Some(0.5),
            },
            LayoutItem {
                id: LayoutId(1),
                element: Element::Text(text),
                animation_time: Some(0.5),
            },
        ];

        Some(Layout { items })
    }
}

// ----------------------------------------------------------------------------
impl Scene for SlideShowScene {
    fn update(
        &mut self,
        event: &SceneEvent,
        ctx: &Context,
        layouter: &mut Layouter,
    ) -> Option<Layout> {
        match event {
            SceneEvent::Enter | SceneEvent::User(UserEvent::Home) => {
                self.index = 0;
                self.layout(ctx, layouter)
            }
            SceneEvent::TimeTick => {
                self.tick_count += 1;
                if self.tick_count >= 150 {
                    self.tick_count = 0;
                    self.index = (self.index + 1) % self.photos.len();
                    self.layout(ctx, layouter)
                } else {
                    None
                }
            }
            SceneEvent::User(UserEvent::Next) => {
                self.index = (self.index + 1) % self.photos.len();
                self.layout(ctx, layouter)
            }

            SceneEvent::User(UserEvent::Previous) => {
                self.index = (self.index - 1) % self.photos.len();
                self.layout(ctx, layouter)
            }

            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------
fn select_same_day(date: Date, ctx: &Context) -> Vec<usize> {
    ctx.photos
        .iter()
        .enumerate()
        .filter(|(_, p)| p.meta.datetime.map(|dt| dt.date == date).unwrap_or(false))
        .map(|(idx, _)| idx)
        .collect()
}

// ----------------------------------------------------------------------------
fn select_all(ctx: &Context) -> Vec<usize> {
    Vec::from_iter(0..ctx.photos.len())
}

// ----------------------------------------------------------------------------
pub fn create_daily_slideshow(ctx: &Context) -> Result<SlideShowScene> {
    let today = ctx.time.date;
    let photos = select_same_day(today, ctx);
    SlideShowScene::new(
        photos,
        format!("Photos from {}", fmt_long(&today, ctx.locale.as_ref())),
    )
}

// ----------------------------------------------------------------------------
pub fn create_slideshow_all(ctx: &Context) -> Result<SlideShowScene> {
    SlideShowScene::new(select_all(ctx), String::from("All Photos"))
}
