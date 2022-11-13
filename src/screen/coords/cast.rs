use crate::screen::drawing_state::{SubCellIndex, SubTilePosition, TilePosition};
use crate::world::map::CellIndex;

/// Uses type inference to do an explicit cast, but without writing the target type.
pub trait Cast<T> {
    fn cast(&self) -> T;
}

impl Cast<CellIndex> for SubCellIndex {
    fn cast(&self) -> CellIndex {
        CellIndex::new(self.x as i32, self.y as i32, self.z as i32)
    }
}
impl Cast<SubCellIndex> for CellIndex {
    fn cast(&self) -> SubCellIndex {
        SubCellIndex::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl Cast<TilePosition> for SubTilePosition {
    fn cast(&self) -> TilePosition {
        TilePosition::new(self.x as i32, self.y as i32)
    }
}

impl Cast<SubTilePosition> for TilePosition {
    fn cast(&self) -> SubTilePosition {
        SubTilePosition::new(self.x as f32, self.y as f32)
    }
}
