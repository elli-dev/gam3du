use crate::{game_state::GameState, plugin};
use gam3du_framework::event::{ApplicationEvent, EngineEvent};
use log::debug;
use std::{
    sync::{
        mpsc::{Receiver, TryRecvError},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

/// Number of game loop iterations per second.
/// This is a multiple of common frame rates.
const TICKS_PER_SECOND: u32 = 240;

/// Duration of each game tick. Same as
/// `Duration::from_secs_f64(f64::from(TICKS_PER_SECOND).recip())`
/// but with const support
const TICK_DURATION: Duration = Duration::from_nanos(
    (1_000_000_000_u64 + TICKS_PER_SECOND as u64 / 2) / TICKS_PER_SECOND as u64,
);

/// The root object of a running engine
pub struct GameLoop<Plugin: plugin::Plugin> {
    /// Contains the current state which will be updated by the game loop.
    /// This might be shared with renderers.
    /// In order to allow multiple renderers, this is a `RwLock` rather than a `Mutex`.
    game_state: Arc<RwLock<Box<GameState>>>,
    plugin: Option<Plugin>,
}

impl<Plugin: plugin::Plugin> Default for GameLoop<Plugin> {
    fn default() -> Self {
        Self {
            game_state: Arc::new(RwLock::new(Box::new(GameState::default()))),
            plugin: None,
        }
    }
}

impl<Plugin: plugin::Plugin> GameLoop<Plugin> {
    pub fn run(mut self, event_source: &Receiver<EngineEvent>) {
        let mut time = Instant::now();

        if let Some(plugin) = &mut self.plugin {
            let mut game_state = self.game_state.write().unwrap();

            plugin.init(&mut game_state);
        }

        'game_loop: loop {
            {
                let mut game_state = self.game_state.write().unwrap();

                'next_event: loop {
                    match event_source.try_recv() {
                        Ok(engine_event) => match engine_event {
                            EngineEvent::Window { event } => {
                                debug!("{event:?}");
                            }
                            EngineEvent::Device { event } => {
                                debug!("{event:?}");
                            }
                            EngineEvent::Application { event } => match event {
                                ApplicationEvent::Exit => {
                                    debug!("Received Exit-event. Exiting game loop");
                                    break 'game_loop;
                                }
                            },
                        },
                        Err(TryRecvError::Disconnected) => {
                            debug!("Event source disconnected. Exiting game loop");
                            break 'game_loop;
                        }
                        Err(TryRecvError::Empty) => break 'next_event,
                    }
                }

                // run scripting runtimes here
                if let Some(plugin) = &mut self.plugin {
                    plugin.update(&mut game_state);
                }

                game_state.update();
            }

            // compute the timestamp of the next game loop iteration
            time += TICK_DURATION;
            if let Some(delay) = time.checked_duration_since(Instant::now()) {
                thread::sleep(delay);
            } else {
                // game loop is running too slow
            }
        }
    }

    #[must_use]
    pub fn clone_state(&self) -> Arc<RwLock<Box<GameState>>> {
        Arc::clone(&self.game_state)
    }

    pub fn add_plugin(&mut self, plugin: Plugin) {
        assert!(
            self.plugin.replace(plugin).is_none(),
            "only one plugin can be set for now"
        );
    }
}
