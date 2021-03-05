use crate::bind;
use crate::color::Rgb;

use super::Surface;

pub struct Color<S> {
    surface: S,
    color: Rgb,
}

impl<S> Color<S> {
    pub fn color(&self) -> &Rgb {
        &self.color
    }
}

impl<S: Surface> Color<S> {
    pub(super) fn new(surface: S, color: Rgb) -> Self {
        unsafe {
            let _ =
                bind::SDL_SetSurfaceColorMod(surface.as_ptr().as_ptr(), color.r, color.g, color.b);
        }
        Self { surface, color }
    }
}

impl<S: Surface> Surface for Color<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface.as_ptr()
    }
}
