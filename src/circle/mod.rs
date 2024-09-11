use std::cell::Cell;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub struct Circle {
    center_x: f32,
    center_y: f32,
    radius: f32,
    step: f32, // Adjust the step size for smoother or faster motion
    angle: Cell<f32>,
    pub wait_time: u64, // Milliseconds between each step
}

impl Circle {
    pub fn new() -> Self {
        let screen_width;
        let screen_height;
        unsafe {
            screen_width = GetSystemMetrics(SM_CXSCREEN);
            screen_height = GetSystemMetrics(SM_CYSCREEN);
        }
        let new_circle = Self {
            center_x: screen_width as f32 / 2f32,
            center_y: screen_height as f32 / 2f32,
            radius: 50f32,
            step: 0.09, // Adjust the step size for smoother or faster motion
            angle: Cell::new(0.0),
            wait_time: 50.0 as u64, // Milliseconds between each step
        };
        new_circle
    }
    pub fn get_x_coord(&self) -> i32 {
        let x = (self.center_x + self.radius * self.angle.get().cos()) as i32;
        x
    }

    pub fn get_y_coord(&self) -> i32 {
        let y = (self.center_y + self.radius * self.angle.get().sin()) as i32;
        y
    }

    pub fn increase_angle(&self) {
        let new_angle_value = self.angle.get() + self.step;
        self.angle.set(new_angle_value);
    }
}