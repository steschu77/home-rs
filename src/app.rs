use crate::core::IApp;
use crate::core::gl_canvas::Canvas;
use crate::core::gl_renderer::Renderer;
use crate::core::input::Input;
use crate::error::Result;
use crate::gl::opengl::OpenGlFunctions;
use crate::scene::{layouter::Layouter, manager::SceneManager};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub photo_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            photo_dir: PathBuf::from("assets/photos/"),
        }
    }
}

pub struct App {
    config: AppConfig,
    renderer: Renderer,
    scenes: SceneManager,
}

impl App {
    pub fn new(config: AppConfig, gl: OpenGlFunctions, cx: i32, cy: i32) -> Result<Self> {
        let gl = Rc::new(gl);
        let aspect_ratio = cx as f32 / cy as f32;
        let canvas = Canvas::new(Rc::clone(&gl), aspect_ratio)?;
        let layouter = Layouter::new(canvas)?;
        let scenes = SceneManager::new(layouter, &config.photo_dir)?;

        Ok(Self {
            config,
            renderer: Renderer::new(gl)?,
            scenes,
        })
    }

    pub fn resize(&mut self, cx: i32, cy: i32) {
        let aspect_ratio = cx as f32 / cy as f32;
        self.renderer.resize(cx, cy);
        self.scenes.resize(aspect_ratio);
    }
}

impl IApp for App {
    fn update(
        &mut self,
        _t: std::time::Instant,
        _dt: std::time::Duration,
        _input: &mut Input,
    ) -> Result<()> {
        self.scenes.update(&crate::scene::SceneEvent::TimeTick);
        Ok(())
    }

    fn render(&mut self, _t: &std::time::Instant) -> Result<()> {
        //let camera = camera::Camera::new([0.0, 0.0, 0.0, 1.0].into(), 1.0);
        self.renderer.render(self.scenes.canvas())?;
        Ok(())
    }
}
