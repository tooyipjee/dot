use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    // Lifetime counters
    pub total_feeds: u64,
    pub total_plays: u64,
    pub total_sleeps: u64,
    pub total_pets_raised: u64,

    // Streak tracking
    pub current_streak: u32,
    pub longest_streak: u32,
    pub last_care_date: Option<String>, // "YYYY-MM-DD"

    // Achievement unlocks (store achievement IDs)
    pub unlocked: Vec<String>,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            total_feeds: 0,
            total_plays: 0,
            total_sleeps: 0,
            total_pets_raised: 1, // starts at 1 for first pet
            current_streak: 0,
            longest_streak: 0,
            last_care_date: None,
            unlocked: Vec::new(),
        }
    }

    pub fn record_feed(&mut self) {
        self.total_feeds += 1;
        self.update_streak();
    }

    pub fn record_play(&mut self) {
        self.total_plays += 1;
        self.update_streak();
    }

    pub fn record_sleep(&mut self) {
        self.total_sleeps += 1;
        self.update_streak();
    }

    pub fn record_revive(&mut self) {
        self.total_pets_raised += 1;
    }

    fn update_streak(&mut self) {
        let today = Utc::now().format("%Y-%m-%d").to_string();

        match &self.last_care_date {
            Some(last_date) => {
                if *last_date == today {
                    // Already cared today, no streak change
                    return;
                }

                // Check if yesterday
                let yesterday = (Utc::now() - chrono::Duration::days(1))
                    .format("%Y-%m-%d")
                    .to_string();

                if *last_date == yesterday {
                    // Consecutive day - increment streak
                    self.current_streak += 1;
                } else {
                    // Gap in care - reset streak
                    self.current_streak = 1;
                }
            }
            None => {
                // First care ever
                self.current_streak = 1;
            }
        }

        self.last_care_date = Some(today);
        if self.current_streak > self.longest_streak {
            self.longest_streak = self.current_streak;
        }
    }

    pub fn unlock_achievement(&mut self, id: &str) -> bool {
        if self.unlocked.contains(&id.to_string()) {
            return false;
        }
        self.unlocked.push(id.to_string());
        true
    }
}

impl Default for GameStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stats() {
        let stats = GameStats::new();
        assert_eq!(stats.total_feeds, 0);
        assert_eq!(stats.total_plays, 0);
        assert_eq!(stats.total_sleeps, 0);
        assert_eq!(stats.total_pets_raised, 1);
        assert_eq!(stats.current_streak, 0);
        assert_eq!(stats.longest_streak, 0);
        assert!(stats.last_care_date.is_none());
        assert!(stats.unlocked.is_empty());
    }

    #[test]
    fn test_record_feed_increments() {
        let mut stats = GameStats::new();
        stats.record_feed();
        assert_eq!(stats.total_feeds, 1);
        stats.record_feed();
        assert_eq!(stats.total_feeds, 2);
    }

    #[test]
    fn test_record_play_increments() {
        let mut stats = GameStats::new();
        stats.record_play();
        assert_eq!(stats.total_plays, 1);
    }

    #[test]
    fn test_record_sleep_increments() {
        let mut stats = GameStats::new();
        stats.record_sleep();
        assert_eq!(stats.total_sleeps, 1);
    }

    #[test]
    fn test_record_revive_increments() {
        let mut stats = GameStats::new();
        assert_eq!(stats.total_pets_raised, 1);
        stats.record_revive();
        assert_eq!(stats.total_pets_raised, 2);
    }

    #[test]
    fn test_first_care_starts_streak() {
        let mut stats = GameStats::new();
        stats.record_feed();
        assert_eq!(stats.current_streak, 1);
        assert_eq!(stats.longest_streak, 1);
        assert!(stats.last_care_date.is_some());
    }

    #[test]
    fn test_same_day_care_no_streak_change() {
        let mut stats = GameStats::new();
        stats.record_feed();
        assert_eq!(stats.current_streak, 1);
        stats.record_feed();
        assert_eq!(stats.current_streak, 1); // same day, no change
    }

    #[test]
    fn test_streak_resets_on_gap() {
        let mut stats = GameStats::new();
        // Simulate care two days ago
        stats.last_care_date = Some("2020-01-01".to_string());
        stats.current_streak = 5;
        stats.longest_streak = 5;

        stats.record_feed(); // today is not Jan 2, so gap resets
        assert_eq!(stats.current_streak, 1);
        assert_eq!(stats.longest_streak, 5); // longest preserved
    }

    #[test]
    fn test_unlock_achievement() {
        let mut stats = GameStats::new();
        assert!(stats.unlock_achievement("first_hatch"));
        assert!(!stats.unlock_achievement("first_hatch")); // duplicate
        assert_eq!(stats.unlocked.len(), 1);
    }

    #[test]
    fn test_longest_streak_preserved() {
        let mut stats = GameStats::new();
        stats.current_streak = 10;
        stats.longest_streak = 10;
        stats.last_care_date = Some("2020-01-01".to_string());

        // Gap resets current but longest stays
        stats.record_feed();
        assert_eq!(stats.current_streak, 1);
        assert_eq!(stats.longest_streak, 10);
    }
}
