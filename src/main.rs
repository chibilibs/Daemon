use inferno::{
    app::{InfernoConfig, InfernoGame, InfernoWindow},
    window::{WindowEventHandler, WindowEvents, WindowHandler},
};
use winit::event::*;

struct Daemon {
    config: InfernoConfig,
}

impl InfernoGame for Daemon {}

fn main() {
    let daemon = Daemon {
        config: InfernoConfig::new(String::from("Inferno Game"), (1280, 720)),
    };

    Daemon::new(&daemon.config);

    let window = InfernoWindow::new(&daemon.config);

    window
        .event_loop
        .run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.window.id() => {
                if let Some(window_event) = InfernoWindow::handle_window_event(&event) {
                    match window_event {
                        WindowEvents::KeyPressed(VirtualKeyCode::Escape) => {
                            InfernoWindow::handle_key_press(VirtualKeyCode::Escape, control_flow)
                        }
                        WindowEvents::Close => InfernoWindow::handle_close(control_flow),
                        _ => (),
                    }
                }
            }
            _ => {}
        });
}
