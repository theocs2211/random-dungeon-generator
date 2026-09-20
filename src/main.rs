use isaac_dungeon_generator::{RoomType, generate_rooms};

fn main() {
    let grid = generate_rooms(10, None);

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

