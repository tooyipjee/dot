# dot

A cute Tamagotchi-style virtual pet that lives in your macOS menu bar.

## Features

- **Menu Bar Pet**: Lives quietly in your menu bar, always accessible
- **Pet Care**: Feed, play with, and put your pet to sleep
- **Evolution**: Watch your pet grow from Egg to Baby to Teen to Adult
- **Stats System**: Monitor hunger, happiness, energy, and health
- **Persistence**: Your pet survives app restarts
- **Death & Revival**: If neglected, your pet may pass away, but you can start fresh with a new egg

## Requirements

- macOS 10.15+
- Rust (for building)

## Building

```bash
# Install Rust if you haven't
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the app
cd src-tauri
cargo build --release

# Run
./target/release/dot
```

## Usage

- **Click the menu bar icon** to toggle the pet window
- **Right-click the icon** for quick actions (Feed, Play, Sleep)
- **Keep your pet happy** by regularly feeding, playing, and letting it rest

## Evolution Timeline

- **Egg**: Birth stage (5 minutes)
- **Baby**: First evolution (15 minutes from birth)
- **Teen**: Second evolution (30 minutes from birth)
- **Adult**: Final form

## Pet Care Tips

- Hunger decays fastest - feed regularly
- Playing uses energy - let your pet rest afterward
- If any stat gets critically low, health will decline
- If health reaches 0, your pet will pass away

## Tech Stack

- **Frontend**: Vanilla JavaScript, HTML5 Canvas
- **Backend**: Rust with Tauri v2
- **Persistence**: JSON file storage

## License

MIT
