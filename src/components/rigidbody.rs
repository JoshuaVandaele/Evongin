use nalgebra::Vector2;

use crate::traits::{collider::Collider, physics::Physics};

pub struct Rigidbody {
    pub gravity: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub collider: Box<dyn Collider>,
}

impl Rigidbody {
    pub fn new(gravity: Vector2<f32>, velocity: Vector2<f32>, collider: Box<dyn Collider>) -> Self {
        Self {
            gravity,
            velocity,
            collider,
        }
    }
}

impl Physics for Rigidbody {
    fn update(&mut self, delta_time: f32, colliders: &Vec<Box<dyn Collider>>) {
        let is_colliding = colliders
            .iter()
            .any(|collider| self.collider.is_colliding(collider));

        if is_colliding {
            self.velocity = -self.velocity * 0.5;
        } else {
            self.velocity += self.gravity * delta_time;
        }

        self.collider.get_shape_mut().get_transform_mut().position += self.velocity * delta_time;
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        self.velocity += force;
    }
}
