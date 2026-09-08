use isaac_dungeon_generator::{generate_rooms, RoomType};

fn main() {
    let grid = generate_rooms(1);

    for linha in grid {
        for room in linha {
            match room {
                Some(RoomType::Start) => print!("[ S ]"),
                Some(RoomType::Normal) => print!("[ N ]"),
                Some(RoomType::Boss) => print!("[ B ]"),
                Some(RoomType::Treasure) => print!("[ T ]"),
                None => print!("[ . ]"),
            }
        }
        println!();
    }
}