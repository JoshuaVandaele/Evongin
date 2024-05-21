use nalgebra::{Rotation2, Vector2};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// The position of the object in the world.
    pub position: Vector2<f32>,
    /// The scale of the object in the world.
    pub scale: Vector2<f32>,
    /// The rotation of the object in radians.
    pub rotation: Rotation2<f32>,
}

impl Transform {
    pub fn new(position: Vector2<f32>, scale: Vector2<f32>, rotation: Rotation2<f32>) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector2::default(),
            scale: Vector2::new(1.0, 1.0),
            rotation: Rotation2::identity(),
        }
    }
}
