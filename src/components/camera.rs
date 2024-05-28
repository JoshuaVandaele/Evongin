use super::transform::Transform;
use crate::traits::object::Object;
use crate::utils::render_targets::RenderTargets;
use sdl2::surface::Surface;

pub struct Camera {
    pub canvas: RenderTargets<'static>,
    pub transform: Transform,
}

impl Camera {
    pub fn new(surface: Surface<'static>, transform: Transform) -> Self {
        let canvas = surface.into_canvas().unwrap();
        let canvas = RenderTargets::Surface(canvas);
        Self { canvas, transform }
    }
}

impl Object for Camera {
    /// Draw the camera to the canvas
    fn draw(&self, canvas: &mut RenderTargets) {
        let self_canvas = match canvas {
            RenderTargets::Window(_) => unimplemented!(),
            RenderTargets::Surface(surface) => surface,
        };
        let texture_creator = self_canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&self_canvas.surface())
            .unwrap();

        // Create a destination rectangle with the position of the transform
        let dst = Some(sdl2::rect::Rect::new(
            self.transform.position.x as i32,
            self.transform.position.y as i32,
            self_canvas.surface().width(),
            self_canvas.surface().height(),
        ));

        match canvas {
            RenderTargets::Window(window_canvas) => {
                let _ = window_canvas.copy_ex(
                    &texture,
                    None,
                    dst,
                    self.transform.rotation.angle().into(),
                    None,
                    false,
                    false,
                );
            }
            RenderTargets::Surface(surface) => {
                let _ = surface.copy_ex(
                    &texture,
                    None,
                    dst,
                    self.transform.rotation.angle().into(),
                    None,
                    false,
                    false,
                );
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
