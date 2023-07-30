use super::TileType::*;
use crate::world::fluids::tests::assert_steps_with_types;
use crate::world::map::CellIndex;

#[test]
fn test_basic_floodable_top_view() {
    #[rustfmt::skip]
    let cells = vec![(5, DirtyWaterSurface), (0, FloorRock)];
    let expected = cells.clone();
    assert_steps_with_types(
        vec![cells, expected],
        CellIndex::new(0, 0, 0),
        CellIndex::new(0, 0, 1),
    );
}
