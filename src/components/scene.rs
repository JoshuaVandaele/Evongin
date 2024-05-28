use sdl2::render::Canvas;

use crate::{
    traits::{collider::Collider, object::Object, physics::Physics},
    utils::render_targets::RenderTargets,
};

use super::camera::Camera;

pub struct Scene {
    pub camera: Camera,
    objects: Vec<Box<dyn Object>>,
    colliders: Vec<Box<dyn Collider>>,
    physics_objects: Vec<Box<dyn Physics>>,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(),
            colliders: Vec::new(),
            physics_objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, mut object: Box<dyn Object>) {
        if let Some(collider) = object.as_any_mut().downcast_mut::<Box<dyn Collider>>() {
            self.colliders.push(collider.clone_box());
        }

        if let Some(physics_object) = object.as_any_mut().downcast_mut::<Box<dyn Physics>>() {
            self.physics_objects.push(physics_object.clone_box());
        }

        self.objects.push(object);
    }

    pub fn add_collider(&mut self, collider: Box<dyn Collider>) {
        self.colliders.push(collider);
    }

    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }

        for physics_object in &mut self.physics_objects {
            physics_object.update(delta_time, &self.colliders);
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        for object in &self.objects {
            object.draw(&mut self.camera.canvas);
        }

        let mut render_target = RenderTargets::Window(canvas);

        self.camera.draw(&mut render_target);
    }

    pub fn run(&mut self, delta_time: f32, canvas: &mut Canvas<sdl2::video::Window>) {
        self.update(delta_time);
        self.draw(canvas);
    }
}
