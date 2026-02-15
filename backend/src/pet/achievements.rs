use serde::{Deserialize, Serialize};
use super::evolution::EvolutionStage;
use super::stats::GameStats;
use super::state::PetState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
}

struct AchievementDef {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    check: fn(&GameStats, &PetState) -> bool,
}

const ACHIEVEMENTS: &[AchievementDef] = &[
    AchievementDef {
        id: "first_hatch",
        name: "First Hatch",
        description: "Evolve from Egg to Baby",
        check: |_stats, pet| !matches!(pet.stage, EvolutionStage::Egg),
    },
    AchievementDef {
        id: "growing_up",
        name: "Growing Up",
        description: "Reach Child stage",
        check: |_stats, pet| {
            matches!(pet.stage, EvolutionStage::Child | EvolutionStage::Adult)
        },
    },
    AchievementDef {
        id: "all_grown_up",
        name: "All Grown Up",
        description: "Reach Adult stage",
        check: |_stats, pet| matches!(pet.stage, EvolutionStage::Adult),
    },
    AchievementDef {
        id: "streak_3",
        name: "Caretaker",
        description: "3-day care streak",
        check: |stats, _pet| stats.longest_streak >= 3,
    },
    AchievementDef {
        id: "streak_7",
        name: "Devoted",
        description: "7-day care streak",
        check: |stats, _pet| stats.longest_streak >= 7,
    },
    AchievementDef {
        id: "streak_30",
        name: "Best Friend",
        description: "30-day care streak",
        check: |stats, _pet| stats.longest_streak >= 30,
    },
    AchievementDef {
        id: "feed_100",
        name: "Gourmet",
        description: "Feed 100 times",
        check: |stats, _pet| stats.total_feeds >= 100,
    },
    AchievementDef {
        id: "play_50",
        name: "Playmate",
        description: "Play 50 times",
        check: |stats, _pet| stats.total_plays >= 50,
    },
    AchievementDef {
        id: "sleep_50",
        name: "Lullaby",
        description: "Put to sleep 50 times",
        check: |stats, _pet| stats.total_sleeps >= 50,
    },
    AchievementDef {
        id: "comeback",
        name: "Second Chance",
        description: "Revive a pet",
        check: |stats, _pet| stats.total_pets_raised > 1,
    },
];

/// Check for newly unlocked achievements. Returns list of newly unlocked ones.
pub fn check_achievements(stats: &mut GameStats, pet: &PetState) -> Vec<Achievement> {
    let mut newly_unlocked = Vec::new();

    for def in ACHIEVEMENTS {
        if stats.unlocked.contains(&def.id.to_string()) {
            continue;
        }
        if (def.check)(stats, pet) {
            stats.unlock_achievement(def.id);
            newly_unlocked.push(Achievement {
                id: def.id.to_string(),
                name: def.name.to_string(),
                description: def.description.to_string(),
                unlocked: true,
            });
        }
    }

    newly_unlocked
}

/// Get all achievements with their unlock status.
pub fn get_all_achievements(stats: &GameStats) -> Vec<Achievement> {
    ACHIEVEMENTS
        .iter()
        .map(|def| Achievement {
            id: def.id.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            unlocked: stats.unlocked.contains(&def.id.to_string()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pet(stage: EvolutionStage) -> PetState {
        let mut pet = PetState::new();
        pet.stage = stage;
        pet
    }

    #[test]
    fn test_first_hatch_unlocks_on_baby() {
        let mut stats = GameStats::new();
        let pet = make_pet(EvolutionStage::Baby);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "first_hatch"));
    }

    #[test]
    fn test_first_hatch_not_on_egg() {
        let mut stats = GameStats::new();
        let pet = make_pet(EvolutionStage::Egg);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(!unlocked.iter().any(|a| a.id == "first_hatch"));
    }

    #[test]
    fn test_growing_up_on_child() {
        let mut stats = GameStats::new();
        let pet = make_pet(EvolutionStage::Child);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "growing_up"));
    }

    #[test]
    fn test_all_grown_up_on_adult() {
        let mut stats = GameStats::new();
        let pet = make_pet(EvolutionStage::Adult);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "all_grown_up"));
    }

    #[test]
    fn test_streak_achievements() {
        let mut stats = GameStats::new();
        stats.longest_streak = 7;
        let pet = make_pet(EvolutionStage::Egg);

        let unlocked = check_achievements(&mut stats, &pet);
        let ids: Vec<&str> = unlocked.iter().map(|a| a.id.as_str()).collect();
        assert!(ids.contains(&"streak_3"));
        assert!(ids.contains(&"streak_7"));
        assert!(!ids.contains(&"streak_30"));
    }

    #[test]
    fn test_feed_100_achievement() {
        let mut stats = GameStats::new();
        stats.total_feeds = 100;
        let pet = make_pet(EvolutionStage::Egg);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "feed_100"));
    }

    #[test]
    fn test_play_50_achievement() {
        let mut stats = GameStats::new();
        stats.total_plays = 50;
        let pet = make_pet(EvolutionStage::Egg);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "play_50"));
    }

    #[test]
    fn test_sleep_50_achievement() {
        let mut stats = GameStats::new();
        stats.total_sleeps = 50;
        let pet = make_pet(EvolutionStage::Egg);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "sleep_50"));
    }

    #[test]
    fn test_comeback_achievement() {
        let mut stats = GameStats::new();
        stats.total_pets_raised = 2;
        let pet = make_pet(EvolutionStage::Egg);
        let unlocked = check_achievements(&mut stats, &pet);
        assert!(unlocked.iter().any(|a| a.id == "comeback"));
    }

    #[test]
    fn test_no_duplicate_unlocks() {
        let mut stats = GameStats::new();
        let pet = make_pet(EvolutionStage::Adult);

        let first = check_achievements(&mut stats, &pet);
        assert!(!first.is_empty());

        let second = check_achievements(&mut stats, &pet);
        // All already unlocked, nothing new
        for a in &second {
            assert!(!first.iter().any(|f| f.id == a.id));
        }
    }

    #[test]
    fn test_get_all_achievements_count() {
        let stats = GameStats::new();
        let all = get_all_achievements(&stats);
        assert_eq!(all.len(), 10);
        assert!(all.iter().all(|a| !a.unlocked));
    }

    #[test]
    fn test_get_all_shows_unlock_status() {
        let mut stats = GameStats::new();
        stats.unlocked.push("first_hatch".to_string());
        let all = get_all_achievements(&stats);
        let first = all.iter().find(|a| a.id == "first_hatch").unwrap();
        assert!(first.unlocked);
    }
}
