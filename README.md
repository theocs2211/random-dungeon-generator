- 🎯 **Special Room Placement:** Automatically locates valid dead-ends for **Boss** and **Treasure** rooms based on distance from the start position.
- 🎲 **Deterministic Generation:** Support for seed-based generation using `Option<u64>` for reproducible layouts.
- 🧪 **Stress-Tested:** Built-in test suite executing 1,000+ iterations per run to ensure algorithmic stability.
- 📚 **Integrated Doctests:** Executable documentation samples verified automatically via `cargo test`.

---

## 🛠️ Usage Example

Add the library module to your Rust project and generate a grid:

```rust
use isaac_dungeon::{generate_rooms, RoomType};

fn main() {
    // 1. Generate a random dungeon layout (passing None for seed)
    let grid = generate_rooms(10, None);

    // 2. Or generate a reproducible dungeon layout using a specific seed
    // let grid = generate_rooms(10, Some(12345));

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

🧪 Running Tests & Generating Docs
To run the full suite of unit, stress, and documentation tests:
```Bash
cargo test
```
To build and open the full interactive HTML documentation in your browser:
```Bash
cargo doc --open
```

