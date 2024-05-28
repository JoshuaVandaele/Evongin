use std::any::Any;

use crate::{
    components::{colliders::polycollider::PolyCollider, shapes::circle::Circle},
    traits::collider::Collider,
};

pub struct CircleCollider {
    pub circle: Circle,
}

impl CircleCollider {
    pub fn new(circle: Box<Circle>) -> Self {
        Self { circle: *circle }
    }
}

impl Collider for CircleCollider {
    fn is_colliding_with_circle(&self, other: &CircleCollider) -> bool {
        let distance = (self.circle.transform.position.x - other.circle.transform.position.x)
            .powi(2)
            + (self.circle.transform.position.y - other.circle.transform.position.y).powi(2);
        let radius_sum = self.circle.radius + other.circle.radius;
        distance < radius_sum.powi(2)
    }

    fn is_colliding_with_poly(&self, other: &PolyCollider) -> bool {
        let poly_points = other.poly.get_points();
        let circle_radius = self.circle.radius;
        let circle_center = self.circle.transform.position;

        for i in 0..poly_points.len() {
            let p1 = poly_points[i];
            let p2 = poly_points[(i + 1) % poly_points.len()];

            let edge = p2 - p1;
            let t = ((circle_center - p1).dot(&edge)) / edge.norm_squared();
            let t = t.max(0.0).min(1.0);
            let closest_point = p1 + t * edge;

            if (closest_point - circle_center).norm() <= circle_radius {
                return true;
            }
        }

        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_shape(&self) -> &dyn crate::traits::shape::Shape {
        &self.circle
    }

    fn get_shape_mut(&mut self) -> &mut dyn crate::traits::shape::Shape {
        &mut self.circle
    }

    fn clone_box(&self) -> Box<dyn Collider> {
        Box::new(CircleCollider {
            circle: self.circle.clone(),
        })
    }
}
