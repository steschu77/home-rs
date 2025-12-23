pub mod opengl;

#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_os = "linux")]
pub mod linux;
