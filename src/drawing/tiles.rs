use crate::drawing::coords::cast::Cast;
use crate::drawing::coords::cell_pixel::{cell_to_pixel, subcell_center_to_pixel};
use crate::drawing::coords::truncate::assert_in_range_0_1;
use crate::drawing::{assets, Drawing, SubCellIndex};
use crate::gui::{FONT_SIZE, TEXT_COLOR};
use crate::input::PixelPosition;
use crate::map::{Cell, CellIndex, TileType};
use crate::Color;
use crate::{DrawingTrait, GameState};
use crate::game_state::Robot;

pub fn draw_map(drawer: &impl DrawingTrait, game_state: &GameState) {
    let drawing = game_state.get_drawing();
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
    let drawing = game_state.get_drawing();
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    let cell = game_state.map.get_cell(cell_index);
    let tile_type = cell.tile_type;

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
    // draw_pressure_number(drawer, cell_index, screen_width, drawing, max_cell, cell)
    // draw_cell_hit_box(drawer, game_state, cell_index);
    if game_state.robots.contains(&Robot {position: cell_index} ) {
        drawer.draw_transparent_texture(TileType::Robot, pixel.x, pixel.y, 1.0);
    }
}

#[allow(unused)]
fn draw_pressure_number(
    drawer: &impl DrawingTrait,
    cell_index: CellIndex,
    screen_width: f32,
    drawing: &Drawing,
    max_cell: &CellIndex,
    cell: &Cell,
) {
    if cell_index.y == max_cell.y {
        let center_pixel = subcell_center_to_pixel(
            SubCellIndex::new(
                cell_index.x as f32 + 0.25,
                cell_index.y as f32,
                cell_index.z as f32 + 0.5,
            ),
            drawing,
            screen_width,
        );
        drawer.draw_text(
            format!("{}", cell.pressure).as_str(),
            center_pixel.x,
            center_pixel.y,
            FONT_SIZE,
            TEXT_COLOR,
        )
    }
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

#[allow(dead_code)]
fn draw_cell_hit_box(drawer: &impl DrawingTrait, game_state: &GameState, cell_index: CellIndex) {
    let mut subcell: SubCellIndex = cell_index.cast();
    let size = 2.0;
    let color = Color::new(1.0, 1.0, 1.0, 1.0);
    let drawing = game_state.get_drawing();
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
#[allow(dead_code)]
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
        let offset = SubCellIndex::new(0.5, 0.0, 0.0);
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
        let cell = CellIndex::new(min_cell.x, 0, min_cell.z);
        let offset = SubCellIndex::new(0.0, 0.0, 0.0);
        let t = get_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.0);

        let offset = SubCellIndex::new(0.2, 0.0, 0.8);
        let t = get_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.2);
    }
}
