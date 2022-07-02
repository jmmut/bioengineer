pub use i32 as Pressure;
use TileType::*;

#[derive(Clone, Copy)]
pub struct Cell {
    pub tile_type: TileType,
    pub pressure: Pressure,
    pub next_pressure: Pressure,
    pub can_flow_out: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum TileType {
    Unset = -1,
    // Helper = 2,
    WallRock = 16,
    WallDirt = 24,
    FloorRock = 17,
    FloorDirt = 20,
    Stairs = 18,
    Air = 29,
    Wire = 5,
    MachineAssembler = 12,
    MachineDrill = 13,
    MachineSolarPanel = 21,
    MachineShip = 28,
    DirtyWaterSurface = 6,
    CleanWaterSurface = 7,
    DirtyWaterWall = 14,
    CleanWaterWall = 15,
    Robot = 4,
}

pub fn is_liquid(tile: TileType) -> bool {
    [
        DirtyWaterWall,
        CleanWaterWall,
        DirtyWaterSurface,
        CleanWaterSurface,
        Air,
    ]
    .contains(&tile)
}

pub fn is_liquid_or_air(tile: TileType) -> bool {
    tile == Air || is_liquid(tile)
}

impl Cell {
    pub fn new(tile_type: TileType) -> Self {
        Cell {
            tile_type,
            pressure: 0,
            next_pressure: 0,
            can_flow_out: false,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(TileType::Unset)
    }
}
