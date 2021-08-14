use crate::{bind, Result, Sdl, SdlError};
use std::ffi::CString;

use super::Window;

pub use self::{
    button::{Button, ButtonId, ButtonKind},
    color_scheme::ColorScheme,
};

mod button;
mod color_scheme;

#[derive(Debug)]
pub enum MessageBoxKind {
    Error,
    Warning,
    Information,
}

#[derive(Debug)]
pub struct MessageBox {
    kind: MessageBoxKind,
    title: CString,
    message: CString,
    buttons: Vec<Button>,
    color_scheme: Option<ColorScheme>,
}

impl MessageBox {
    pub fn new(kind: MessageBoxKind) -> Self {
        Self {
            kind,
            title: CString::default(),
            message: CString::default(),
            buttons: vec![],
            color_scheme: None,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = CString::new(title).unwrap();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = CString::new(message).unwrap();
        self
    }

    pub fn add_button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn color_scheme(mut self, scheme: Option<ColorScheme>) -> Self {
        self.color_scheme = scheme;
        self
    }

    pub fn show(self, parent: &'_ Window<'_>) -> Result<ButtonId> {
        let title_cstr = CString::new(self.title).unwrap_or_default();
        let message_cstr = CString::new(self.message).unwrap_or_default();
        let buttons_raw: Vec<_> = self.buttons.iter().map(|button| button.as_raw()).collect();
        let color_scheme = self.color_scheme.map(|scheme| scheme.into());
        let data = bind::SDL_MessageBoxData {
            flags: 0,
            window: parent.as_ptr(),
            title: title_cstr.as_ptr(),
            message: message_cstr.as_ptr(),
            numbuttons: buttons_raw.len() as i32,
            buttons: buttons_raw.as_ptr(),
            colorScheme: color_scheme
                .as_ref()
                .map_or(std::ptr::null(), |scheme| scheme as *const _),
        };
        let mut button_id = 0;
        let ret = unsafe { bind::SDL_ShowMessageBox(&data as *const _, &mut button_id as *mut _) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(button_id)
    }
}
