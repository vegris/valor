<img src="assets/logo.png" alt="Project Logo" width="500"/>

A work in progress implementation of Heroes III Battle Mode written in Rust.

## Screenshots

<p align="center">
    <img src="assets/harpy_attacks_devil.webp?raw=true" width="400" alt="Screenshot: Harpy attacks Devil">
    <img src="assets/archangel_attacks_archdevil.webp?raw=true" width="400" alt="Screenshot: ArchAngel attacks ArchDevil">
</p>
<p align="center">
    <img src="assets/armageddon.webp?raw=true" width="400" alt="Screenshot: Armageddon">
    <img src="assets/archmage_shoots_black_knight.webp?raw=true" width="400" alt="Screenshot: ArchMage shoots BlackKnight">
</p>

## Setup

### Requirements

* **Rust compiler**

Any relatively recent stable version should work.

* **SDL2 libraries** (SDL2, SDL2_mixer, SDL2_ttf)

**Important**: SDL3 (via sdl2-compat) is currently incompatible — graphics appear as black squares. You must use a real SDL2 installation.

* **Heroes of Might and Magic III**

The engine is developed with the **GoG Complete Edition**, but a [demo version](https://www.gamefront.com/games/heroes-of-might-magic-iii/file/heroes-of-might-magic-iii-the-restoration-of-erathia-demo-english) is also verified to work with minor tweaks.

### Configuration

1. Copy `config.default.ron` to `config.ron`:

```sh
cp config.default.ron config.ron
```

2. Open `config.ron` and adjust the following settings:
    * `game_folder`: Path to your Heroes 3 installation (where the `.exe` is located).
    * `ttf_font`: Path to a `.ttf` font file on your system (used for creature counts).

### Building and running

Development build:
```sh
cargo build
```

Release (optimized) build:
```sh
cargo build --release
```

After compiling, the executable will be located in:
* Development: `./target/debug/Valor`
* Release: `./target/release/Valor`

### Using the demo version

If you are using the demo version, you’ll need to adjust it's files for the engine to locate them.

1. Rename files in `Data` folder to be capitalized lowercase:
    - `Data/H3BITMAP.LOD` -> `Data/H3bitmap.lod`
    - `Data/H3SPRITE.LOD` -> `Data/H3sprite.lod`
    - `Data/HEROES3.SND` -> `Data/Heroes3.snd`
2. Rename `MP3` folder to `Mp3`
3. Make 3 copies of `Mp3/COMBAT01.MP3` and name them `COMBAT02.MP3`, `COMBAT03.MP3`, `COMBAT04.MP3`

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
