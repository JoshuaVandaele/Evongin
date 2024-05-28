use std::any::Any;

use crate::utils::render_targets::RenderTargets;

pub trait Object: Any {
    fn draw(&self, canvas: &mut RenderTargets);
    fn update(&mut self, _delta_time: f32) {}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
