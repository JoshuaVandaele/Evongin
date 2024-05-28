use std::time::Instant;

use sdl2::render::Canvas;

use super::scene::Scene;

pub struct Simulation {
    canvas: Canvas<sdl2::video::Window>,
    scenes: Vec<Scene>,
    current_scene: usize,
}

impl Simulation {
    pub fn new(window: sdl2::video::Window, scenes: Vec<Scene>, current_scene: usize) -> Self {
        let canvas = window.into_canvas().build().unwrap();
        Self {
            canvas,
            scenes,
            current_scene: current_scene,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = sdl2::init().unwrap().event_pump().unwrap();
        let mut last_instant = Instant::now();
        let mut delta_time: f32;

        'running: loop {
            delta_time = last_instant.elapsed().as_secs_f32();
            last_instant = Instant::now();

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            let current_scene = &mut self.scenes[self.current_scene];
            self.canvas.clear();
            current_scene.run(delta_time, &mut self.canvas);
            self.canvas.present();
        }
    }
}
