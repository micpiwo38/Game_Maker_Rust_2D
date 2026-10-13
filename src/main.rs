use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::game_state::GameState;

mod game_state;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Game Maker Rust 2D")
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .build(&event_loop).unwrap();
    let mut state = pollster::block_on(GameState::new(&window));
    //Boucle infinie
    event_loop
        .run(move|event, control_flow|{
            control_flow.set_control_flow(winit::event_loop::ControlFlow::Poll);

            match event{
                Event::NewEvents(_) => {}
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window.id() => match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        event:
                        KeyEvent{
                            state: ElementState::Pressed,
                            logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                            ..
                        },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {}
                },
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::AboutToWait => {
                    state.window.request_redraw()
                }
                _ => {}
                Event::LoopExiting => {}
                Event::MemoryWarning => {}
            } {  }
        }).unwrap();
    println!("Hello, world!");
}
