use static_assertions::assert_not_impl_all;
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use crate::{bind, Result, Sdl, SdlError};

use self::{axis::Axis, button::Button, map::MapInput};

pub mod axis;
pub mod button;
pub mod event;
pub mod map;

pub struct GameController {
    pub(in crate::event) ptr: NonNull<bind::SDL_GameController>,
}

impl std::fmt::Debug for GameController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameController")
            .field("name", &self.name())
            .finish()
    }
}

assert_not_impl_all!(GameController: Send, Sync);

impl GameController {
    pub fn mapping(&self) -> String {
        let ptr = unsafe { bind::SDL_GameControllerMapping(self.ptr.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let ret = cstr.to_string_lossy().to_string();
        unsafe { bind::SDL_free(ptr.cast()) };
        ret
    }

    pub fn name(&self) -> String {
        let ptr = unsafe { bind::SDL_GameControllerName(self.ptr.as_ptr()) };
        if ptr.is_null() {
            return "".into();
        }
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_string_lossy().to_string()
    }

    pub fn bind_for_axis(&self, axis: Axis) -> Option<MapInput> {
        let ret =
            unsafe { bind::SDL_GameControllerGetBindForAxis(self.ptr.as_ptr(), axis.as_raw()) };
        (ret.bindType != bind::SDL_GameControllerBindType_SDL_CONTROLLER_BINDTYPE_NONE)
            .then(|| ret.into())
    }

    pub fn bind_for_button(&self, button: Button) -> Option<MapInput> {
        let ret =
            unsafe { bind::SDL_GameControllerGetBindForButton(self.ptr.as_ptr(), button.as_raw()) };
        (ret.bindType != bind::SDL_GameControllerBindType_SDL_CONTROLLER_BINDTYPE_NONE)
            .then(|| ret.into())
    }
}

#[derive(Debug)]
pub struct GameControllerSet {
    controls: Vec<GameController>,
}

impl GameControllerSet {
    pub fn new() -> Self {
        let num_controls = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_JOYSTICK);
            bind::SDL_NumJoysticks()
        };
        let controls = (0..num_controls)
            .filter(|&index| unsafe { bind::SDL_IsGameController(index) != 0 })
            .filter_map(|index| {
                let raw = unsafe { bind::SDL_GameControllerOpen(index) };
                NonNull::new(raw)
            })
            .map(|ptr| GameController { ptr })
            .collect();
        Self { controls }
    }

    pub fn add_mapping(string: &str) -> Result<bool> {
        let cstr = CString::new(string).expect("string must not be empty");
        let ret = unsafe { bind::SDL_GameControllerAddMapping(cstr.as_ptr()) };
        if ret == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret == 1)
        }
    }

    pub fn add_mapping_from_file(file_name: &str) -> Result<u32> {
        let cstr = CString::new(file_name).expect("string must not be empty");
        let read_binary_mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let ret = unsafe {
            bind::SDL_GameControllerAddMappingsFromRW(
                bind::SDL_RWFromFile(cstr.as_ptr(), read_binary_mode.as_ptr()),
                1,
            )
        };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as u32)
        }
    }

    pub fn controllers(&self) -> &[GameController] {
        &self.controls
    }
}

impl Drop for GameControllerSet {
    fn drop(&mut self) {
        for control in &mut self.controls {
            unsafe { bind::SDL_GameControllerClose(control.ptr.as_ptr()) }
        }
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_JOYSTICK) }
    }
}
