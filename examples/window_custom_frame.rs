use simple_logger::SimpleLogger;
use winit::{window::{WindowBuilder, WindowArea}, event_loop::EventLoop, event::{StartCause, Event, WindowEvent}, platform::windows::WindowBuilderExtWindows};

fn hittest(area: WindowArea, x: i32, y: i32) -> WindowArea {
    let hits = [
                    WindowArea::NOWHERE,
                    WindowArea::RIGHT,
                    WindowArea::LEFT,
                    WindowArea::TOPLEFT,
                    WindowArea::TOP,
                    WindowArea::TOPRIGHT,
                    WindowArea::BOTTOMRIGHT,
                    WindowArea::BOTTOM,
                    WindowArea::BOTTOMLEFT,
                    WindowArea::LEFT,
                ];
    if hits.contains(&area) {
        return area;
    }

    if y < 30 {
        return WindowArea::MAXBUTTON
    }
    WindowArea::CAPTION
}

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(400.0, 400.0))
        .with_hittest_fn(hittest)
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::NewEvents(StartCause::Init) => {

            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            _ => (),
        }
    })
}