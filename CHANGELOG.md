# Changelog

## v1.0.0

### Added
- Streaks: daily care streak tracking displayed in pet info line
- Achievements: 10 unlockable achievements (evolution milestones, care streaks, action counters, revival)
- Achievements panel: trophy button (+) opens a full overlay showing all badges
- Achievement toast: brief notification when a new achievement is unlocked
- Game stats persistence: lifetime counters saved separately from pet state (`~/.config/dot/game-stats.json`)
- Unit tests: 57 tests covering needs decay, evolution, lifecycle, state actions, stats, and achievements
- CI: GitHub Actions workflow for testing and building on push/PR
- Release: GitHub Actions workflow for building and publishing `.dmg` on version tags
- MIT LICENSE file

### Changed
- Action commands (feed, play, sleep, revive) now return pet state along with any newly unlocked achievements
- Trimmed `tokio` features from `full` to `rt, macros`
- Bumped version to 1.0.0
- Updated metadata in Cargo.toml, tauri.conf.json, and package.json
- Added `category: entertainment` to macOS bundle config
