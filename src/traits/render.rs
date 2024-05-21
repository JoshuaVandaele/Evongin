pub trait Render {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
}
