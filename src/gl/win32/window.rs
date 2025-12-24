use windows::Win32::UI::Input::HRAWINPUT;
use windows::Win32::{
    Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};
use windows::core::*;

pub const WM_GAMELOOP: u32 = WM_USER + 1;

pub fn loword(dword: u32) -> i32 {
    (dword & 0xffff) as i16 as i32
}

pub fn hiword(dword: u32) -> i32 {
    ((dword >> 16) & 0xffff) as i16 as i32
}

pub trait IWindow {
    type Params;
    fn create(
        hwnd: HWND,
        pos: POINT,
        size: SIZE,
        params: &Self::Params,
    ) -> crate::error::Result<Self>
    where
        Self: Sized;

    fn on_create(&mut self) -> LRESULT;
    fn on_destroy(&mut self) -> LRESULT;
    fn on_size(&mut self, cx: i32, cy: i32) -> LRESULT;
    fn on_loop(&mut self) -> LRESULT;
    fn on_key_event(&mut self, msg: u32, key: u32) -> LRESULT;
    fn on_mouse_event(&mut self, msg: u32, x: i32, y: i32, keys: u32, delta: i32) -> LRESULT;
    fn on_input(&mut self, _raw_input: HRAWINPUT) -> LRESULT;
}

pub struct WindowProc<T> {
    hwnd: HWND,
    data: Box<T>,
}

impl<T: IWindow> WindowProc<T> {
    pub fn create(
        title: &str,
        class_name: &str,
        style: WINDOW_STYLE,
        params: T::Params,
    ) -> Result<HWND> {
        let title = title.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let class_name = class_name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let h_instance = unsafe { GetModuleHandleW(None) }?;
        let h_cursor = unsafe { LoadCursorW(None, IDC_ARROW) }?;
        let hbr_background = unsafe { HBRUSH(GetStockObject(NULL_BRUSH).0) };

        let wc = WNDCLASSW {
            hCursor: h_cursor,
            hbrBackground: hbr_background,
            hInstance: h_instance.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            style: CS_OWNDC,
            lpfnWndProc: Some(Self::wndproc),
            ..Default::default()
        };

        unsafe { RegisterClassW(&wc) };

        let params = Box::new(params);

        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(title.as_ptr()),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                Some(h_instance.into()),
                Some(Box::into_raw(params) as *const core::ffi::c_void),
            )?
        };

        Ok(hwnd)
    }

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if msg == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTW;
                let params = (*cs).lpCreateParams as *mut T::Params;
                let create_data = Box::from_raw(params);

                let pos = POINT {
                    x: (*cs).x,
                    y: (*cs).y,
                };
                let size = SIZE {
                    cx: (*cs).cx,
                    cy: (*cs).cy,
                };
                let Ok(data) = T::create(hwnd, pos, size, &create_data) else {
                    return LRESULT(0);
                };
                let data = Box::new(data);

                let window = Box::new(WindowProc { hwnd, data });
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(window) as isize);
            }

            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowProc<T>;
            if let Some(data) = ptr.as_mut() {
                let result = data.handle_msg(msg, wparam, lparam);

                if msg == WM_NCDESTROY {
                    // remove the object from the window and delete it
                    SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);

                    let window = Box::from_raw(ptr);
                    drop(window); // make it obivous that the window is being deleted
                }
                result
            } else {
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
    }

    fn handle_msg(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_CREATE => self.data.on_create(),
            WM_DESTROY => self.data.on_destroy(),
            WM_SIZE => {
                let cx = loword(lparam.0 as u32);
                let cy = hiword(lparam.0 as u32);
                self.data.on_size(cx, cy)
            }
            WM_GAMELOOP => self.data.on_loop(),
            WM_KEYDOWN | WM_KEYUP => self.data.on_key_event(msg, wparam.0 as u32),
            WM_MOUSEMOVE | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONDOWN | WM_RBUTTONUP
            | WM_MBUTTONDOWN | WM_MBUTTONUP | WM_MOUSEWHEEL => {
                let x = loword(lparam.0 as u32);
                let y = hiword(lparam.0 as u32);
                let keys = (wparam.0 & 0xffff) as u32;
                let delta = hiword(wparam.0 as u32);
                self.data.on_mouse_event(msg, x, y, keys, delta)
            }
            WM_INPUT => {
                let raw_input = HRAWINPUT(lparam.0 as *mut core::ffi::c_void);
                self.data.on_input(raw_input)
            }
            _ => unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) },
        }
    }
}

pub fn run_message_loop(hwnd: HWND) {
    let mut msg = MSG::default();
    unsafe {
        loop {
            while PeekMessageA(&mut msg, None, 0, 0, PM_NOREMOVE).as_bool() {
                if !GetMessageA(&mut msg, None, 0, 0).as_bool() {
                    return;
                }
                if msg.message == WM_QUIT {
                    return;
                }
                let _ = TranslateMessage(&msg);
                let _ = DispatchMessageA(&msg);
            }
            SendMessageA(hwnd, WM_GAMELOOP, WPARAM(0), LPARAM(0));
        }
    }
}
