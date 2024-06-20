pub mod component;
pub mod entity;
pub mod event_subscriber;
mod runtime;
pub mod state;

use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct Application {
    pub state: Arc<RwLock<state::State>>,

    application_runtime: runtime::ApplicationRuntime,
}

impl Application {
    pub fn start(&mut self) {
        let runtime = &mut self.application_runtime;

        runtime.start(Arc::clone(&self.state));
    }

    #[must_use]
    pub fn get_state_arc(&self) -> Arc<RwLock<state::State>> {
        Arc::clone(&self.state)
    }
}
