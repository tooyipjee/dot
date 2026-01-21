use serde::{Deserialize, Serialize};
use super::state::PetState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStage {
    Egg,
    Baby,
    Teen,
    Adult,
}

impl EvolutionStage {
    pub fn to_string(&self) -> String {
        match self {
            EvolutionStage::Egg => "Egg".to_string(),
            EvolutionStage::Baby => "Baby".to_string(),
            EvolutionStage::Teen => "Teen".to_string(),
            EvolutionStage::Adult => "Adult".to_string(),
        }
    }
}

// Evolution thresholds in seconds
const EGG_TO_BABY: u64 = 300;      // 5 minutes for testing (normally would be hours)
const BABY_TO_TEEN: u64 = 900;     // 15 minutes
const TEEN_TO_ADULT: u64 = 1800;   // 30 minutes

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
            if pet.age >= BABY_TO_TEEN {
                Some(EvolutionStage::Teen)
            } else {
                None
            }
        }
        EvolutionStage::Teen => {
            if pet.age >= TEEN_TO_ADULT {
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
