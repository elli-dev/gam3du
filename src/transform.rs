use glam::Vec3;

use crate::application::{self, component};

pub struct TransformComponent {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl application::component::Component for TransformComponent {
    fn start(&mut self, state: &mut application::state::State) {
        //println!("{}", self.position);
    }

    fn update(&mut self, state: &mut application::state::State) {
        println!(
            "TestComponent::update | FPS: {} | TPS: {}",
            state.measured_frames_per_second, state.measured_ticks_per_second
        );
    }

    fn render(&mut self, state: &mut application::state::State) {
        println!("TestComponent::render");
    }

    fn stop(&mut self, state: &mut application::state::State) {
        println!("TestComponent::stop");
    }
}
