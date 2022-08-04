use crate::drawing::Drawing;
use crate::world::map::Map;

pub fn change_height_rel(drawing: &mut Drawing, y: i32) {
    if y != 0 {
        let min_cell = &mut drawing.min_cell;
        let max_cell = &mut drawing.max_cell;
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
