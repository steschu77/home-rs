use super::opengl::*;
use crate::error::{Error, Result};
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::{Foundation::*, Graphics::Gdi::*, Graphics::OpenGL::*};
use windows::core::*;

pub mod window;

const OPENGL32: &str = "opengl32.dll\0";

pub struct Win32GlContext {
    hwnd: HWND,
    hdc: HDC,
    hglrc: HGLRC,
}

impl Win32GlContext {
    pub fn from_hwnd(hwnd: HWND) -> Result<Self> {
        let hdc = unsafe { GetDC(Some(hwnd)) };

        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 24,
            cDepthBits: 32,
            iLayerType: PFD_MAIN_PLANE.0 as u8,
            ..Default::default()
        };

        let pf = unsafe { ChoosePixelFormat(hdc, &pfd) };
        let _ = unsafe { SetPixelFormat(hdc, pf, &pfd) };

        if let Ok(hglrc) = unsafe { wglCreateContext(hdc) }
            && unsafe { wglMakeCurrent(hdc, hglrc) }.is_ok()
        {
            Ok(Self { hwnd, hdc, hglrc })
        } else {
            Err(Error::OpenGlLoad {
                name: "wglCreateContext".into(),
            })
        }
    }

    pub fn load(&self) -> Result<OpenGlFunctions> {
        let opengl32 = unsafe { LoadLibraryA(PCSTR(OPENGL32.as_ptr())) };
        let Ok(opengl32) = opengl32 else {
            return Err(Error::OpenGlLoad {
                name: "opengl32.dll".into(),
            });
        };
        OpenGlFunctions::load(|fn_name| {
            let fn_ptr_ogl1 = unsafe { GetProcAddress(opengl32, PCSTR(fn_name.as_ptr())) };
            let fn_ptr_ogl2 = unsafe { wglGetProcAddress(PCSTR(fn_name.as_ptr())) };
            fn_ptr_ogl1.or(fn_ptr_ogl2).map(|f| f as FnOpenGl)
        })
    }

    pub fn swap_buffers(&self) {
        let _ = unsafe { SwapBuffers(self.hdc) };
    }
}

impl Drop for Win32GlContext {
    fn drop(&mut self) {
        let _ = unsafe { wglMakeCurrent(HDC::default(), HGLRC::default()) };
        let _ = unsafe { wglDeleteContext(self.hglrc) };
        unsafe { ReleaseDC(Some(self.hwnd), self.hdc) };
    }
}
