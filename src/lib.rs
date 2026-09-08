//! # Dungeon Generator
//!
//! A procedural dungeon generation library inspired by *The Binding of Isaac*.

use rand::prelude::IndexedRandom;

/// Represents the possible types of rooms on the dungeon map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomType {
    Start,
    Normal,
    Boss,
    Treasure,
}

/// Generates a 13x13 grid containing the dungeon layout.
///
/// The `num_of_rooms` parameter sets the desired total count of rooms.
/// The minimum enforced value is 3 to guarantee enough space for special rooms.
///
/// # Examples
///
/// ```
/// use isaac_dungeon_generator::{generate_rooms, RoomType};
///
/// let grid = generate_rooms(10);
/// assert_eq!(grid[5][5], Some(RoomType::Start));
/// ```
pub fn generate_rooms(num_of_rooms: usize) -> [[Option<RoomType>; 13]; 13] {
    let num_of_rooms = num_of_rooms.max(3);

    loop {
        let mut grid: [[Option<RoomType>; 13]; 13] = [[None; 13]; 13];
        grid[5][5] = Some(RoomType::Start);

        for _ in 0..(num_of_rooms - 1) {
            generate_room(&mut grid);
        }

        if generate_specials_rooms(&mut grid) {
            return grid;
        }
    }
}

fn generate_specials_rooms(grid: &mut [[Option<RoomType>; 13]; 13]) -> bool {
    let mut valid_rooms: Vec<(usize, usize)> = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, room) in line.iter().enumerate() {
            if room.is_some() && !has_multiple_neighbors(*grid, x, y) && *room != Some(RoomType::Start) {
                valid_rooms.push((x, y));
            }
        }
    }

    let mut distances = valid_rooms.iter().map(|(x, y)| {
        let distance = x.abs_diff(5) + y.abs_diff(5);
        (distance, x, y)
    }).collect::<Vec<_>>();

    distances.sort_by(|a, b| b.0.cmp(&a.0));


    let Some([(_, x1, y1), (_, x2, y2)]) = distances.get(..2) else { return false };

    grid[**y1][**x1] = Some(RoomType::Boss);
    grid[**y2][**x2] = Some(RoomType::Treasure);

    true
}
fn generate_room(grid: &mut [[Option<RoomType>; 13]; 13]) {
    let mut valid_spawn_positions: Vec<(usize, usize)> = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, room) in line.iter().enumerate() {
            if room.is_some() {
                //Up
                if y > 0 && grid[y - 1][x].is_none() && !has_multiple_neighbors(*grid, x, y - 1) {
                    valid_spawn_positions.push((y - 1, x));
                }
                //Down
                if y < 12 && grid[y + 1][x].is_none() && !has_multiple_neighbors(*grid, x, y + 1) {
                    valid_spawn_positions.push((y + 1, x));
                }
                //Left
                if x > 0 && grid[y][x - 1].is_none() && !has_multiple_neighbors(*grid, x - 1 , y) {
                    valid_spawn_positions.push((y, x - 1));
                }
                //Right
                if x < 12 && grid[y][x + 1].is_none() && !has_multiple_neighbors(*grid, x + 1, y) {
                    valid_spawn_positions.push((y, x + 1));
                }
            }
        }
    }

    let mut rng = rand::rng();

    if let Some(generated_room) = valid_spawn_positions.choose(&mut rng) {
        grid[generated_room.0][generated_room.1] = Some(RoomType::Normal);
    }
}

fn has_multiple_neighbors(grid: [[Option<RoomType>; 13]; 13], x: usize, y: usize) -> bool {
    let mut count = 0;

    //Up
    if y > 0 && grid[y - 1][x].is_some() {
        count += 1;
    }
    //Down
    if y < 12 && grid[y + 1][x].is_some() {
        count += 1;
    }
    //Left
    if x > 0 && grid[y][x - 1].is_some() {
        count += 1;
    }
    //Right
    if x < 12 && grid[y][x + 1].is_some() {
        count += 1;
    }

    matches!(count, 2..)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_rooms() {
        for _i in 0..1000 {
            let grid = generate_rooms(30);

            let mut count = 0;

            for line in grid {
                for room in line {
                    match room {
                        None => continue,
                        _ => count += 1,
                    }
                }
            }

            assert_eq!(count, 30);
        }
    }

    #[test]
    fn has_specials_rooms() {
        for _i in 0..1000 {
            let grid = generate_rooms(3);

            let mut special_rooms = 0;
            let mut start_room = 0;

            for line in grid {
                for room in line {
                    match room {
                        Some(RoomType::Start) => start_room += 1,
                        Some(RoomType::Normal) => continue,
                        None => continue,
                        _ => special_rooms += 1,
                    }
                }
            }

            assert_eq!(special_rooms, 2);
            assert_eq!(start_room, 1);
        }
    }
}