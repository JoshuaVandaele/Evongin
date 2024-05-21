use crate::components::transform::Transform;
use crate::traits::poly::Poly;
use crate::traits::render::Render;
use crate::traits::shape::Shape;
use nalgebra::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Rect {
    pub transform: Transform,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rect {
    pub fn new(transform: Transform, width: f32, height: f32, color: Color) -> Self {
        Self {
            transform,
            width,
            height,
            color,
        }
    }
}

impl Render for Rect {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let vectors = self.get_points();

        let points: Vec<Point> = vectors
            .iter()
            .map(|vector| Point::new(vector.x as i32, vector.y as i32))
            .collect();

        canvas.set_draw_color(self.color);
        canvas.draw_lines(&points[..]).unwrap();
    }
}

impl Poly for Rect {
    fn get_points(&self) -> Vec<Vector2<f32>> {
        let x = self.transform.position.x;
        let y = self.transform.position.y;

        let points = [
            (x, y),
            (x + self.width, y),
            (x + self.width, y + self.height),
            (x, y + self.height),
            (x, y),
        ];

        let rotation_center = (
            (self.transform.position.x + self.width / 2.0).round(),
            (self.transform.position.y + self.height / 2.0).round(),
        );

        let rotation_matrix = self.transform.rotation;

        points
            .iter()
            .map(|point| {
                let translated_x = point.0 - rotation_center.0;
                let translated_y = point.1 - rotation_center.1;

                let rotated_x =
                    rotation_matrix[(0, 0)] * translated_x + rotation_matrix[(0, 1)] * translated_y;
                let rotated_y =
                    rotation_matrix[(1, 0)] * translated_x + rotation_matrix[(1, 1)] * translated_y;

                let final_x = rotated_x + rotation_center.0;
                let final_y = rotated_y + rotation_center.1;

                Vector2::new(final_x, final_y)
            })
            .collect()
    }

    fn get_shape(&self) -> &dyn Shape {
        self
    }

    fn get_shape_mut(&mut self) -> &mut dyn Shape {
        self
    }

    fn clone_box(&self) -> Box<dyn Poly> {
        Box::new(Rect {
            transform: self.transform.clone(),
            width: self.width,
            height: self.height,
            color: self.color,
        })
    }
}

impl Shape for Rect {
    fn get_center(&self) -> Vector2<f32> {
        Vector2::new(
            self.transform.position.x + self.width / 2.0,
            self.transform.position.y + self.height / 2.0,
        )
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * (self.width as f64 + self.height as f64)
    }
}
