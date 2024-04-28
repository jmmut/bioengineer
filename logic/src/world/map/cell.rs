pub type Health = i16;
pub type Pressure = i32;
pub type Pollution = i16;
use TileType::*;

pub const DEFAULT_HEALTH: Health = 5;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub tile_type: TileType,
    pub pressure: Pressure,
    pub next_pressure: Pressure,
    pub renderable_pressure: Pressure,
    // pub pollution: Pollution,
    pub health: Health,
    pub can_flow_out: bool,
}

impl Cell {
    pub fn new(tile_type: TileType) -> Self {
        Cell {
            tile_type,
            pressure: 0,
            next_pressure: 0,
            renderable_pressure: 0,
            can_flow_out: false,
            health: 0,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(TileType::Unset)
    }
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
    MachineStorage = 0,
    TreeHealthy = 8,
    TreeSparse = 9,
    TreeDying = 10,
    TreeDead = 11,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ExtraTextures {
    DirtyWaterSurface = 6,
    CleanWaterSurface = 7,
    DirtyWaterWall = 14,
    CleanWaterWall = 15,
    ZoomedRobot = 32,
    Robot = 4,
    Movement = 22,
    Ship = 31,
}

#[derive(Clone, Copy)]
pub struct TextureIndex {
    index: usize,
}

impl TextureIndex {
    pub fn get_index(&self) -> usize {
        self.index
    }
}

pub trait TextureIndexTrait {
    fn get_index(&self) -> usize;
}

impl From<&dyn TextureIndexTrait> for TextureIndex {
    fn from(t: &dyn TextureIndexTrait) -> Self {
        Self {
            index: t.get_index(),
        }
    }
}

impl From<TileType> for TextureIndex {
    fn from(texture: TileType) -> Self {
        if texture == Unset {
            panic!("Tried to draw an Unset texture!");
        }
        Self {
            index: texture as usize,
        }
    }
}

impl From<ExtraTextures> for TextureIndex {
    fn from(texture: ExtraTextures) -> Self {
        Self {
            index: texture as usize,
        }
    }
}

impl TextureIndexTrait for TileType {
    fn get_index(&self) -> usize {
        TextureIndex::from(*self).get_index()
    }
}

impl TextureIndexTrait for ExtraTextures {
    fn get_index(&self) -> usize {
        TextureIndex::from(*self).get_index()
    }
}

pub fn is_liquid(tile: TileType) -> bool {
    [
        // DirtyWaterWall,
        // CleanWaterWall,
        // DirtyWaterSurface,
        // CleanWaterSurface,
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
        MachineStorage,
        TreeHealthy,
        TreeSparse,
        TreeDying,
        TreeDead,
    ]
    .contains(&tile)
}

pub fn is_walkable_vertical(target_tile: TileType, origin_tile: TileType) -> bool {
    let vertical_tiles = [Stairs];
    vertical_tiles.contains(&target_tile) && vertical_tiles.contains(&origin_tile)
}

pub fn is_floodable_horizontal(tile: TileType) -> bool {
    ![Unset, WallDirt, WallRock].contains(&tile)
}

pub fn is_floodable_from_above(tile: TileType) -> bool {
    ![Unset, WallDirt, WallRock].contains(&tile)
}

pub fn is_floodable_from_below(tile: TileType) -> bool {
    ![
        Unset,
        WallDirt,
        WallRock,
        FloorDirt,
        FloorRock,
        TreeHealthy,
        TreeSparse,
        TreeDying,
        TreeDead,
    ]
    .contains(&tile)
}

/// Returns whether the tile image is tall enough that it would cover a robot behind this tile.
/// This function is used to reduce the opacity for such tiles.
pub fn is_covering(tile: TileType) -> bool {
    [
        Stairs,
        WallRock,
        WallDirt,
        // DirtyWaterWall, //TODO? do we need this kind of opacity test if there's no robots?
        // CleanWaterWall,
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
        MachineShip,
        MachineStorage,
    ]
    .contains(&tile)
}

pub fn ages(tile: TileType) -> bool {
    [TreeHealthy, TreeSparse, TreeDying].contains(&tile)
}

pub fn transition_aging_tile(cell: &mut Cell) {
    if cell.tile_type == TreeHealthy {
        cell.tile_type = TreeSparse;
        cell.health = DEFAULT_HEALTH;
    } else if cell.tile_type == TreeSparse {
        cell.tile_type = TreeDying;
        cell.health = DEFAULT_HEALTH;
    } else if cell.tile_type == TreeDying {
        cell.tile_type = TreeDead;
        cell.health = 0;
    }
}
