# Procedural World Generation
This repo showcases how to procedurally generate a simple world using perlin noise. It also includes a simple animated player character that can move around the world.

Refer [Tutorial Section](#tutorial-section) for a link to a youtube video tutorial.

Built in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine.

![screenshot](/screenshot.png)

# Tutorial <a name="tutorial-section"></a>
[![youtube](https://img.youtube.com/vi/NSDdJeCmXXE/0.jpg)](https://youtu.be/NSDdJeCmXXE)

## Usage
- Clone the repo
```bash
git clone git@github.com:bones-ai/rust-procedural-world.git
cd rust-procedural-world
```
- Run the simulation
```bash
cargo run
```

## Configurations
- The project config file is located at `src/configs.rs`
- To modify the terrain generation, update the thresholds in `src/terrain.rs`
