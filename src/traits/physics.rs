use nalgebra::Vector2;

use super::collider::Collider;

pub trait Physics {
    fn update(&mut self, delta_time: f32, colliders: &Vec<Box<dyn Collider>>);
    fn apply_force(&mut self, force: Vector2<f32>);
    fn clone_box(&self) -> Box<dyn Physics>;
}
