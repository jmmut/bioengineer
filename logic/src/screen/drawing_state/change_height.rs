use crate::screen::drawing_state::DrawingState;
use crate::world::map::{CellIndex, Map};

impl DrawingState {
    pub fn change_height_to(&mut self, y: i32) {
        self.maybe_change_height_rel(y - self.max_cell.y, None);
    }
    pub fn maybe_change_height_rel(&mut self, y: i32, go_to_robot: Option<CellIndex>) {
        self.change_height_rel(y);
        if let Option::Some(robot_pos) = go_to_robot {
            let level_diff = robot_pos.y - self.max_cell.y;
            self.change_height_rel(level_diff);
        }
    }

    fn change_height_rel(&mut self, y: i32) {
        let min_cell = &mut self.min_cell;
        let max_cell = &mut self.max_cell;
        max_cell.y += y;
        min_cell.y += y;
        move_inside_range(
            &mut min_cell.y,
            &mut max_cell.y,
            Map::default_min_cell().y,
            Map::default_max_cell().y,
        );
    }
}

/// returns if it moved min and max
fn move_inside_range(min: &mut i32, max: &mut i32, hard_min: i32, hard_max: i32) -> bool {
    if *min < hard_min {
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
