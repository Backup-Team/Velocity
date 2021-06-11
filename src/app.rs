use bumpalo::Bump;

use crate::{graphics::Renderer, keyboard::Keyboard, mouse::Mouse};

pub enum LoopState {
    Continue,
    Exit,
}

#[allow(unused_variables)]
pub trait App<UserEvent = ()> {
    const EXIT_ON_CLOSE: bool;
    const INITIAL_FRAME_MEMORY: usize = 1024;

    fn update_and_render(
        &mut self,
        keyboard: &Keyboard,
        mouse: &Mouse,
        frame_memory: &mut Bump,
        renderer: &mut Renderer,
    ) -> LoopState;

    fn clean_up(&mut self) {}
}
