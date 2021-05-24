use winit::{
    error::OsError,
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::graphics::{Gpu, Vertex};

fn create_window(title: impl AsRef<str>) -> Result<(EventLoop, Window), OsError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title.as_ref())
        .build(&event_loop);

    window.map(|window| (event_loop, window))
}
