use std::any::Any;

use nalgebra::Vector2;

use crate::{
    components::colliders::circlecollider::CircleCollider,
    traits::{collider::Collider, poly::Poly},
};

pub struct PolyCollider {
    pub poly: Box<dyn Poly>,
}

impl PolyCollider {
    pub fn new(poly: Box<dyn Poly>) -> Self {
        Self { poly }
    }

    pub fn clone(&self) -> Self {
        Self {
            poly: self.poly.clone_box(),
        }
    }
}

impl Collider for PolyCollider {
    fn is_colliding_with_poly(&self, other: &PolyCollider) -> bool {
        let poly1_points = self.poly.get_points();
        let poly2_points = other.poly.get_points();

        for poly_points in &[&poly1_points, &poly2_points] {
            for i in 0..poly_points.len() {
                let p1 = poly_points[i];
                let p2 = poly_points[(i + 1) % poly_points.len()];

                let edge = p2 - p1;

                let normal = Vector2::new(-edge.y, edge.x);

                let mut min1 = f32::MAX;
                let mut max1 = f32::MIN;
                for p in &poly1_points {
                    let dot = normal.dot(&p);
                    min1 = min1.min(dot);
                    max1 = max1.max(dot);
                }

                let mut min2 = f32::MAX;
                let mut max2 = f32::MIN;
                for p in &poly2_points {
                    let dot = normal.dot(&p);
                    min2 = min2.min(dot);
                    max2 = max2.max(dot);
                }

                // If the projections do not overlap, the polygons do not collide
                if max1 < min2 || max2 < min1 {
                    return false;
                }
            }
        }

        // No separating axis found, the polygons collide
        true
    }

    fn is_colliding_with_circle(&self, other: &CircleCollider) -> bool {
        let poly_points = self.poly.get_points();
        let circle_radius = other.circle.radius;
        let circle_center = other.circle.transform.position;

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
        self.poly.get_shape()
    }

    fn get_shape_mut(&mut self) -> &mut dyn crate::traits::shape::Shape {
        self.poly.get_shape_mut()
    }

    fn clone_box(&self) -> Box<dyn Collider> {
        Box::new(self.clone())
    }
}
