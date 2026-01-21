use super::state::PetState;

// Decay rates per minute
const HUNGER_DECAY_RATE: f64 = 0.5;
const HAPPINESS_DECAY_RATE: f64 = 0.3;
const ENERGY_DECAY_RATE: f64 = 0.2;

pub fn apply_decay(pet: &mut PetState, elapsed_seconds: u64) {
    if !pet.is_alive {
        return;
    }

    let minutes = elapsed_seconds as f64 / 60.0;

    // Apply decay
    let hunger_decay = (HUNGER_DECAY_RATE * minutes) as u8;
    let happiness_decay = (HAPPINESS_DECAY_RATE * minutes) as u8;
    let energy_decay = (ENERGY_DECAY_RATE * minutes) as u8;

    pet.hunger = pet.hunger.saturating_sub(hunger_decay);
    pet.happiness = pet.happiness.saturating_sub(happiness_decay);
    pet.energy = pet.energy.saturating_sub(energy_decay);

    // Health is affected by critical needs
    if pet.hunger < 20 || pet.happiness < 20 || pet.energy < 10 {
        let health_decay = (0.1 * minutes) as u8;
        pet.health = pet.health.saturating_sub(health_decay);
    } else if pet.hunger > 80 && pet.happiness > 80 && pet.energy > 50 {
        // Recover health when well cared for
        let health_recovery = (0.2 * minutes) as u8;
        pet.health = (pet.health + health_recovery).min(100);
    }
}
