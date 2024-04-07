use crate::world::fluids::tests::assert_steps_with_types;
use crate::world::map::CellIndex;
use crate::world::map::TileType::{Air, FloorRock};

#[test]
fn test_basic_floodable_top_view() {
    #[rustfmt::skip]
    let cells = vec![(5, Air), (0, FloorRock)];
    let expected = cells.clone();
    // let expected = vec![(4, Air), (1, FloorRock)];
    assert_steps_with_types(
        vec![cells, expected],
        CellIndex::new(0, 0, 0),
        CellIndex::new(0, 0, 1),
    );
}
