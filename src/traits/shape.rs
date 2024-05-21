use nalgebra::Vector2;

use crate::components::transform::Transform;

use super::render::Render;

pub trait Shape: Render {
    fn set_transform(&mut self, transform: Transform);

    fn get_transform(&self) -> Transform;
    fn get_transform_mut(&mut self) -> &mut Transform;
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
    fn get_center(&self) -> Vector2<f32>;
}
