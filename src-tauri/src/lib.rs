pub mod commands;
pub mod pet;
pub mod storage;

use std::sync::Mutex;
use pet::PetState;

pub struct AppState {
    pub pet: Mutex<PetState>,
}

impl AppState {
    pub fn new() -> Self {
        // Try to load saved state, otherwise create new pet
        let pet = match storage::load_state() {
            Ok(Some(saved_pet)) => saved_pet,
            Ok(None) => PetState::new(),
            Err(_) => PetState::new(),
        };

        Self {
            pet: Mutex::new(pet),
        }
    }
}
