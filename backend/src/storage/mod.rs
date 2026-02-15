use std::path::PathBuf;
use std::fs;
use crate::pet::PetState;
use crate::pet::GameStats;

fn get_config_dir() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| "Could not find config directory".to_string())?;
    path.push("dot");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn get_storage_path() -> Result<PathBuf, String> {
    let mut path = get_config_dir()?;
    path.push("pet-state.json");
    Ok(path)
}

fn get_stats_path() -> Result<PathBuf, String> {
    let mut path = get_config_dir()?;
    path.push("game-stats.json");
    Ok(path)
}

pub fn save_state(pet: &PetState) -> Result<(), String> {
    let path = get_storage_path()?;
    let json = serde_json::to_string_pretty(pet).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_state() -> Result<Option<PetState>, String> {
    let path = get_storage_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let pet: PetState = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(Some(pet))
}

pub fn save_stats(stats: &GameStats) -> Result<(), String> {
    let path = get_stats_path()?;
    let json = serde_json::to_string_pretty(stats).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_stats() -> Result<Option<GameStats>, String> {
    let path = get_stats_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let stats: GameStats = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(Some(stats))
}
