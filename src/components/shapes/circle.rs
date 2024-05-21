use crate::components::transform::Transform;
use crate::traits::render::Render;
use crate::traits::shape::Shape;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Circle {
    pub transform: Transform,
    pub radius: f32,
    pub color: Color,
}

impl Circle {
    pub fn new(transform: Transform, radius: f32, color: Color) -> Self {
        Self {
            transform,
            radius,
            color,
        }
    }
}

impl Render for Circle {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let x0 = (self.transform.position.x * self.transform.scale.x) as i32;
        let y0 = (self.transform.position.y * self.transform.scale.y) as i32;

        let radius = (self.radius * self.transform.scale.x) as i32;

        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        canvas.set_draw_color(self.color);
        while x >= y {
            canvas.draw_point(Point::new(x0 + x, y0 + y)).unwrap();
            canvas.draw_point(Point::new(x0 + y, y0 + x)).unwrap();
            canvas.draw_point(Point::new(x0 - y, y0 + x)).unwrap();
            canvas.draw_point(Point::new(x0 - x, y0 + y)).unwrap();
            canvas.draw_point(Point::new(x0 - x, y0 - y)).unwrap();
            canvas.draw_point(Point::new(x0 - y, y0 - x)).unwrap();
            canvas.draw_point(Point::new(x0 + y, y0 - x)).unwrap();
            canvas.draw_point(Point::new(x0 + x, y0 - y)).unwrap();

            y += 1;
            err += 1 + (y << 1);
            if ((err - x) << 1) + 1 > 0 {
                x -= 1;
                err += 1 - (x << 1);
            }
        }
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius) as f64
    }

    fn get_center(&self) -> nalgebra::Vector2<f32> {
        self.transform.position
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius as f64
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
}
