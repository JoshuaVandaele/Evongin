pub enum RenderTargets<'a> {
    Window(&'a mut sdl2::render::Canvas<sdl2::video::Window>),
    Surface(sdl2::render::Canvas<sdl2::surface::Surface<'a>>),
}
