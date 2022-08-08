use crate::screen::drawing_state::coords::cast::Cast;
use crate::screen::drawing_state::coords::cell_pixel::{cell_to_pixel, subcell_center_to_pixel};
use crate::screen::drawing_state::coords::truncate::assert_in_range_0_1;
use crate::screen::drawing_state::{DrawingState, SubCellIndex};
use crate::world::game_state::robots::Robot;
use crate::screen::gui::{FONT_SIZE, TEXT_COLOR};
use crate::screen::input::PixelPosition;
use crate::world::map::{Cell, CellIndex, is_covering, TileType};
use crate::Color;
use crate::GameState;
use crate::screen::assets;
use crate::screen::drawer_trait::DrawerTrait;

const REDUCED_OPACITY_TO_SEE_ROBOT: f32 = 0.5;
const SELECTION_COLOR: Color = Color::new(0.7, 0.8, 1.0, 1.0);

pub fn draw_map(drawer: &impl DrawerTrait, game_state: &GameState, drawing: &DrawingState) {
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    for i_y in min_cell.y..=max_cell.y {
        for i_z in min_cell.z..=max_cell.z {
            for i_x in min_cell.x..=max_cell.x {
                draw_cell(drawer, game_state, CellIndex::new(i_x, i_y, i_z), drawing);
            }
        }
    }
}

fn draw_cell(
    drawer: &impl DrawerTrait,
    game_state: &GameState,
    cell_index: CellIndex,
    drawing: &DrawingState,
) {
    let screen_width = drawer.screen_width();
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    let cell = game_state.map.get_cell(cell_index);
    let tile_type = cell.tile_type;

    let pixel = cell_to_pixel(cell_index, drawing, screen_width);
    // if drawing.highlighted_cells.len() > 0 {
    //     println!("selected something");
    // }
    if drawing.highlighted_cells.contains(&cell_index) {
        drawer.draw_colored_texture(tile_type, pixel.x, pixel.y, SELECTION_COLOR);
    } else {
        let opacity = get_opacity(
            &cell_index,
            tile_type,
            &game_state,
            &drawing,
            &min_cell,
            &max_cell,
        );
        // let opacity = 1.0; // for debugging
        drawer.draw_transparent_texture(tile_type, pixel.x, pixel.y, opacity);
    }
    // draw_pressure_number(drawer, cell_index, screen_width, drawing, max_cell, cell)
    // draw_cell_hit_box(drawer, game_state, cell_index);
    if game_state.robots.contains(&Robot {
        position: cell_index,
    }) {
        drawer.draw_transparent_texture(TileType::Robot, pixel.x, pixel.y, 1.0);
    }
}

fn get_opacity(
    cell_index: &CellIndex,
    tile_type: TileType,
    game_state: &GameState,
    drawing: &DrawingState,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
) -> f32 {
    let border_opacity = get_border_opacity(cell_index, min_cell, max_cell, &drawing.subcell_diff);
    let opacity_to_see_robot = get_opacity_to_see_robot(&cell_index, tile_type, &game_state.robots);
    let opacity = f32::min(border_opacity, opacity_to_see_robot);
    opacity
}

#[allow(unused)]
fn draw_pressure_number(
    drawer: &impl DrawerTrait,
    cell_index: CellIndex,
    screen_width: f32,
    drawing: &DrawingState,
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

fn get_border_opacity(
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

fn get_opacity_to_see_robot(
    cell_index: &CellIndex,
    tile_type: TileType,
    robots: &Vec<Robot>,
) -> f32 {
    if is_covering(tile_type) {
        let mut cells_with_reduced_opacity = Vec::new();
        for robot in robots {
            cells_with_reduced_opacity.push(robot.position + CellIndex::new(0, 0, 1));
            cells_with_reduced_opacity.push(robot.position + CellIndex::new(1, 0, 1));
            cells_with_reduced_opacity.push(robot.position + CellIndex::new(1, 0, 0));
        }
        if cells_with_reduced_opacity.contains(cell_index) {
            return REDUCED_OPACITY_TO_SEE_ROBOT;
        }
    }
    return 1.0;
}

#[allow(dead_code)]
fn draw_cell_hit_box(drawer: &impl DrawerTrait, cell_index: CellIndex, drawing: &DrawingState) {
    let mut subcell: SubCellIndex = cell_index.cast();
    let size = 2.0;
    let color = Color::new(1.0, 1.0, 1.0, 1.0);
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
        let t = get_border_opacity(&cell, &min_cell, &max_cell, &no_offset);
        assert_eq!(t, 1.0);

        cell.x = min_cell.x;
        let t = get_border_opacity(&cell, &min_cell, &max_cell, &no_offset);
        assert_eq!(t, 0.0);
    }

    #[test]
    fn transparency_border_offset() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        let mut cell = CellIndex::new(0, 0, 0);
        let offset = SubCellIndex::new(0.5, 0.0, 0.0);
        let t = get_border_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 1.0);

        cell.x = min_cell.x;
        let t = get_border_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.5);
    }

    #[test]
    fn transparency_border_corner() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        let cell = CellIndex::new(min_cell.x, 0, min_cell.z);
        let offset = SubCellIndex::new(0.0, 0.0, 0.0);
        let t = get_border_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.0);

        let offset = SubCellIndex::new(0.2, 0.0, 0.8);
        let t = get_border_opacity(&cell, &min_cell, &max_cell, &offset);
        assert_eq!(t, 0.2);
    }

    #[test]
    fn test_opacity_to_see_robot() {
        let full_opacity = 1.0;
        let robots = vec![Robot {
            position: CellIndex::new(2, 5, 7),
        }];
        let tile = TileType::WallRock;
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(2, 5, 7), tile, &robots);
        assert_eq!(opacity_to_see_robot, full_opacity);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(2, 5, 8), tile, &robots);
        assert_eq!(opacity_to_see_robot, REDUCED_OPACITY_TO_SEE_ROBOT);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(2, 6, 7), tile, &robots);
        assert_eq!(opacity_to_see_robot, full_opacity);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(3, 5, 7), tile, &robots);
        assert_eq!(opacity_to_see_robot, REDUCED_OPACITY_TO_SEE_ROBOT);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(3, 5, 8), tile, &robots);
        assert_eq!(opacity_to_see_robot, REDUCED_OPACITY_TO_SEE_ROBOT);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(2, 5, 9), tile, &robots);
        assert_eq!(opacity_to_see_robot, full_opacity);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(4, 7, 9), tile, &robots);
        assert_eq!(opacity_to_see_robot, full_opacity);
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(4, 7, 7), tile, &robots);
        assert_eq!(opacity_to_see_robot, full_opacity);
    }

    #[test]
    fn test_full_opacity_to_see_robot_when_non_covering() {
        let full_opacity = 1.0;
        let robots = vec![Robot {
            position: CellIndex::new(2, 5, 7),
        }];
        let opacity_to_see_robot =
            get_opacity_to_see_robot(&CellIndex::new(2, 5, 8), TileType::FloorRock, &robots);
        assert_eq!(opacity_to_see_robot, full_opacity);
    }
}
