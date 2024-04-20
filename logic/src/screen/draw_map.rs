use crate::screen::assets;
use crate::screen::coords::cast::Cast;
use crate::screen::coords::cell_pixel::{cell_to_pixel, subcell_center_to_pixel};
use crate::screen::coords::truncate::assert_in_range_0_1;
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::{DrawingState, SubCellIndex, SubTilePosition};
use crate::screen::gui::{FONT_SIZE, TEXT_COLOR};
use crate::screen::main_scene_input::PixelPosition;
use crate::world::fluids::VERTICAL_PRESSURE_DIFFERENCE;
use crate::world::map::cell::{ExtraTextures, TextureIndexTrait};
use crate::world::map::{Cell, CellIndex, Pressure, TileType};
use crate::world::World;
use mq_basics::Color;
use crate::screen::coords::tile_pixel::subtile_to_pixel_offset;

const SELECTION_COLOR: Color = Color::new(0.7, 0.8, 1.0, 1.0);

pub fn draw_map(drawer: &dyn DrawerTrait, world: &World, drawing: &DrawingState) {
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    let fog = Color::new(0.5, 0.5, 0.5, 0.7);
    for i_y in min_cell.y..=max_cell.y {
        // draw fog on lower levels, without affecting the top level textures
        drawer.draw_rectangle(0.0, 0.0,  drawer.screen_width(), drawer.screen_height(), fog);
        for i_z in min_cell.z..=max_cell.z {
            for i_x in min_cell.x..=max_cell.x {
                draw_cell(drawer, world, CellIndex::new(i_x, i_y, i_z), drawing);
            }
        }
    }
}

fn draw_cell(
    drawer: &dyn DrawerTrait,
    world: &World,
    cell_index: CellIndex,
    drawing: &DrawingState,
) {
    let screen_width = drawer.screen_width();
    let min_cell = &drawing.min_cell;
    let max_cell = &drawing.max_cell;
    let cell = world.map.get_cell(cell_index);
    let tile_type = cell.tile_type;
    let texture = choose_texture(cell, &tile_type);
    let depth = max_cell.y - cell_index.y;

    let mut pixel = cell_to_pixel(cell_index, drawing, screen_width);
    let level_offset = subtile_to_pixel_offset(SubTilePosition::new(1.0/64.0, -0.5), drawing.zoom);
    pixel += level_offset * depth as f32;

    // if drawing.highlighted_cells.len() > 0 {
    //     println!("selected something");
    // }
    if drawing.highlighted_cells().contains(&cell_index) {
        drawer.draw_colored_texture(texture, pixel.x, pixel.y, drawing.zoom, SELECTION_COLOR);
    } else {
        let opacity = get_opacity(&cell_index, drawing, min_cell, max_cell, tile_type, cell.pressure);
        // let opacity = 1.0; // for debugging

        // let depth = max_cell.y - cell_index.y;
        // let depth = 0;
        let fog = 1.0 - 0.2*depth as f32;
        let color = Color::new(fog, fog, fog, opacity);
        drawer.draw_colored_texture(texture, pixel.x, pixel.y, drawing.zoom, color);
    }
    // draw_pressure_number(drawer, cell_index, screen_width, drawing, max_cell, cell)
    // draw_cell_hit_box(drawer, game_state, cell_index);
}

fn choose_texture<'a>(cell: &'a Cell, tile_type: &'a TileType) -> &'a dyn TextureIndexTrait {
    if cell.renderable_pressure <= 0 {
        tile_type
    } else if cell.renderable_pressure <= VERTICAL_PRESSURE_DIFFERENCE {
        &ExtraTextures::DirtyWaterSurface
    } else {
        &ExtraTextures::DirtyWaterWall
    }
}

fn get_opacity(
    cell_index: &CellIndex,
    drawing: &DrawingState,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
    tile_type: TileType,
    pressure: Pressure,
) -> f32 {
    if
    // cell_index.y == max_cell.y  &&
    tile_type == TileType::Air
        && pressure == 0
    {
        0.0
    } else {
        get_border_opacity(cell_index, min_cell, max_cell, &drawing.subcell_diff)
    }
}

#[allow(unused)]
fn draw_pressure_number(
    drawer: &dyn DrawerTrait,
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

#[allow(dead_code)]
fn draw_cell_hit_box(drawer: &dyn DrawerTrait, cell_index: CellIndex, drawing: &DrawingState) {
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
    PixelPosition::new(0.0, assets::PIXELS_PER_TILE_HEIGHT as f32 * -0.125)
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
}
