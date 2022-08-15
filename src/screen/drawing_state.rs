pub mod change_height;
pub mod highlight_cells;
pub mod move_horizontally;

use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::hud;
use crate::screen::gui::{draw_map, GuiActions};
use crate::world::game_state::GameState;
use crate::world::map::CellIndex;
use crate::{Color, IVec2, Vec2, Vec3};
use std::collections::HashSet;

pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;
const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub fn draw(drawer: &impl DrawerTrait, game_state: &GameState, drawing: &DrawingState) {
    drawer.clear_background(GREY);
    draw_map::draw_map(drawer, game_state, drawing);
    hud::draw_fps(drawer, game_state);
    hud::draw_level(drawer, drawing.min_cell.y, drawing.max_cell.y);
    hud::draw_networks(drawer, game_state);
}

pub struct DrawingState {
    pub min_cell: CellIndex,
    pub max_cell: CellIndex,
    pub subtile_offset: SubTilePosition,
    pub subcell_diff: SubCellIndex,
    pub highlighted_cells: HashSet<CellIndex>,
    highlight_start_height: i32,
}

impl DrawingState {
    pub fn new() -> Self {
        DrawingState {
            min_cell: CellIndex::new(-10, -1, -10),
            max_cell: CellIndex::new(9, 1, 9),
            subtile_offset: SubTilePosition::new(0.0, 0.0),
            subcell_diff: SubCellIndex::new(0.0, 0.0, 0.0),
            highlighted_cells: HashSet::new(),
            highlight_start_height: 0,
        }
    }

    pub fn apply_input(&mut self, unhandled: &GuiActions, screen_width: f32) {
        let input = &unhandled.input;
        self.maybe_change_height_rel(input.change_height_rel, unhandled.go_to_robot);
        self.maybe_move_map_horizontally(
            input.move_map_horizontally,
            unhandled.go_to_robot,
            screen_width,
        );
        self.maybe_select_cells(&input.cell_selection, screen_width);
    }
}
