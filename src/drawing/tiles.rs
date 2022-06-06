use crate::drawing::coords::cast::Cast;
use crate::drawing::coords::cell_pixel::{cell_to_pixel, subcell_center_to_pixel};
use crate::drawing::coords::truncate::assert_in_range_0_1;
use crate::drawing::{assets, Drawing, PixelPosition, SubCellIndex};
use crate::map::CellIndex;
use crate::Color;
use crate::{DrawingTrait, GameState};
use macroquad::window::screen_width;

pub fn draw_map(drawer: &impl DrawingTrait, game_state: &GameState) {
    let drawing = drawer.drawing();
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    for i_y in min_cell.y..=max_cell.y {
        for i_z in min_cell.z..=max_cell.z {
            for i_x in min_cell.x..=max_cell.x {
                draw_cell(drawer, game_state, CellIndex::new(i_x, i_y, i_z));
            }
        }
    }
}

fn draw_cell(drawer: &impl DrawingTrait, game_state: &GameState, cell_index: CellIndex) {
    let screen_width = drawer.screen_width();
    let drawing = drawer.drawing();
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    let tile_type = game_state.map.get_cell(cell_index).tile_type;

    let pixel = cell_to_pixel(cell_index, drawing, screen_width);
    // if drawing.highlighted_cells.len() > 0 {
    //     println!("selected something");
    // }
    if drawing.highlighted_cells.contains(&cell_index) {
        let selection_color = Color::new(0.2, 0.8, 0.2, 1.0);
        drawer.draw_colored_texture(tile_type, pixel.x, pixel.y, selection_color);
    } else {
        let opacity = get_opacity(&cell_index, &min_cell, &max_cell, &drawing.subcell_diff);
        // let opacity = 1.0; // for debugging
        drawer.draw_transparent_texture(tile_type, pixel.x, pixel.y, opacity);
    }
    // draw_cell_hit_box(drawer, cell_index);
}

fn get_opacity(
    cell: &CellIndex,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
    subcell_offset: &SubCellIndex,
) -> f32 {
    assert_in_range_0_1(if cell.x == min_cell.x && cell.z == min_cell.z {
        f32::min(subcell_offset.x, subcell_offset.z)
    } else if cell.x == min_cell.x && cell.z == max_cell.z {
        f32::min(subcell_offset.x, 1.0 - subcell_offset.z)
    } else if cell.x == max_cell.x && cell.z == min_cell.z {
        f32::min(1.0 - subcell_offset.x, subcell_offset.z)
    } else if cell.x == max_cell.x && cell.z == max_cell.z {
        f32::min(1.0 - subcell_offset.x, 1.0 - subcell_offset.z)
    } else if cell.x == min_cell.x {
        subcell_offset.x
    } else if cell.x == max_cell.x {
        1.0 - subcell_offset.x
    } else if cell.z == min_cell.z {
        subcell_offset.z
    } else if cell.z == max_cell.z {
        1.0 - subcell_offset.z
    } else {
        1.0
    })
}

fn draw_cell_hit_box(drawer: &impl DrawingTrait, cell_index: CellIndex) {
    let mut subcell: SubCellIndex = cell_index.cast();
    let size = 2.0;
    let color = Color::new(1.0, 1.0, 1.0, 1.0);
    let drawing = drawer.drawing();
    let screen_width = drawer.screen_width();
    let offset = hitbox_offset();
    let me = subcell_center_to_pixel(subcell, drawing, screen_width) - offset;
    drawer.draw_rectangle(me.x, me.y, size, size, color);
    subcell.x += 0.5;
    let me = subcell_center_to_pixel(subcell, drawing, screen_width) - offset;
    drawer.draw_rectangle(me.x, me.y, size, size, color);
    subcell.x += 0.5;
    let me = subcell_center_to_pixel(subcell, drawing, screen_width) - offset;
    drawer.draw_rectangle(me.x, me.y, size, size, color);
    subcell.z += 1.0;
    let me = subcell_center_to_pixel(subcell, drawing, screen_width) - offset;
    drawer.draw_rectangle(me.x, me.y, size, size, color);
    subcell.x -= 1.0;
    let me = subcell_center_to_pixel(subcell, drawing, screen_width) - offset;
    drawer.draw_rectangle(me.x, me.y, size, size, color);
    subcell.z -= 0.5;
    let me = subcell_center_to_pixel(subcell, drawing, screen_width) - offset;
    drawer.draw_rectangle(me.x, me.y, size, size, color);
}

/// use this function before `pixel_to_subcell_center()` for a lifted rhombus hitbox
pub fn hitbox_offset() -> PixelPosition {
    PixelPosition::new(0.0, assets::PIXELS_PER_TILE_HEIGHT as f32 * 0.125)
}

/// use this function before `pixel_to_cell()` for a centered square hitbox
/// it might not work because of truncation errors
pub fn hitbox_offset_square() -> PixelPosition {
    PixelPosition::new(
        -(assets::PIXELS_PER_TILE_WIDTH as f32 * 0.25),
        -(assets::PIXELS_PER_TILE_HEIGHT as f32 * 0.125),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparency_border_no_offset() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        let mut cell = CellIndex::new(0, 0, 0);
        let no_offset = SubCellIndex::new(0.0, 0.0, 0.0);
        let t = get_opacity(&cell, &min_cell, &max_cell, &no_offset);
        assert_eq!(t, 1.0);

        cell.x = min_cell.x;
        let t = get_opacity(&cell, &min_cell, &max_cell, &no_offset);
        assert_eq!(t, 0.0);
    }

    #[test]
    fn transparency_border_offset() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        let mut cell = CellIndex::new(0, 0, 0);
        let mut offset = SubCellIndex::new(0.5, 0.0, 0.0);
        let t = get_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 1.0);

        cell.x = min_cell.x;
        let t = get_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.5);
    }

    #[test]
    fn transparency_border_corner() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        let mut cell = CellIndex::new(min_cell.x, 0, min_cell.z);
        let mut offset = SubCellIndex::new(0.0, 0.0, 0.0);
        let t = get_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.0);

        let mut offset = SubCellIndex::new(0.2, 0.0, 0.8);
        let t = get_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.2);
    }
}
