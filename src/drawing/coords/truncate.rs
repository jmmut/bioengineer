use crate::drawing::SubCellIndex;
use crate::map::trunc::trunc_towards_neg_inf_f;
use crate::map::CellIndex;

pub fn truncate_cell_offset(subcell_diff: SubCellIndex) -> (CellIndex, SubCellIndex) {
    let mut cell_diff = CellIndex::new(0, subcell_diff.y as i32, 0);
    let mut new_subcell_diff = SubCellIndex::new(0.0, 0.0, 0.0);
    (cell_diff.x, new_subcell_diff.x) = trunc_tile_offset(subcell_diff.x);
    (cell_diff.z, new_subcell_diff.z) = trunc_tile_offset(subcell_diff.z);
    (cell_diff, new_subcell_diff)
}

/// returns the integer and decimal part of the offset
pub fn trunc_tile_offset(new_tile_offset: f32) -> (i32, f32) {
    let int_tile_offset = trunc_towards_neg_inf_f(new_tile_offset);
    let new_subtiles_offset = new_tile_offset - int_tile_offset;
    assert_in_range_0_1(new_subtiles_offset);
    (int_tile_offset as i32, new_subtiles_offset)
}

pub fn assert_in_range_0_1(x: f32) -> f32 {
    if x < 0.0 || x > 1.0 {
        panic!("out of range: {}", x);
    } else {
        x
    }
}
