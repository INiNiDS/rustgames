use crate::core::time::Time;
use crate::graphics::renderer::Renderer;

pub struct Context<'a> {
    pub time: &'a Time,
}

pub struct RenderContext<'a> {
    pub renderer: &'a mut Renderer,
    pub time: &'a Time,
}