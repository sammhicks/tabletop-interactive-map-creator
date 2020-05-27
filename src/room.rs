use crate::tile;

#[derive(Clone, Debug, PartialEq)]
pub struct Room {
    pub tile_material: tile::Material,
}

pub type Rooms = crate::list::List<Room>;
