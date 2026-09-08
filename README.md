# 🗝️ Random Dungeon Generator

A fast, robust procedural dungeon layout generator written in idiomatic Rust, inspired by the map generation algorithm of *The Binding of Isaac*.

It constructs a **13x13 grid** using a random-walk strategy, dynamically identifies optimal dead-ends via Manhattan distance calculations, and places special rooms (**Boss** and **Treasure**) at the furthest points from the start location.

---

## ✨ Features

- 🎲 **Random-Walk Strategy:** Generates realistic, connected room layouts.
- 🎯 **Special Room Placement:** Automatically locates valid dead-ends for **Boss** and **Treasure** rooms based on distance from the start position.
- 🧪 **Stress-Tested:** Built-in test suite executing 1,000+ iterations per run to ensure algorithmic stability.
- 📚 **Integrated Doctests:** Executable documentation samples verified automatically via `cargo test`.

---

## 🛠️ Usage Example

Add the library module to your Rust project and generate a grid:

```rust
use isaac_dungeon::{generate_rooms, RoomType};

fn main() {
    // Generate a dungeon floor with a minimum target of 3 rooms
    let grid = generate_rooms(10);

    // The starting room is guaranteed to be at center position (5, 5)
    assert_eq!(grid[5][5], Some(RoomType::Start));

    // Inspect grid layout
    for row in grid.iter() {
        for room in row.iter() {
            match room {
                Some(RoomType::Start) => print!("[S]"),
                Some(RoomType::Normal) => print!("[N]"),
                Some(RoomType::Boss) => print!("[B]"),
                Some(RoomType::Treasure) => print!("[T]"),
                None => print!(" . "),
            }
        }
        println!();
    }
}
```

---

## 🧪 Running Tests & Generating Docs

To run the full suite of unit, stress, and documentation tests:

```bash
cargo test
```

To build and open the full interactive HTML documentation in your browser:

```bash
cargo doc --open
```
