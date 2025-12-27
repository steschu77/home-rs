mod app;
mod core;
mod error;
mod gfx;
mod gl;
mod scene;
mod util;
mod v2d;

// ----------------------------------------------------------------------------
#[cfg(target_os = "windows")]
pub fn main() {
    if let Err(e) = win32::main() {
        eprintln!("Error: {e:?}");
    }
}

// ----------------------------------------------------------------------------
#[cfg(target_os = "linux")]
pub fn main() {
    if let Err(e) = linux::main() {
        eprintln!("Error: {e:?}");
    }
}

// ----------------------------------------------------------------------------
#[cfg(target_os = "windows")]
mod win32 {
    use crate::app::App;
    use crate::core::app_loop::AppLoop;
    use crate::core::clock::Clock;
    use crate::core::input::{self, Key};
    use crate::error::{Error, Result};
    use crate::gl::win32::Win32GlContext;
    use crate::gl::win32::window::{IWindow, WindowProc};
    use windows::Win32::UI::Input::{
        GetRawInputData, HRAWINPUT, KeyboardAndMouse, RAWINPUT, RAWINPUTHEADER, RID_INPUT,
        RIM_TYPEKEYBOARD, RIM_TYPEMOUSE,
    };
    use windows::Win32::{
        Foundation::*,
        UI::Input::{RAWINPUTDEVICE, RIDEV_INPUTSINK, RegisterRawInputDevices},
        UI::WindowsAndMessaging::*,
    };

    // ------------------------------------------------------------------------
    pub fn main() -> Result<()> {
        let cfg = super::init()?;
        let hwnd = WindowProc::<AppWindow>::create(
            "Home",
            "AppWindow",
            WS_POPUP | WS_VISIBLE,
            AppWindowParams { cfg },
        );

        if let Ok(hwnd) = hwnd {
            crate::gl::win32::window::run_message_loop(hwnd);
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    struct AppWindowParams {
        cfg: super::AppConfig,
    }

    // ------------------------------------------------------------------------
    struct AppWindow {
        clock: Clock,
        win32: Win32GlContext,
        input: input::Input,
        app_loop: AppLoop,
        app: App,
    }

    // ------------------------------------------------------------------------
    impl IWindow for AppWindow {
        type Params = AppWindowParams;
        fn create(hwnd: HWND, _pos: POINT, size: SIZE, params: &AppWindowParams) -> Result<Self> {
            let rid_mouse = RAWINPUTDEVICE {
                usUsagePage: 0x01,
                usUsage: 0x02, // Mouse
                dwFlags: RIDEV_INPUTSINK,
                hwndTarget: hwnd,
            };
            let rid_keyboard = RAWINPUTDEVICE {
                usUsagePage: 0x01,
                usUsage: 0x06, // Keyboard
                dwFlags: RIDEV_INPUTSINK,
                hwndTarget: hwnd,
            };
            unsafe {
                RegisterRawInputDevices(
                    &[rid_mouse, rid_keyboard],
                    size_of::<RAWINPUTDEVICE>() as u32,
                )
                .map_err(Error::from)?
            };

            let t_update = std::time::Duration::from_millis(10);
            let win32 = Win32GlContext::from_hwnd(hwnd)?;
            let app_loop = AppLoop::new(t_update);
            let gl = win32.load()?;
            let app = App::new(params.cfg.clone(), gl, size.cx, size.cy)?;

            Ok(Self {
                clock: Clock::new(),
                win32,
                input: input::Input::new(),
                app_loop,
                app,
            })
        }

        fn on_create(&mut self) -> LRESULT {
            LRESULT(0)
        }

        fn on_destroy(&mut self) -> LRESULT {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }

        fn on_size(&mut self, cx: i32, cy: i32) -> LRESULT {
            self.app.resize(cx, cy);
            LRESULT(0)
        }

        fn on_loop(&mut self) -> LRESULT {
            if let Err(e) = self
                .app_loop
                .step(&mut self.app, &self.clock, &mut self.input)
            {
                eprintln!("Home loop exited with: {e:?}");
                unsafe { PostQuitMessage(0) };
                return LRESULT(0);
            }

            self.win32.swap_buffers();
            LRESULT(0)
        }

        fn on_key_event(&mut self, msg: u32, vk: u32) -> LRESULT {
            if let Some(key) = vk_to_key(vk) {
                match msg {
                    WM_KEYDOWN => self.input.add_event(input::Event::KeyDown { key }),
                    WM_KEYUP => self.input.add_event(input::Event::KeyUp { key }),
                    _ => {}
                }
            }
            LRESULT(0)
        }

        fn on_mouse_event(
            &mut self,
            msg: u32,
            _x: i32,
            _y: i32,
            _keys: u32,
            delta: i32,
        ) -> LRESULT {
            match msg {
                WM_MOUSEWHEEL => self.input.add_event(input::Event::Wheel { delta }),
                WM_LBUTTONDOWN => self.input.add_event(input::Event::ButtonDown { button: 1 }),
                WM_LBUTTONUP => self.input.add_event(input::Event::ButtonUp { button: 1 }),
                WM_RBUTTONDOWN => self.input.add_event(input::Event::ButtonDown { button: 2 }),
                WM_RBUTTONUP => self.input.add_event(input::Event::ButtonUp { button: 2 }),
                WM_MBUTTONDOWN => self.input.add_event(input::Event::ButtonDown { button: 3 }),
                WM_MBUTTONUP => self.input.add_event(input::Event::ButtonUp { button: 3 }),
                _ => {}
            }
            LRESULT(0)
        }

        fn on_input(&mut self, raw_input: HRAWINPUT) -> LRESULT {
            let mut data_size = 0u32;
            unsafe {
                GetRawInputData(
                    raw_input,
                    RID_INPUT,
                    None,
                    &mut data_size,
                    size_of::<RAWINPUTHEADER>() as u32,
                );
            }

            let mut raw_input_bytes = vec![0u8; data_size as usize];
            unsafe {
                GetRawInputData(
                    raw_input,
                    RID_INPUT,
                    Some(raw_input_bytes.as_mut_ptr() as *mut _),
                    &mut data_size,
                    size_of::<RAWINPUTHEADER>() as u32,
                )
            };

            unsafe {
                let raw: &RAWINPUT = &*(raw_input_bytes.as_ptr() as *const RAWINPUT);
                if raw.header.dwType == RIM_TYPEMOUSE.0 {
                    let mouse = raw.data.mouse;
                    if (mouse.lLastX != 0) || (mouse.lLastY != 0) {
                        self.input.add_event(input::Event::MouseMove {
                            x: mouse.lLastX,
                            y: mouse.lLastY,
                        });
                    }
                }
                if raw.header.dwType == RIM_TYPEKEYBOARD.0 {
                    let kb = raw.data.keyboard;
                    let vk = kb.VKey as u32;

                    if let Some(key) = vk_to_key(vk) {
                        match kb.Message {
                            WM_KEYDOWN | WM_SYSKEYDOWN => {
                                self.input.add_event(input::Event::KeyDown { key })
                            }
                            WM_KEYUP | WM_SYSKEYUP => {
                                self.input.add_event(input::Event::KeyUp { key })
                            }
                            _ => {}
                        }
                    }
                }
            }
            LRESULT(0)
        }
    }

    // ------------------------------------------------------------------------
    fn vk_to_key(vk: u32) -> Option<Key> {
        const VK_ESCAPE: u32 = KeyboardAndMouse::VK_ESCAPE.0 as u32;
        const VK_LEFT: u32 = KeyboardAndMouse::VK_LEFT.0 as u32;
        const VK_RIGHT: u32 = KeyboardAndMouse::VK_RIGHT.0 as u32;
        const VK_HOME: u32 = KeyboardAndMouse::VK_HOME.0 as u32;

        match vk {
            VK_ESCAPE => Some(Key::Exit),
            VK_LEFT => Some(Key::PrevScene),
            VK_RIGHT => Some(Key::NextScene),
            VK_HOME => Some(Key::Home),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------
#[cfg(target_os = "linux")]
mod linux {
    use crate::error::Result;
    use crate::app::App;
    use crate::core::app_loop::AppLoop;
    use crate::core::clock::Clock;
    use crate::core::input::{self, Event, Key};
    use crate::gl::linux::LinuxGLContext;

    pub fn main() -> Result<()> {
        let cfg = super::init()?;

        let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
        let screen = unsafe { x11::xlib::XDefaultScreen(display) };
        let root = unsafe { x11::xlib::XRootWindow(display, screen) };

        let cx = 800;
        let cy = 600;
        let win = unsafe { x11::xlib::XCreateSimpleWindow(display, root, 0, 0, cx, cy, 0, 0, 0) };

        unsafe {
            x11::xlib::XSelectInput(
                display,
                win,
                x11::xlib::ExposureMask | x11::xlib::KeyPressMask,
            );
            x11::xlib::XMapWindow(display, win);
        }

        let context = LinuxGLContext::from_window(display, screen, win)?;
        let gl = context.load()?;
        let clock = Clock::new();

        let t_update = std::time::Duration::from_millis(10);
        let mut app_loop = AppLoop::new(t_update);
        let mut app = App::new(cfg, gl, cx as i32, cy as i32)?;
        let mut input = input::Input::new();

        loop {
            while unsafe { x11::xlib::XPending(display) } > 0 {
                let mut event: x11::xlib::XEvent = unsafe { std::mem::zeroed() };
                unsafe { x11::xlib::XNextEvent(display, &mut event) };
                    
                match unsafe { event.type_ } {
                    x11::xlib::Expose => {
                    }
                    x11::xlib::KeyPress => {
                        let key_event = unsafe { event.key };
                        if let Some(key) = xkey_to_key(key_event.keycode) {
                            input.add_event(Event::KeyDown{ key });
                        }
                    }
                    x11::xlib::ClientMessage => {
                        unsafe {
                            x11::xlib::XDestroyWindow(display, win);
                            x11::xlib::XCloseDisplay(display);
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }
            
            if let Err(e) = app_loop.step(&mut app, &clock, &mut input) {
                eprintln!("Home loop exited with: {e:?}");
                unsafe {
                    x11::xlib::XDestroyWindow(display, win);
                    x11::xlib::XCloseDisplay(display);
                }
                return Ok(());
            }
            
            context.swap_buffers();
        }
    }
    
    fn xkey_to_key(keycode: u32) -> Option<Key> {
        match keycode {
            9 => Some(Key::Exit),        // ESC
            110 => Some(Key::Home),      // Home
            113 => Some(Key::PrevScene), // Left arrow
            114 => Some(Key::NextScene), // Right arrow
            _ => None,
        }
    }    
}

use crate::app::AppConfig;
use crate::error::{Error, Result};
use crate::util::logger;
use std::{env, path::PathBuf};

// ----------------------------------------------------------------------------
fn init() -> Result<AppConfig> {
    let _ = logger::init_logger(log::LevelFilter::Info);

    let mut config = AppConfig::default();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        #[allow(clippy::single_match)]
        match arg.as_str() {
            "--photo-dir" => {
                if let Some(dir) = args.next() {
                    config.photo_dir = PathBuf::from(dir);
                }
            }
            _ => {
                return Err(Error::InvalidArgument { arg });
            }
        }
    }

    Ok(config)
}
