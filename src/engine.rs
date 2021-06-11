use bumpalo::Bump;
use wgpu::PowerPreference;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{app::App, graphics::Renderer, keyboard::Keyboard, mouse::Mouse};

pub struct EngineSettings {
    pub power_preference: PowerPreference,
    pub control_flow:     ControlFlow,
    pub resizable_window: bool,
}

impl EngineSettings {
    pub fn default_game() -> Self {
        Self {
            power_preference: PowerPreference::HighPerformance,
            control_flow:     ControlFlow::Poll,
            resizable_window: false,
        }
    }

    pub fn default_app() -> Self {
        Self {
            power_preference: PowerPreference::LowPower,
            control_flow:     ControlFlow::Wait,
            resizable_window: true,
        }
    }
}

pub struct Engine<UserEvent = ()>
where
    UserEvent: 'static,
{
    event_loop:           EventLoop<UserEvent>,
    window:               Window,
    renderer:             Renderer,
    default_control_flow: ControlFlow,
}

impl<UserEvent> Engine<UserEvent>
where
    UserEvent: 'static,
{
    pub async fn new(title: impl AsRef<str>, settings: EngineSettings) -> Result<Self, ()> {
        let event_loop = EventLoop::<UserEvent>::with_user_event();
        let window = WindowBuilder::new()
            .with_title(title.as_ref())
            .with_resizable(settings.resizable_window)
            .build(&event_loop)
            .unwrap();

        // window.map(|window| (event_loop, window))
        let renderer = Renderer::new(&window, settings.power_preference)
            .await
            .unwrap();

        Ok(Self {
            event_loop,
            window,
            renderer,
            default_control_flow: settings.control_flow,
        })
    }

    #[inline(always)]
    pub fn run<A, I>(self, init: I) -> !
    where
        A: App<UserEvent> + 'static,
        I: FnOnce(&mut Renderer) -> A,
    {
        let Engine {
            window,
            event_loop,
            default_control_flow,
            mut renderer,
        } = self;

        let mut game = init(&mut renderer);
        let mut keyboard = Keyboard;
        let mut mouse = Mouse;
        let mut frame_memory = Bump::with_capacity(A::INITIAL_FRAME_MEMORY);

        event_loop.run(move |event, _, control_flow| {
            let _ = window;

            *control_flow = default_control_flow;

            match event {
                Event::NewEvents(cause) => {
                    // Update timers etc...
                },
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested if A::EXIT_ON_CLOSE => {
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::Resized(new_size) => renderer.resize(new_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(*new_inner_size)
                    },
                    _ => {},
                },
                Event::DeviceEvent { device_id, event } => {},
                Event::UserEvent(event) => {},
                Event::Suspended => {},
                Event::Resumed => {},
                Event::MainEventsCleared => {
                    game.update_and_render(&keyboard, &mouse, &mut frame_memory, &mut renderer);
                    frame_memory.reset();
                },
                Event::RedrawRequested(_) => {},
                Event::RedrawEventsCleared => {},
                Event::LoopDestroyed => game.clean_up(),
            }
        })
    }
}
