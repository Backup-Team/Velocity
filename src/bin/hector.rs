use velocity::{
    graphics::{create_renderer, Pipeline, Vertex},
    maths::Vec3,
    window::{create_window, ControlFlow, Event, WindowEvent},
};

static VERTICES: &[Vertex] = &[
    Vertex {
        position: Vec3::new(0.5, 0.75, 0.0),
        colour:   Vec3::unit_x(),
    },
    Vertex {
        position: Vec3::new(-0.75, -0.75, 0.0),
        colour:   Vec3::unit_y(),
    },
    Vertex {
        position: Vec3::new(0.75, -0.75, 0.0),
        colour:   Vec3::unit_z(),
    },
];

#[tokio::main]
async fn main() {
    let (event_loop, window) = create_window("demo").expect("Unable to open window");

    let mut renderer = create_renderer(&window)
        .await
        .expect("Unable to create renderer");

    let pipeline = Pipeline::new(&renderer);
    let buffer = renderer.create_buffer(VERTICES);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => {
                renderer.handle_event(&event);

                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => {},
                    WindowEvent::ModifiersChanged(_) => {},
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {},
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {},

                    _ => {},
                }
            },

            Event::MainEventsCleared => {
                if let Err(err) = renderer.render(&pipeline, &buffer) {
                    *control_flow = err;
                }
            },

            _ => {},
        }
    });
}
