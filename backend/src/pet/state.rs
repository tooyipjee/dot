use serde::{Deserialize, Serialize};
use chrono::Utc;
use super::evolution::EvolutionStage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetState {
    pub id: String,
    pub name: String,
    pub stage: EvolutionStage,
    pub age: u64,           // in seconds
    pub hunger: u8,         // 0-100
    pub happiness: u8,      // 0-100
    pub energy: u8,         // 0-100
    pub health: u8,         // 0-100
    pub is_alive: bool,
    pub birth_time: i64,
    pub last_update: i64,
}

impl PetState {
    pub fn new() -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "dot".to_string(),
            stage: EvolutionStage::Egg,
            age: 0,
            hunger: 100,
            happiness: 100,
            energy: 100,
            health: 100,
            is_alive: true,
            birth_time: now,
            last_update: now,
        }
    }

    pub fn update(&mut self) {
        if !self.is_alive {
            return;
        }

        let now = Utc::now().timestamp();
        let elapsed = (now - self.last_update) as u64;

        // Update age
        self.age += elapsed;

        // Update needs based on elapsed time
        super::needs::apply_decay(self, elapsed);

        // Check evolution
        super::evolution::check_evolution(self);

        // Check death conditions
        super::lifecycle::check_death(self);

        self.last_update = now;
    }

    pub fn feed(&mut self) {
        self.update();
        if !self.is_alive {
            return;
        }

        self.hunger = (self.hunger + 30).min(100);
        self.happiness = (self.happiness + 5).min(100);
    }

    pub fn play(&mut self) {
        self.update();
        if !self.is_alive {
            return;
        }

        self.happiness = (self.happiness + 20).min(100);
        self.energy = self.energy.saturating_sub(10);
        self.hunger = self.hunger.saturating_sub(5);
    }

    pub fn sleep(&mut self) {
        self.update();
        if !self.is_alive {
            return;
        }

        self.energy = (self.energy + 40).min(100);
        self.hunger = self.hunger.saturating_sub(10);
    }
}

impl Default for PetState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pet_defaults() {
        let pet = PetState::new();
        assert_eq!(pet.name, "dot");
        assert_eq!(pet.stage, EvolutionStage::Egg);
        assert_eq!(pet.age, 0);
        assert_eq!(pet.hunger, 100);
        assert_eq!(pet.happiness, 100);
        assert_eq!(pet.energy, 100);
        assert_eq!(pet.health, 100);
        assert!(pet.is_alive);
    }

    #[test]
    fn test_feed_increases_hunger() {
        let mut pet = PetState::new();
        pet.hunger = 50;
        pet.last_update = Utc::now().timestamp(); // prevent decay
        pet.feed();
        assert!(pet.hunger > 50);
    }

    #[test]
    fn test_feed_caps_at_100() {
        let mut pet = PetState::new();
        pet.hunger = 90;
        pet.last_update = Utc::now().timestamp();
        pet.feed();
        assert!(pet.hunger <= 100);
    }

    #[test]
    fn test_feed_increases_happiness() {
        let mut pet = PetState::new();
        pet.happiness = 50;
        pet.last_update = Utc::now().timestamp();
        pet.feed();
        assert!(pet.happiness > 50);
    }

    #[test]
    fn test_play_increases_happiness() {
        let mut pet = PetState::new();
        pet.happiness = 50;
        pet.last_update = Utc::now().timestamp();
        pet.play();
        assert!(pet.happiness > 50);
    }

    #[test]
    fn test_play_drains_energy() {
        let mut pet = PetState::new();
        pet.energy = 50;
        pet.last_update = Utc::now().timestamp();
        pet.play();
        assert!(pet.energy < 50);
    }

    #[test]
    fn test_play_drains_hunger() {
        let mut pet = PetState::new();
        pet.hunger = 50;
        pet.last_update = Utc::now().timestamp();
        pet.play();
        assert!(pet.hunger < 50);
    }

    #[test]
    fn test_sleep_increases_energy() {
        let mut pet = PetState::new();
        pet.energy = 50;
        pet.last_update = Utc::now().timestamp();
        pet.sleep();
        assert!(pet.energy > 50);
    }

    #[test]
    fn test_sleep_caps_energy_at_100() {
        let mut pet = PetState::new();
        pet.energy = 90;
        pet.last_update = Utc::now().timestamp();
        pet.sleep();
        assert!(pet.energy <= 100);
    }

    #[test]
    fn test_no_action_when_dead() {
        let mut pet = PetState::new();
        pet.is_alive = false;
        pet.hunger = 10;
        pet.happiness = 10;
        pet.energy = 10;

        pet.feed();
        assert_eq!(pet.hunger, 10);

        pet.play();
        assert_eq!(pet.happiness, 10);

        pet.sleep();
        assert_eq!(pet.energy, 10);
    }

    #[test]
    fn test_energy_no_underflow() {
        let mut pet = PetState::new();
        pet.energy = 3;
        pet.last_update = Utc::now().timestamp();
        pet.play(); // -10 energy
        assert_eq!(pet.energy, 0); // saturating_sub prevents underflow
    }

    #[test]
    fn test_hunger_no_underflow_on_play() {
        let mut pet = PetState::new();
        pet.hunger = 2;
        pet.last_update = Utc::now().timestamp();
        pet.play(); // -5 hunger
        assert_eq!(pet.hunger, 0);
    }
}
