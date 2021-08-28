use velocity::{
    app::{App, LoopState},
    core::maths::{Angle, Vec3},
    engine::{Engine, EngineSettings},
    graphics::{Buffer, Camera, Pipeline, Renderer, Vertex},
    keyboard::Keyboard,
    mouse::Mouse,
    Bump,
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
    Vertex {
        position: Vec3::new(0.5, 0.75, 0.0),
        colour:   Vec3::unit_x(),
    },
    Vertex {
        position: Vec3::new(-0.75, 0.75, 0.0),
        colour:   Vec3::unit_y(),
    },
    Vertex {
        position: Vec3::new(-0.75, -0.75, 0.0),
        colour:   Vec3::unit_z(),
    },
];

struct Demo {
    pipeline: Pipeline,
    buffer:   Buffer,
    camera:   Camera,
}

impl Demo {
    pub fn new(renderer: &mut Renderer) -> Self {
        let pipeline = Pipeline::new(&renderer);
        let buffer = renderer.create_buffer(VERTICES);
        let camera = Camera::perspective(0.0, Angle::degrees(75.0), 0.1, 1000.0);

        Self {
            pipeline,
            buffer,
            camera,
        }
    }
}

impl App for Demo {
    const EXIT_ON_CLOSE: bool = true;

    fn update_and_render(
        &mut self,
        _keyboard: &Keyboard,
        _mouse: &Mouse,
        frame_memory: &mut Bump,
        renderer: &mut Renderer,
    ) -> LoopState {
        if let Err(err) = renderer.render(&self.pipeline, &self.buffer) {
            LoopState::Exit
        } else {
            LoopState::Continue
        }
    }
}

#[tokio::main]
async fn main() {
    let mut engine = Engine::new("demo", EngineSettings::default_game())
        .await
        .expect("Unable to instatiate engine instance");

    engine.run(Demo::new);
}
