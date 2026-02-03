
use crate::core::Engine;

pub trait Game {
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
}