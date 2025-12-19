use tauri::State;
use crate::{AppState, pet::PetState, storage};

fn save_pet(pet: &PetState) {
    if let Err(e) = storage::save_state(pet) {
        eprintln!("Failed to save pet state: {}", e);
    }
}

#[tauri::command]
pub fn get_pet_state(state: State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.update();
    save_pet(&pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn feed_pet(state: State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.feed();
    save_pet(&pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn play_with_pet(state: State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.play();
    save_pet(&pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn put_to_sleep(state: State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.sleep();
    save_pet(&pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn revive_pet(state: State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    crate::pet::lifecycle::revive(&mut pet);
    save_pet(&pet);
    Ok(pet.clone())
}
