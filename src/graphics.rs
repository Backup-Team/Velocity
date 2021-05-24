mod pipeline;
mod renderer;
mod vertex;

pub use crate::graphics::{pipeline::*, renderer::*, vertex::*};

use winit::window::Window;

async fn create_renderer(window: &Window) -> Result<Renderer, RendererError> {
    Renderer::new(window)
}
