use tracing::info;
use winit::{event_loop::EventLoop, window::Window};

use crate::window::{WindowEventHandler, WindowHandler};

#[derive(Debug)]
pub struct InfernoConfig {
    pub window_title: String,
    /// width, height
    pub window_size: (u32, u32),
}

impl InfernoConfig {
    pub fn new(window_title: String, window_size: (u32, u32)) -> Self {
        Self {
            window_title,
            window_size,
        }
    }
}

pub struct InfernoWindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl WindowHandler for InfernoWindow {}
impl WindowEventHandler for InfernoWindow {}

pub trait InfernoGame {
    fn new(config: &InfernoConfig) {
        tracing_subscriber::fmt().init();

        info!("CONFIG: {:?}", config)
    }
}
