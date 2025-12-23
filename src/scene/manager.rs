use crate::core::gl_canvas::Canvas;
use crate::error::Result;
use crate::scene::{
    Context, Layout, Layouter, Scene, SceneEvent, photo, slideshow::create_slideshow_all,
};
use crate::util::datetime::DateTime;
use std::path::Path;

pub struct SceneManager {
    scene: Option<Box<dyn Scene>>,
    context: Context,
    layouter: Layouter,
    layout: Layout,
}

impl SceneManager {
    pub fn new(layouter: Layouter, photo_dir: &Path) -> Result<Self> {
        let photos = photo::read_webp_photos(photo_dir);

        let mut layouter = layouter;

        let context = Context {
            photos,
            time: DateTime::now(),
            weather: None,
            locale: Box::new(crate::util::locale::LocaleUs {}),
        };

        let mut scene = create_slideshow_all(&context)
            .ok()
            .map(|s| Box::new(s) as Box<dyn Scene>);

        let mut layout = Layout::empty();
        update_scene(
            &mut scene,
            &SceneEvent::Enter,
            &context,
            &mut layouter,
            &mut layout,
        );

        Ok(Self {
            scene,
            context,
            layouter,
            layout,
        })
    }

    pub fn update(&mut self, event: &SceneEvent) {
        self.context.time = DateTime::now();
        update_scene(
            &mut self.scene,
            event,
            &self.context,
            &mut self.layouter,
            &mut self.layout,
        );
    }

    pub fn canvas(&self) -> &Canvas {
        self.layouter.canvas()
    }

    pub fn resize(&mut self, aspect_ratio: f32) {
        self.layouter.resize(aspect_ratio);
    }
}

fn update_scene(
    scene: &mut Option<Box<dyn Scene>>,
    event: &SceneEvent,
    ctx: &Context,
    layouter: &mut Layouter,
    layout: &mut Layout,
) {
    if let Some(scene) = scene.as_mut()
        && let Some(new_layout) = scene.update(event, ctx, layouter)
    {
        layout.replace(new_layout);
        layouter.update_layout(layout);
    }
}
