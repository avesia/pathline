use crate::context::Renderer;

pub struct Canvas<T: Renderer> {
    renderer: T,
}

impl<T: Renderer> Canvas<T> {
    pub fn new(renderer: T) -> Self {
        Canvas {
            renderer,
        }
    }
}
