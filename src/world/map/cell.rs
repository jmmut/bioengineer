pub use i32 as Pressure;
use TileType::*;

#[derive(Clone, Copy, PartialEq, Debug)]
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
    Air = 26,
    Wire = 5,
    MachineAssembler = 12,
    MachineAirCleaner = 30,
    MachineDrill = 13,
    MachineSolarPanel = 21,
    MachineShip = 28,
    DirtyWaterSurface = 6,
    CleanWaterSurface = 7,
    DirtyWaterWall = 14,
    CleanWaterWall = 15,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ExtraTextures {
    ZoomedRobot = 32,
    Robot = 4,
    Movement = 22,
}

pub struct TextureIndex {
    index: usize
}

impl TextureIndex {
    pub fn get_index(&self) -> usize {
        self.index
    }
}

impl From<TileType> for TextureIndex {
    fn from(texture: TileType) -> Self {
        if texture == Unset {
            panic!("Tried to draw an Unset texture!");
        }
        Self {index: texture as usize}
    }
}

impl From<ExtraTextures> for TextureIndex {
    fn from(texture: ExtraTextures) -> Self {
        Self {index: texture as usize}
    }
}

impl TileType {
    pub fn get_index(&self) -> usize {
        TextureIndex::from(*self).get_index()
    }
}

impl ExtraTextures {
    pub fn get_index(&self) -> usize {
        TextureIndex::from(*self).get_index()
    }
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

pub fn is_walkable_horizontal(tile: TileType) -> bool {
    [
        FloorRock,
        FloorDirt,
        Stairs,
        Wire,
        MachineAssembler,
        MachineAirCleaner,
        MachineDrill,
        MachineSolarPanel,
        MachineShip,
    ]
    .contains(&tile)
}

pub fn is_walkable_vertical(target_tile: TileType, origin_tile: TileType) -> bool {
    let vertical_tiles = [Stairs];
    vertical_tiles.contains(&target_tile) && vertical_tiles.contains(&origin_tile)
}

/// Returns whether the tile image is tall enough that it would cover a robot behind this tile.
/// This function is used to reduce the opacity for such tiles.
pub fn is_covering(tile: TileType) -> bool {
    [
        Stairs,
        WallRock,
        WallDirt,
        DirtyWaterWall,
        CleanWaterWall,
        MachineAssembler,
        MachineAirCleaner,
        MachineDrill,
        MachineSolarPanel,
        MachineShip,
    ]
    .contains(&tile)
}

pub fn is_networkable(tile: TileType) -> bool {
    [
        Wire,
        MachineAssembler,
        MachineAirCleaner,
        MachineDrill,
        MachineSolarPanel,
    ]
    .contains(&tile)
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
