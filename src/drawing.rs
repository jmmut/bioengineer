mod actions;
pub mod assets;
mod coords;
mod hud;
mod tiles;

use crate::drawing::actions::change_height::change_height_rel;
use crate::drawing::actions::highlight_cells::highlight_cells_from_pixels;
use crate::drawing::actions::move_horizontally::move_map_horizontally;
use crate::game_state::GameState;
use crate::input::{CellSelection, CellSelectionType, Input, PixelPosition};
use crate::map::{CellIndex, TileType};
use crate::{Color, IVec2, Texture2D, Vec2, Vec3};
use std::collections::HashSet;

pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;
const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub fn apply_input(drawer: &mut impl DrawingTrait, input: &Input) {
    let screen_width = drawer.screen_width();
    let drawing = drawer.drawing_mut();
    drawing.maybe_change_height_rel(input.change_height_rel);
    drawing.maybe_move_map_horizontally(input.move_map_horizontally, screen_width);
    drawing.maybe_select_cells(&input.cell_selection, screen_width);
}

pub fn draw(drawer: &impl DrawingTrait, game_state: &GameState) {
    drawer.clear_background(GREY);
    tiles::draw_map(drawer, game_state);
    hud::draw_fps(drawer, game_state);
    hud::draw_level(
        drawer,
        drawer.drawing().min_cell.y,
        drawer.drawing().max_cell.y,
    );
}

pub struct Drawing {
    min_cell: CellIndex,
    max_cell: CellIndex,
    subtile_offset: SubTilePosition,
    subcell_diff: SubCellIndex,
    highlighted_cells: HashSet<CellIndex>,
    highlight_start_height: i32,
}

impl Drawing {
    pub fn new() -> Self {
        Drawing {
            min_cell: CellIndex::new(-10, -1, -10),
            max_cell: CellIndex::new(9, 1, 9),
            subtile_offset: SubTilePosition::new(0.0, 0.0),
            subcell_diff: SubCellIndex::new(0.0, 0.0, 0.0),
            highlighted_cells: HashSet::new(),
            highlight_start_height: 10,
        }
    }

    fn maybe_change_height_rel(&mut self, y: i32) {
        if y != 0 {
            change_height_rel(self, y);
        }
    }

    fn maybe_move_map_horizontally(&mut self, diff: PixelPosition, _screen_width: f32) {
        if diff != PixelPosition::new(0.0, 0.0) {
            move_map_horizontally(self, diff, _screen_width);
        }
    }

    fn maybe_select_cells(&mut self, cell_selection: &CellSelection, screen_width: f32) {
        if cell_selection.state == CellSelectionType::SelectionStarted {
            self.highlight_start_height = self.max_cell.y;
        }
        if let Option::Some(selection) = &cell_selection.selection {
            highlight_cells_from_pixels(
                selection.start,
                self.highlight_start_height,
                selection.end,
                screen_width,
                self,
            );
        }
    }
}

/// Trait to be implemented by a graphics library.
/// The purpose of this class is to decouple the project from the graphics library.
/// Hopefully, if I ever need to swap the graphics library (currently macroquad), classes like
/// this one will be the only places to change.
/// I'm not sure this will actually help, but we'll see.
pub trait DrawingTrait {
    fn new(textures: Vec<Texture2D>) -> Self
    where
        Self: Sized;

    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn clear_background(&self, color: Color);
    fn draw_texture(&self, tile: TileType, x: f32, y: f32);
    fn draw_transparent_texture(&self, tile: TileType, x: f32, y: f32, opacity_coef: f32);
    fn draw_colored_texture(&self, tile: TileType, x: f32, y: f32, color_mask: Color);
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);
    fn drawing(&self) -> &Drawing;
    fn drawing_mut(&mut self) -> &mut Drawing;
}
