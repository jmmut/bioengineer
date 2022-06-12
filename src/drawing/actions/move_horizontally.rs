use crate::drawing::coords::cell_pixel::pixel_to_subcell_offset;
use crate::drawing::coords::cell_tile::subcell_to_subtile_offset;
use crate::drawing::coords::truncate::truncate_cell_offset;
use crate::drawing::{Drawing, SubCellIndex};
use crate::input::PixelPosition;
use crate::map::{CellIndex, Map};

pub fn move_map_horizontally(drawing: &mut Drawing, diff: PixelPosition, _screen_width: f32) {
    let subcell_diff_ = pixel_to_subcell_offset(diff);

    // let new_cell_offset = pixel_to_cell_offset(diff);

    // println!(
    //     "pixel_diff: {}, subcell_diff: {}",
    //     diff, subcell_diff_
    // );
    let (truncated_cell_diff, truncated_subcell_diff) =
        truncate_cell_offset(subcell_diff_ + drawing.subcell_diff);

    move_map_horizontally_to(drawing, truncated_cell_diff, truncated_subcell_diff);
}

fn move_map_horizontally_to(
    drawing_: &mut Drawing,
    truncated_cell_diff: CellIndex,
    truncated_subcell_diff: SubCellIndex,
) {
    drawing_.subcell_diff = truncated_subcell_diff;

    // println!(
    //     "truncated_cell_diff: {}, truncated_subcell_diff: {}",
    //     truncated_cell_diff, truncated_subcell_diff
    // );
    let min_cell = &mut drawing_.min_cell;
    let max_cell = &mut drawing_.max_cell;

    max_cell.x -= truncated_cell_diff.x;
    min_cell.x -= truncated_cell_diff.x;
    max_cell.z -= truncated_cell_diff.z;
    min_cell.z -= truncated_cell_diff.z;
    if move_inside_range_min_equals(
        &mut min_cell.x,
        &mut max_cell.x,
        Map::min_cell().x,
        Map::max_cell().x,
    ) {
        drawing_.subcell_diff.x = 0.0;
    }
    if move_inside_range_min_equals(
        &mut min_cell.z,
        &mut max_cell.z,
        Map::min_cell().z,
        Map::max_cell().z,
    ) {
        drawing_.subcell_diff.z = 0.0;
    }

    drawing_.subtile_offset = subcell_to_subtile_offset(drawing_.subcell_diff);
    // {
    //     let test_cell = CellIndex::new(2, drawing_.max_cell.y, 2);
    //     let p = cell_to_pixel(test_cell, drawing_, screen_width);
    //     let test_cell_2 = pixel_to_cell(p, drawing_, screen_width);
    //     println!("for test_cell {}, got cell {}", test_cell, test_cell_2);
    // }
    // println!("subtile_offset: {}\n ", drawing_.subtile_offset);
}

/// returns if it moved min and max
fn move_inside_range_min_equals(
    min: &mut i32,
    max: &mut i32,
    hard_min: i32,
    hard_max: i32,
) -> bool {
    if *min <= hard_min {
        let diff = hard_min - *min;
        *min += diff;
        *max += diff;
        true
    } else if *max > hard_max {
        let diff = hard_max - *max;
        *min += diff;
        *max += diff;
        true
    } else {
        false
    }
}
