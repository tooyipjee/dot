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
