use crate::util::datetime::DateTime;
use crate::util::locale::DateLocale;
use crate::v2d::{v2::V2, v4::V4};
use layouter::Layouter;
use photo::Photo;

pub mod font;
pub mod layouter;
pub mod manager;
pub mod photo;
pub mod slideshow;

pub trait Scene {
    fn update(
        &mut self,
        event: &SceneEvent,
        ctx: &Context,
        layouter: &mut Layouter,
    ) -> Option<Layout>;
}

#[derive(Clone, Debug)]
pub enum SceneEvent {
    Enter,
    Exit,
    TimeTick,
    User(UserEvent),
    System(SystemEvent),
}

#[derive(Clone, Debug)]
pub enum UserEvent {
    Home,
    Exit,
    Next,
    Previous,
}

#[derive(Clone, Debug)]
pub enum SystemEvent {
    WeatherUpdate,
    Alarm,
}

pub struct Layout {
    pub items: Vec<LayoutItem>,
}

impl Layout {
    pub fn empty() -> Self {
        Self { items: vec![] }
    }

    pub fn replace(&mut self, other: Layout) {
        self.items = other.items;
    }
}

pub struct Context {
    pub photos: Vec<Photo>,
    pub time: DateTime,
    pub weather: Option<Weather>,
    pub locale: Box<dyn DateLocale>,
}

impl Context {
    pub fn find_photo(&self, id: usize) -> Option<&Photo> {
        self.photos.get(id)
    }
}

#[derive(Clone, Debug)]
pub struct Weather {
    pub temperature: f32,
    pub condition_icon: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LayoutId(pub u32);

#[derive(Clone, Debug)]
pub struct LayoutItem {
    pub id: LayoutId,
    pub element: Element,
    pub animation_time: Option<f32>,
}

#[derive(Clone, Debug)]
pub enum Element {
    Picture(Picture),
    Thumbnail(Picture),
    Icon(Icon),
    Text(Text),
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub pos: V2,
    pub size: V2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Handle {
    pub material_id: Option<usize>,
    pub mesh_id: Option<usize>,
    pub aspect_ratio: f32,
}

#[derive(Clone, Debug)]
pub struct Picture {
    pub dst: Rect,
    pub src: Rect,
    pub opacity: f32,
    pub handle: Handle,
}

#[derive(Clone, Debug)]
pub struct Icon {
    pub dst: Rect,
    pub opacity: f32,
    pub color: V4,
    pub handle: Handle,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub dst: Rect,
    pub opacity: f32,
    pub color: V4,
    pub handle: Handle,
}
