use crate::drawing::coords::cast::Cast;
use crate::drawing::{SubCellIndex, SubTilePosition, TilePosition};
use crate::world::map::CellIndex;

#[allow(dead_code)]
pub fn cell_to_tile_unwrapped(
    min_cell: &CellIndex,
    max_cell: &CellIndex,
    i_x: i32,
    i_y: i32,
    i_z: i32,
) -> TilePosition {
    let cell = CellIndex::new(i_x, i_y, i_z);
    cell_to_tile(cell, min_cell, max_cell)
}

pub fn cell_to_tile(cell: CellIndex, min_cell: &CellIndex, max_cell: &CellIndex) -> TilePosition {
    let offset = cell_offset(min_cell, max_cell);
    cell_to_tile_offset(cell - offset)
}

pub fn subcell_to_subtile(
    subcell: SubCellIndex,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
) -> SubTilePosition {
    let offset = cell_offset(min_cell, max_cell).cast();
    subcell_to_subtile_offset(subcell - offset)
}

#[allow(dead_code)]
pub fn tile_to_cell(tile: TilePosition, min_cell: &CellIndex, max_cell: &CellIndex) -> CellIndex {
    let offset = cell_offset(min_cell, max_cell);
    tile_to_cell_offset(tile) + offset
}

pub fn subtile_to_subcell(
    tile: SubTilePosition,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
) -> SubCellIndex {
    let offset = cell_offset(min_cell, max_cell);
    subtile_to_subcell_offset(tile) + offset.cast()
}

/// NOTE we are mixing min_cell and max_cell !!! this is intended
pub fn cell_offset(min_cell: &CellIndex, max_cell: &CellIndex) -> CellIndex {
    CellIndex::new(min_cell.x, max_cell.y, min_cell.z)
}

pub fn cell_to_tile_offset(cell: CellIndex) -> TilePosition {
    TilePosition::new(cell.x - cell.z, cell.x + cell.z - 2 * cell.y)
}

pub fn subcell_to_subtile_offset(cell: SubCellIndex) -> SubTilePosition {
    SubTilePosition::new(cell.x - cell.z, cell.x + cell.z - 2.0 * cell.y)
}

#[allow(dead_code)]
pub fn tile_to_cell_offset(tile_offset: TilePosition) -> CellIndex {
    CellIndex::new(
        (tile_offset.x + tile_offset.y) / 2,
        0,
        (-tile_offset.x + tile_offset.y) / 2,
    )
}

pub fn subtile_to_subcell_offset(subtile_offset: SubTilePosition) -> SubCellIndex {
    SubCellIndex::new(
        (subtile_offset.x + subtile_offset.y) / 2.0,
        0.0,
        (-subtile_offset.x + subtile_offset.y) / 2.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_tile_basic() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(10, 10, 10);
        assert_eq!(
            cell_to_tile_unwrapped(&min_cell, &max_cell, 0, max_cell.y, 0),
            TilePosition::new(0, 0)
        );
        assert_eq!(
            cell_to_tile_unwrapped(&min_cell, &max_cell, 1, max_cell.y, 1),
            TilePosition::new(0, 2)
        );
        assert_eq!(
            cell_to_tile_unwrapped(&min_cell, &max_cell, 1, max_cell.y, 0),
            TilePosition::new(1, 1)
        );
    }

    #[test]
    fn position_tile_min_cell() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(10, 10, 10);
        let tile = cell_to_tile_unwrapped(&min_cell, &max_cell, min_cell.x, max_cell.y, min_cell.z);
        assert_eq!(tile, TilePosition::new(0, 0));
    }

    #[test]
    fn position_tile_negative() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        assert_eq!(
            cell_to_tile_unwrapped(&min_cell, &max_cell, min_cell.x, max_cell.y, min_cell.z),
            TilePosition::new(0, 0)
        );
        assert_eq!(
            cell_to_tile_unwrapped(
                &min_cell,
                &max_cell,
                min_cell.x + 1,
                max_cell.y,
                min_cell.z + 1
            ),
            TilePosition::new(0, 2)
        );
    }

    #[test]
    fn position_tile_height() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        assert_eq!(
            cell_to_tile_unwrapped(
                &min_cell,
                &max_cell,
                min_cell.x + 1,
                max_cell.y,
                min_cell.z + 1
            ),
            cell_to_tile_unwrapped(&min_cell, &max_cell, min_cell.x, max_cell.y - 1, min_cell.z)
        );
    }

    fn cell_to_tile_to_cell(initial_cell: CellIndex) {
        let min_cell = CellIndex::new(-10, -10, -10);
        let max_cell = CellIndex::new(10, initial_cell.y, 10);
        let tile = cell_to_tile_unwrapped(
            &min_cell,
            &max_cell,
            initial_cell.x,
            initial_cell.y,
            initial_cell.z,
        );
        let final_cell = tile_to_cell(tile, &min_cell, &max_cell);
        assert_eq!(final_cell, initial_cell);
    }
    #[test]
    fn test_cell_to_tile_to_cell() {
        cell_to_tile_to_cell(CellIndex::new(0, 0, 0));
        cell_to_tile_to_cell(CellIndex::new(1, 0, 0));
        cell_to_tile_to_cell(CellIndex::new(0, 1, 0));
        cell_to_tile_to_cell(CellIndex::new(0, 0, 1));
    }

    #[test]
    fn test_cell_to_tile() {
        let subtile = subcell_to_subtile_offset(SubCellIndex::new(0.5, 0.0, 0.5));
        assert_eq!(subtile, SubTilePosition::new(0.0, 1.0));

        let subtile = subcell_to_subtile_offset(SubCellIndex::new(1.0, 0.0, 0.0));
        assert_eq!(subtile, SubTilePosition::new(1.0, 1.0));

        let subtile = subcell_to_subtile_offset(SubCellIndex::new(0.0, 0.0, 1.0));
        assert_eq!(subtile, SubTilePosition::new(-1.0, 1.0));
    }
    fn cell_to_tile_to_cell_offset(initial_cell: CellIndex) {
        let tile = cell_to_tile_offset(initial_cell);
        let final_cell = tile_to_cell_offset(tile);
        assert_eq!(final_cell, initial_cell);
    }
    #[test]
    fn test_cell_to_tile_to_cell_offset() {
        cell_to_tile_to_cell_offset(CellIndex::new(10, 0, 10));
    }
}
