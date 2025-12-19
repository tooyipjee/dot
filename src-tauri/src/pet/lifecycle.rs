use super::state::PetState;

pub fn check_death(pet: &mut PetState) {
    if !pet.is_alive {
        return;
    }

    // Death conditions
    if pet.health == 0 {
        pet.is_alive = false;
        println!("Pet died from poor health!");
    }

    // Can also die from extreme neglect
    if pet.hunger == 0 && pet.happiness < 10 {
        pet.is_alive = false;
        println!("Pet died from neglect!");
    }
}

pub fn revive(pet: &mut PetState) {
    // Reset to new egg
    let now = chrono::Utc::now().timestamp();
    pet.stage = super::evolution::EvolutionStage::Egg;
    pet.age = 0;
    pet.hunger = 100;
    pet.happiness = 100;
    pet.energy = 100;
    pet.health = 100;
    pet.is_alive = true;
    pet.birth_time = now;
    pet.last_update = now;
    println!("Pet revived as a new egg!");
}
