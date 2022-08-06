use crate::prelude::*;

// 4,000 tiles
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;


#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall, 
    Floor,
}


pub struct Map {
    pub tiles: Vec<TileType>
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                // set the TileType depending on the Index position
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    },
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    // check if player is within the bounds of the Map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH 
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // determine if player Tile, only if its a FLoor type
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) 
            && self.tiles[map_idx(point.x, point.y)] == TileType::Floor 
    }

    // return current player index
    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

}

// get the Map index (row_first encoding)
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
