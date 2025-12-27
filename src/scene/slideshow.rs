use crate::error::{Error, Result};
use crate::scene::{
    Context, Element, Handle, Layout, LayoutId, LayoutItem, Layouter, Picture, Rect, Scene,
    SceneEvent, Text, Transition, UserEvent,
};
use crate::util::datetime::Date;
use crate::util::locale::fmt_long;
use crate::v2d::{v2::V2, v4::V4};

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct SlideShowScene {
    photos: Vec<usize>,
    title: String,
    tick_count: usize,
    index: usize,
    state: SlideshowState,
}

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
struct PhotoState {
    index: usize,
    photo: Handle,
    text: Handle,
}

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
enum SlideshowState {
    Idle,
    Static {
        photo: PhotoState,
    },
    Transitioning {
        photo_from: PhotoState,
        photo_to: PhotoState,
        duration: usize,
    },
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
            state: SlideshowState::Idle,
        })
    }

    // ------------------------------------------------------------------------
    fn start_transition(
        &mut self,
        next_index: usize,
        ctx: &Context,
        layouter: &mut Layouter,
    ) -> Option<bool> {
        self.finish_transition(layouter);
        log::info!("Slideshow: transitioning to photo index {}", next_index);

        let id = self.photos[next_index];
        let photo = ctx.find_photo(id)?;
        let photo_handle = layouter.load_photo(photo).ok()?;

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

        //let res = layouter.create_text(&text);
        let text_handle = layouter.create_multiline_text(&text, 0.6 / 0.05).ok()?;
        let photo_to = PhotoState {
            index: next_index,
            photo: photo_handle,
            text: text_handle,
        };

        self.tick_count = 0;
        self.index = next_index;
        self.state = if let SlideshowState::Static { photo } = &self.state {
            SlideshowState::Transitioning {
                photo_from: photo.clone(),
                photo_to: photo_to.clone(),
                duration: 40,
            }
        } else {
            SlideshowState::Static {
                photo: photo_to.clone(),
            }
        };

        Some(true)
    }

    // ------------------------------------------------------------------------
    fn finish_transition(&mut self, layouter: &mut Layouter) {
        log::info!("Slideshow: finishing transition");
        self.tick_count = 0;
        self.state = if let SlideshowState::Transitioning {
            photo_from,
            photo_to,
            ..
        } = &self.state
        {
            layouter.free_handle(photo_from.photo);
            layouter.free_handle(photo_from.text);
            SlideshowState::Static {
                photo: photo_to.clone(),
            }
        } else {
            self.state.clone()
        };
    }

    // ------------------------------------------------------------------------
    fn layout(&mut self, layouter: &mut Layouter) -> Option<Layout> {
        match &self.state {
            SlideshowState::Idle => None,
            SlideshowState::Static { photo } => self.static_layout(photo, layouter),
            SlideshowState::Transitioning {
                photo_from,
                photo_to,
                duration,
            } => self.transition_layout(photo_from, photo_to, duration, layouter),
        }
    }

    // ------------------------------------------------------------------------
    fn static_layout(&self, current: &PhotoState, layouter: &mut Layouter) -> Option<Layout> {
        let src_aspect = current.photo.aspect_ratio;
        let dst_aspect = layouter.aspect_ratio();
        let dst = place_photo(src_aspect, dst_aspect);

        let picture = Picture {
            dst,
            src: Rect {
                pos: V2::new([0.0, 0.0]),
                size: V2::new([1.0, 1.0]),
            },
            opacity: 1.0,
            handle: current.photo,
        };

        let text = Text {
            dst: Rect {
                pos: V2::new([0.025, 0.025]),
                size: V2::new([0.05, 0.05]),
            },
            color: V4::new([1.0, 1.0, 1.0, 1.0]),
            opacity: 1.0,
            handle: current.text,
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

        log::info!("Slideshow: static layout for index {}", current.index);

        Some(Layout { items })
    }

    // ------------------------------------------------------------------------
    fn transition_layout(
        &self,
        from: &PhotoState,
        to: &PhotoState,
        duration: &usize,
        layouter: &mut Layouter,
    ) -> Option<Layout> {
        let dst_aspect = layouter.aspect_ratio();
        let from_dst = place_photo(from.photo.aspect_ratio, dst_aspect);
        let to_dst = place_photo(to.photo.aspect_ratio, dst_aspect);
        let progress = (self.tick_count as f32 / *duration as f32).min(1.0);

        let transition = Transition {
            from_dst,
            from_src: Rect {
                pos: V2::new([0.0, 0.0]),
                size: V2::new([1.0, 1.0]),
            },
            to_dst,
            to_src: Rect {
                pos: V2::new([0.0, 0.0]),
                size: V2::new([1.0, 1.0]),
            },
            from: from.photo,
            to: to.photo,
            progress,
        };

        let items = vec![LayoutItem {
            id: LayoutId(0),
            element: Element::Transition(transition),
            animation_time: Some(0.5),
        }];

        log::info!(
            "Slideshow: transition progress {:.2} from index {} to index {}",
            progress,
            from.index,
            to.index
        );
        Some(Layout { items })
    }

    fn next_index(&self) -> usize {
        (self.index + 1) % self.photos.len()
    }

    fn prev_index(&self) -> usize {
        (self.index + self.photos.len() - 1) % self.photos.len()
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
                self.start_transition(0, ctx, layouter)?;
            }
            SceneEvent::TimeTick => {
                self.tick_count += 1;
                match &mut self.state {
                    SlideshowState::Transitioning { duration, .. } => {
                        if self.tick_count >= *duration {
                            self.finish_transition(layouter);
                        }
                    }
                    SlideshowState::Static { .. } => {
                        if self.tick_count >= 150 {
                            self.start_transition(self.next_index(), ctx, layouter);
                        }
                    }
                    _ => {}
                }
            }
            SceneEvent::User(UserEvent::Next) => {
                self.start_transition(self.next_index(), ctx, layouter);
            }

            SceneEvent::User(UserEvent::Previous) => {
                self.start_transition(self.prev_index(), ctx, layouter);
            }

            _ => {}
        }

        self.layout(layouter)
    }
}

// ----------------------------------------------------------------------------
fn place_photo(src_aspect: f32, dst_aspect: f32) -> Rect {
    if src_aspect > dst_aspect {
        // source is wider than destination
        let scaled_height = dst_aspect / src_aspect;
        let ofs_y = (1.0 - scaled_height) / 2.0;
        let pos = V2::new([0.0, ofs_y]);
        let size = V2::new([1.0, 1.0 * scaled_height]);
        Rect { pos, size }
    } else {
        // source is taller than destination
        let scaled_width = src_aspect / dst_aspect;
        let ofs_x = (1.0 - scaled_width) / 2.0;
        let pos = V2::new([ofs_x, 0.0]);
        let size = V2::new([scaled_width, 1.0]);
        Rect { pos, size }
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
