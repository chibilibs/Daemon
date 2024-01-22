use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::WindowBuilder,
};

use crate::app::{InfernoConfig, InfernoWindow};

pub enum WindowEvents {
    Resize(u32, u32),
    Close,
    KeyPressed(VirtualKeyCode),
}

pub trait WindowHandler {
    fn new(config: &InfernoConfig) -> InfernoWindow {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build();
        let window = WindowBuilder::new()
            .with_title(&config.window_title)
            .build(&event_loop)
            .unwrap();

        InfernoWindow { event_loop, window }
    }
}

pub trait WindowEventHandler {
    fn handle_close(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Exit
    }

    fn handle_key_press(key_code: VirtualKeyCode, control_flow: &mut ControlFlow) {
        // TODO! do a better implementation
        if key_code == VirtualKeyCode::Escape {
            *control_flow = ControlFlow::Exit;
        }
    }

    fn handle_window_event(event: &WindowEvent) -> Option<WindowEvents> {
        match event {
            WindowEvent::CloseRequested => Some(WindowEvents::Close),
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode,
                        ..
                    },
                ..
            } => {
                if let Some(key_code) = virtual_keycode {
                    if *state == ElementState::Pressed {
                        return Some(WindowEvents::KeyPressed(*key_code));
                    }
                }
                None
            }
            _ => None,
        }
    }
}
