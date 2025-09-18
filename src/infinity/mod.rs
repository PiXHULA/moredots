use std::cell::Cell;
use std::f64::consts::PI;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub struct Infinity {
    center_x: f32,
    center_y: f32,
    scale: f64,
    points: f32,
    step: Cell<f64>,
    pub wait_time: u64, // Milliseconds between each step
}

impl Infinity {
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
            scale: 1.0,
            points: 100f32,
            step: Cell::new(1.0),
            wait_time: 50, // Milliseconds between each step
        };
        new_circle
    }

    pub fn get_x_coord(&self) -> f64 {
        let t = 2.0 * PI * (self.step.get()) / (self.points as f64);
        let x = self.center_x as f64 + (self.scale * t.cos()) / (1.0 + t.sin().powi(2));
        println!("t:{}, x:{}", t, x);
        x
    }

    pub fn get_y_coord(&self) -> f64 {
        let t = 2.0 * PI * (self.step.get()) / (self.points as f64);
        let y = self.center_y as f64 +  (self.scale * t.sin() * t.cos()) / (1.0 + t.sin().powi(2));
        println!("t:{}, y:{} step:{}", t, y, self.step.get());
        y
    }

    pub fn increase_step(&self) {
        let new_step_value = self.step.get() + 1.0;
        self.step.set(new_step_value);
    }
}