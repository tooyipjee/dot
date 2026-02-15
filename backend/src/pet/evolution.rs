use serde::{Deserialize, Serialize};
use super::state::PetState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStage {
    Egg,
    Baby,
    Child,
    Adult,
}

impl EvolutionStage {
    pub fn to_string(&self) -> String {
        match self {
            EvolutionStage::Egg => "Egg".to_string(),
            EvolutionStage::Baby => "Baby".to_string(),
            EvolutionStage::Child => "Child".to_string(),
            EvolutionStage::Adult => "Adult".to_string(),
        }
    }
}

// Evolution thresholds in seconds (longer phases)
const EGG_TO_BABY: u64 = 1800;      // 30 minutes
const BABY_TO_CHILD: u64 = 7200;    // 2 hours
const CHILD_TO_ADULT: u64 = 21600;  // 6 hours

pub fn check_evolution(pet: &mut PetState) {
    if !pet.is_alive {
        return;
    }

    let new_stage = match pet.stage {
        EvolutionStage::Egg => {
            if pet.age >= EGG_TO_BABY {
                Some(EvolutionStage::Baby)
            } else {
                None
            }
        }
        EvolutionStage::Baby => {
            if pet.age >= BABY_TO_CHILD {
                Some(EvolutionStage::Child)
            } else {
                None
            }
        }
        EvolutionStage::Child => {
            if pet.age >= CHILD_TO_ADULT {
                Some(EvolutionStage::Adult)
            } else {
                None
            }
        }
        EvolutionStage::Adult => None,
    };

    if let Some(stage) = new_stage {
        pet.stage = stage;
        println!("Pet evolved to {:?}!", stage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pet_at_age(age: u64) -> PetState {
        let mut pet = PetState::new();
        pet.age = age;
        pet
    }

    #[test]
    fn test_egg_stays_egg_before_threshold() {
        let mut pet = pet_at_age(EGG_TO_BABY - 1);
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Egg);
    }

    #[test]
    fn test_egg_evolves_to_baby_at_threshold() {
        let mut pet = pet_at_age(EGG_TO_BABY);
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Baby);
    }

    #[test]
    fn test_baby_stays_baby_before_threshold() {
        let mut pet = pet_at_age(BABY_TO_CHILD - 1);
        pet.stage = EvolutionStage::Baby;
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Baby);
    }

    #[test]
    fn test_baby_evolves_to_child() {
        let mut pet = pet_at_age(BABY_TO_CHILD);
        pet.stage = EvolutionStage::Baby;
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Child);
    }

    #[test]
    fn test_child_evolves_to_adult() {
        let mut pet = pet_at_age(CHILD_TO_ADULT);
        pet.stage = EvolutionStage::Child;
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Adult);
    }

    #[test]
    fn test_adult_stays_adult() {
        let mut pet = pet_at_age(999999);
        pet.stage = EvolutionStage::Adult;
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Adult);
    }

    #[test]
    fn test_no_evolution_when_dead() {
        let mut pet = pet_at_age(EGG_TO_BABY);
        pet.is_alive = false;
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Egg);
    }

    #[test]
    fn test_no_stage_regression() {
        // A baby should not regress to egg even if age is low
        let mut pet = pet_at_age(0);
        pet.stage = EvolutionStage::Baby;
        check_evolution(&mut pet);
        assert_eq!(pet.stage, EvolutionStage::Baby);
    }
}
