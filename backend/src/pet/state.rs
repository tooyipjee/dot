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
