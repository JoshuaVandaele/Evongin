use nalgebra::Vector2;

use super::shape::Shape;

pub trait Poly: Shape {
    fn get_points(&self) -> Vec<Vector2<f32>>;
    fn get_shape(&self) -> &dyn Shape;
    fn get_shape_mut(&mut self) -> &mut dyn Shape;
    fn clone_box(&self) -> Box<dyn Poly>;
}
