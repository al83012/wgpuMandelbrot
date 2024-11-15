mod state;
mod pixel;
mod compute;
mod camera;

use self::state::State;

fn main() {
    pollster::block_on(run());
}

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("WGPU Mandelbrot")
        .build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    let mut input_helper = winit_input_helper::WinitInputHelper::new();

    event_loop.run(move |event, control_flow| {
        input_helper.update(&event);
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() =>  // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(new_size) => {
                        state.resize(*new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        state.window().request_redraw();

                        /*if !surface_configured {
                            return;
                        }*/

                        state.update(&input_helper);
                        match state.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(
                                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                            ) => state.resize(state.size),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("OutOfMemory");
                                control_flow.exit();
                            }

                            // This happens when the a frame takes too long to present
                            Err(wgpu::SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                    }
                    _ => {}

            }
            _ => {}
        }
    }).expect("event loop run failed");
}


