mod pipeline;
mod renderer;
mod vertex;

pub use crate::graphics::{pipeline::*, renderer::*, vertex::*};

use std::future::Future;

use winit::window::Window;

pub fn create_renderer(
    window: &Window,
) -> impl Future<Output = Result<Renderer, RendererError>> + '_ {
    Renderer::new(window)
}
