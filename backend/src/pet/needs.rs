use super::state::PetState;

// Decay rates per minute (slower = easier game)
const HUNGER_DECAY_RATE: f64 = 0.2;    // was 0.5
const HAPPINESS_DECAY_RATE: f64 = 0.15; // was 0.3
const ENERGY_DECAY_RATE: f64 = 0.1;     // was 0.2

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

    // Health is affected by critical needs (more forgiving thresholds)
    if pet.hunger < 10 || pet.happiness < 10 || pet.energy < 5 {
        let health_decay = (0.05 * minutes) as u8; // slower health decay
        pet.health = pet.health.saturating_sub(health_decay);
    } else if pet.hunger > 50 && pet.happiness > 50 && pet.energy > 30 {
        // Recover health more easily
        let health_recovery = (0.3 * minutes) as u8; // faster recovery
        pet.health = (pet.health + health_recovery).min(100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_pet() -> PetState {
        PetState::new()
    }

    #[test]
    fn test_no_decay_at_zero_elapsed() {
        let mut pet = new_pet();
        apply_decay(&mut pet, 0);
        assert_eq!(pet.hunger, 100);
        assert_eq!(pet.happiness, 100);
        assert_eq!(pet.energy, 100);
    }

    #[test]
    fn test_decay_after_one_minute() {
        let mut pet = new_pet();
        apply_decay(&mut pet, 60); // 1 minute
        // 0.2 * 1 = 0.2 -> truncated to 0 for u8
        // Decay is very small at 1 minute, so stats stay at 100
        assert!(pet.hunger >= 99);
        assert!(pet.happiness >= 99);
        assert!(pet.energy >= 99);
    }

    #[test]
    fn test_decay_after_ten_minutes() {
        let mut pet = new_pet();
        apply_decay(&mut pet, 600); // 10 minutes
        // hunger: 0.2 * 10 = 2
        // happiness: 0.15 * 10 = 1
        // energy: 0.1 * 10 = 1
        assert_eq!(pet.hunger, 98);
        assert_eq!(pet.happiness, 99);
        assert_eq!(pet.energy, 99);
    }

    #[test]
    fn test_decay_after_one_hour() {
        let mut pet = new_pet();
        apply_decay(&mut pet, 3600); // 60 minutes
        // hunger: 0.2 * 60 = 12
        // happiness: 0.15 * 60 = 9
        // energy: 0.1 * 60 = 6
        assert_eq!(pet.hunger, 88);
        assert_eq!(pet.happiness, 91);
        assert_eq!(pet.energy, 94);
    }

    #[test]
    fn test_stats_dont_go_below_zero() {
        let mut pet = new_pet();
        pet.hunger = 5;
        pet.happiness = 5;
        pet.energy = 5;
        apply_decay(&mut pet, 36000); // 600 minutes - massive decay
        assert_eq!(pet.hunger, 0);
        assert_eq!(pet.happiness, 0);
        assert_eq!(pet.energy, 0);
    }

    #[test]
    fn test_health_decays_when_hunger_critical() {
        let mut pet = new_pet();
        pet.hunger = 5; // critical
        pet.happiness = 50;
        pet.energy = 50;
        pet.health = 100;
        apply_decay(&mut pet, 6000); // 100 minutes
        // health_decay = 0.05 * 100 = 5
        assert!(pet.health < 100);
    }

    #[test]
    fn test_health_recovers_when_stats_good() {
        let mut pet = new_pet();
        pet.hunger = 80;
        pet.happiness = 80;
        pet.energy = 80;
        pet.health = 50;
        apply_decay(&mut pet, 600); // 10 minutes
        // recovery = 0.3 * 10 = 3
        assert!(pet.health > 50);
    }

    #[test]
    fn test_health_capped_at_100() {
        let mut pet = new_pet();
        pet.hunger = 80;
        pet.happiness = 80;
        pet.energy = 80;
        pet.health = 99;
        apply_decay(&mut pet, 600);
        assert!(pet.health <= 100);
    }

    #[test]
    fn test_no_decay_when_dead() {
        let mut pet = new_pet();
        pet.is_alive = false;
        pet.hunger = 50;
        apply_decay(&mut pet, 3600);
        assert_eq!(pet.hunger, 50); // unchanged
    }
}
