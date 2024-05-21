use std::any::Any;

use crate::components::colliders::{circlecollider::CircleCollider, polycollider::PolyCollider};

use super::shape::Shape;
pub trait Collider {
    fn is_colliding_with_circle(&self, _circle: &CircleCollider) -> bool {
        false
    }

    fn is_colliding_with_poly(&self, _rect: &PolyCollider) -> bool {
        false
    }

    fn is_colliding(&self, other: &Box<dyn Collider>) -> bool {
        if let Some(circle) = other.as_any().downcast_ref::<CircleCollider>() {
            self.is_colliding_with_circle(circle)
        } else if let Some(rect) = other.as_any().downcast_ref::<PolyCollider>() {
            self.is_colliding_with_poly(rect)
        } else {
            unimplemented!("Unknown collider type: {:?}", other.as_any())
        }
    }

    fn as_any(&self) -> &dyn Any;

    fn get_shape(&self) -> &dyn Shape;
    fn get_shape_mut(&mut self) -> &mut dyn Shape;
}
