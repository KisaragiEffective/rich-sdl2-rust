use std::ptr::NonNull;

use crate::bind;

pub struct Sensor {
    ptr: NonNull<bind::SDL_Sensor>,
}

pub struct SensorSet(Vec<Sensor>);

impl SensorSet {
    pub fn new() -> Self {
        let sensor_count = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_SENSOR);
            bind::SDL_NumSensors()
        };
        Self(
            (0..sensor_count)
                .map(|index| {
                    let sensor = unsafe { bind::SDL_SensorOpen(index) };
                    Sensor {
                        ptr: NonNull::new(sensor).expect("opening sensor failed"),
                    }
                })
                .collect(),
        )
    }

    pub fn sensors(&self) -> &[Sensor] {
        &self.0
    }
}

impl Drop for SensorSet {
    fn drop(&mut self) {
        for sensor in &self.0 {
            unsafe { bind::SDL_SensorClose(sensor.ptr.as_ptr()) }
        }
    }
}
