use tauri::{AppHandle, State};
use serde::Serialize;
use crate::{AppState, pet::{PetState, GameStats, Achievement, achievements}, storage};

fn save_pet(pet: &PetState) {
    if let Err(e) = storage::save_state(pet) {
        eprintln!("Failed to save pet state: {}", e);
    }
}

fn save_stats(stats: &GameStats) {
    if let Err(e) = storage::save_stats(stats) {
        eprintln!("Failed to save game stats: {}", e);
    }
}

#[derive(Serialize, Clone)]
pub struct ActionResult {
    pub pet: PetState,
    pub new_achievements: Vec<Achievement>,
}

#[tauri::command]
pub fn get_pet_state(state: State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.update();
    save_pet(&pet);

    // Check achievements on poll too (for evolution-based ones)
    let mut stats = state.stats.lock().map_err(|e| e.to_string())?;
    let _ = achievements::check_achievements(&mut stats, &pet);
    save_stats(&stats);

    Ok(pet.clone())
}

#[tauri::command]
pub fn feed_pet(state: State<AppState>) -> Result<ActionResult, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.feed();
    save_pet(&pet);

    let mut stats = state.stats.lock().map_err(|e| e.to_string())?;
    stats.record_feed();
    let new_achievements = achievements::check_achievements(&mut stats, &pet);
    save_stats(&stats);

    Ok(ActionResult {
        pet: pet.clone(),
        new_achievements,
    })
}

#[tauri::command]
pub fn play_with_pet(state: State<AppState>) -> Result<ActionResult, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.play();
    save_pet(&pet);

    let mut stats = state.stats.lock().map_err(|e| e.to_string())?;
    stats.record_play();
    let new_achievements = achievements::check_achievements(&mut stats, &pet);
    save_stats(&stats);

    Ok(ActionResult {
        pet: pet.clone(),
        new_achievements,
    })
}

#[tauri::command]
pub fn put_to_sleep(state: State<AppState>) -> Result<ActionResult, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.sleep();
    save_pet(&pet);

    let mut stats = state.stats.lock().map_err(|e| e.to_string())?;
    stats.record_sleep();
    let new_achievements = achievements::check_achievements(&mut stats, &pet);
    save_stats(&stats);

    Ok(ActionResult {
        pet: pet.clone(),
        new_achievements,
    })
}

#[tauri::command]
pub fn revive_pet(state: State<AppState>) -> Result<ActionResult, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    crate::pet::lifecycle::revive(&mut pet);
    save_pet(&pet);

    let mut stats = state.stats.lock().map_err(|e| e.to_string())?;
    stats.record_revive();
    let new_achievements = achievements::check_achievements(&mut stats, &pet);
    save_stats(&stats);

    Ok(ActionResult {
        pet: pet.clone(),
        new_achievements,
    })
}

#[tauri::command]
pub fn get_game_stats(state: State<AppState>) -> Result<GameStats, String> {
    let stats = state.stats.lock().map_err(|e| e.to_string())?;
    Ok(stats.clone())
}

#[tauri::command]
pub fn get_achievements(state: State<AppState>) -> Result<Vec<Achievement>, String> {
    let stats = state.stats.lock().map_err(|e| e.to_string())?;
    Ok(achievements::get_all_achievements(&stats))
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}
