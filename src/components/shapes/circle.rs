use crate::components::transform::Transform;
use crate::traits::object::Object;
use crate::traits::shape::Shape;
use crate::utils::render_targets::RenderTargets;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::RenderTarget;

#[derive(Clone)]
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

    fn draw_circle<T: RenderTarget>(
        &self,
        x0: i32,
        y0: i32,
        radius: i32,
        canvas: &mut sdl2::render::Canvas<T>,
    ) {
        let mut err = 0;
        let mut x = radius;
        let mut y = 0;

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

impl Object for Circle {
    fn draw(&self, canvas: &mut RenderTargets) {
        let x0 = (self.transform.position.x * self.transform.scale.x) as i32;
        let y0 = (self.transform.position.y * self.transform.scale.y) as i32;

        let radius = (self.radius * self.transform.scale.x) as i32;

        let x = radius;

        match canvas {
            RenderTargets::Window(canvas) => {
                canvas.set_draw_color(self.color);
                self.draw_circle(x0, y0, x, canvas);
            }
            RenderTargets::Surface(canvas) => {
                canvas.set_draw_color(self.color);
                self.draw_circle(x0, y0, x, canvas);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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
