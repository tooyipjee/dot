pub mod commands;
pub mod pet;
pub mod storage;

use std::sync::Mutex;
use pet::PetState;
use pet::GameStats;

pub struct AppState {
    pub pet: Mutex<PetState>,
    pub stats: Mutex<GameStats>,
}

impl AppState {
    pub fn new() -> Self {
        let pet = match storage::load_state() {
            Ok(Some(saved_pet)) => saved_pet,
            Ok(None) => PetState::new(),
            Err(_) => PetState::new(),
        };

        let stats = match storage::load_stats() {
            Ok(Some(saved_stats)) => saved_stats,
            Ok(None) => GameStats::new(),
            Err(_) => GameStats::new(),
        };

        Self {
            pet: Mutex::new(pet),
            stats: Mutex::new(stats),
        }
    }
}
