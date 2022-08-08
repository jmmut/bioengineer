use crate::screen::drawing_state::coords::cell_pixel::pixel_to_subcell_offset;
use crate::screen::drawing_state::coords::cell_tile::subcell_to_subtile_offset;
use crate::screen::drawing_state::coords::truncate::truncate_cell_offset;
use crate::screen::drawing_state::{DrawingState, SubCellIndex};
use crate::screen::input::PixelPosition;
use crate::world::map::{CellIndex, Map};

impl DrawingState {
    pub fn maybe_move_map_horizontally(
        &mut self,
        diff: PixelPosition,
        go_to_robot: Option<CellIndex>,
        screen_width: f32,
    ) {
        if diff != PixelPosition::new(0.0, 0.0) {
            self.move_map_horizontally(diff, screen_width);
        }
        if let Option::Some(robot_pos) = go_to_robot {
            let center = CellIndex::new(
                (self.max_cell.x + self.min_cell.x) / 2,
                0,
                (self.max_cell.z + self.min_cell.z) / 2,
            );
            let cell_diff = center - robot_pos;
            self.move_map_horizontally_to(cell_diff, SubCellIndex::default());
        }
    }

    fn move_map_horizontally(&mut self, diff: PixelPosition, _screen_width: f32) {
        let subcell_diff_ = pixel_to_subcell_offset(diff);

        // let new_cell_offset = pixel_to_cell_offset(diff);

        // println!(
        //     "pixel_diff: {}, subcell_diff: {}",
        //     diff, subcell_diff_
        // );
        let (truncated_cell_diff, truncated_subcell_diff) =
            truncate_cell_offset(subcell_diff_ + self.subcell_diff);

        self.move_map_horizontally_to(truncated_cell_diff, truncated_subcell_diff);
    }

    fn move_map_horizontally_to(
        &mut self,
        truncated_cell_diff: CellIndex,
        truncated_subcell_diff: SubCellIndex,
    ) {
        self.subcell_diff = truncated_subcell_diff;

        // println!(
        //     "truncated_cell_diff: {}, truncated_subcell_diff: {}",
        //     truncated_cell_diff, truncated_subcell_diff
        // );
        let min_cell = &mut self.min_cell;
        let max_cell = &mut self.max_cell;

        max_cell.x -= truncated_cell_diff.x;
        min_cell.x -= truncated_cell_diff.x;
        max_cell.z -= truncated_cell_diff.z;
        min_cell.z -= truncated_cell_diff.z;
        if move_inside_range_min_equals(
            &mut min_cell.x,
            &mut max_cell.x,
            Map::default_min_cell().x,
            Map::default_max_cell().x,
        ) {
            self.subcell_diff.x = 0.0;
        }
        if move_inside_range_min_equals(
            &mut min_cell.z,
            &mut max_cell.z,
            Map::default_min_cell().z,
            Map::default_max_cell().z,
        ) {
            self.subcell_diff.z = 0.0;
        }

        self.subtile_offset = subcell_to_subtile_offset(self.subcell_diff);
        // {
        //     let test_cell = CellIndex::new(2, self.max_cell.y, 2);
        //     let p = cell_to_pixel(test_cell, self, screen_width);
        //     let test_cell_2 = pixel_to_cell(p, self, screen_width);
        //     println!("for test_cell {}, got cell {}", test_cell, test_cell_2);
        // }
        // println!("subtile_offset: {}\n ", self.subtile_offset);
    }
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
