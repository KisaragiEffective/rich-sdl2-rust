use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    os::raw::c_int,
};

use self::{
    format::AudioFormat,
    spec::{AudioSpec, FallbackFlag},
    status::AudioStatus,
};
use crate::{bind, Result, Sdl, SdlError};

pub mod format;
pub mod spec;
pub mod status;

#[derive(Debug)]
pub struct AudioDeviceProperty {
    pub sample_freq: u32,
    pub format: AudioFormat,
    pub channels: u8,
    pub samples: u16,
}

pub trait AudioDevice {
    fn id(&self) -> u32;

    fn status(&self) -> AudioStatus {
        unsafe { bind::SDL_GetAudioDeviceStatus(self.id()) }.into()
    }
}

pub struct SpeakerDevice {
    id: u32,
}

impl SpeakerDevice {
    pub fn all_devices() -> impl Iterator<Item = String> {
        devices(false)
    }

    pub fn open(
        device: Option<String>,
        spec: &AudioSpec,
        fallback: FallbackFlag,
    ) -> Result<(Self, AudioDeviceProperty)> {
        let (id, prop) = open(false, device, spec, fallback)?;
        Ok((Self { id }, prop))
    }
}

impl AudioDevice for SpeakerDevice {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for SpeakerDevice {
    fn drop(&mut self) {
        unsafe { bind::SDL_CloseAudioDevice(self.id) }
    }
}

pub struct MicrophoneDevice {
    id: u32,
}

impl MicrophoneDevice {
    pub fn all_devices() -> impl Iterator<Item = String> {
        devices(true)
    }
}

impl AudioDevice for MicrophoneDevice {
    fn id(&self) -> u32 {
        self.id
    }
}

fn devices(is_capture: bool) -> impl Iterator<Item = String> {
    let is_capture_raw = if is_capture { 1 } else { 0 };
    let num_devices = unsafe {
        bind::SDL_InitSubSystem(bind::SDL_INIT_AUDIO);
        bind::SDL_GetNumAudioDevices(is_capture_raw)
    };
    (0..num_devices).map(move |index| {
        let cstr = unsafe { CStr::from_ptr(bind::SDL_GetAudioDeviceName(index, is_capture_raw)) };
        cstr.to_string_lossy().to_string()
    })
}

fn open(
    is_capture: bool,
    device: Option<String>,
    spec: &AudioSpec,
    fallback: FallbackFlag,
) -> Result<(u32, AudioDeviceProperty)> {
    let is_capture_raw = if is_capture { 1 } else { 0 };
    let mut actual = MaybeUninit::uninit();
    let id = unsafe {
        bind::SDL_OpenAudioDevice(
            device.map_or(std::ptr::null_mut(), |s| {
                let c_string = CString::new(s).unwrap();
                c_string.into_raw()
            }),
            is_capture_raw,
            spec.raw() as *const _,
            actual.as_mut_ptr(),
            fallback.bits() as c_int,
        )
    };
    if id == 0 {
        return Err(SdlError::Others { msg: Sdl::error() });
    }
    let actual = unsafe { actual.assume_init() };
    Ok((
        id,
        AudioDeviceProperty {
            sample_freq: actual.freq as u32,
            format: actual.format.into(),
            channels: actual.channels,
            samples: actual.samples,
        },
    ))
}
