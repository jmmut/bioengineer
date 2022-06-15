use super::map::Map;
use crate::drawing::Drawing;
use crate::gui::GuiActions;
use crate::map::transform_cells::Transformation;
use crate::map::CellIndex;
use crate::now;
use std::collections::HashSet;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub map: Map,
    pub drawing: Drawing,
    pub advancing_fluids: bool,
}

impl GameState {
    pub fn new() -> GameState {
        let mut map = Map::new();
        map.regenerate();
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            map,
            drawing: Drawing::new(),
            advancing_fluids: false,
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(transformation) = gui_actions.selected_cell_transformation {
            transform_cells(
                &self.drawing.highlighted_cells,
                transformation,
                &mut self.map,
            );
        }
        if gui_actions.input.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }
        if self.advancing_fluids || gui_actions.input.single_fluid {
            self.map.advance_fluids();
        }

        if gui_actions.input.regenerate_map {
            self.map.regenerate();
        }
    }

    pub fn advance_frame(&mut self) {
        self.frame_index = (self.frame_index + 1) % 1000;
        self.previous_frame_ts = self.current_frame_ts;
        self.current_frame_ts = now();
    }
    pub fn get_drawing(&self) -> &Drawing {
        &self.drawing
    }
    pub fn get_drawing_mut(&mut self) -> &mut Drawing {
        &mut self.drawing
    }
}

fn transform_cells(
    to_transform: &HashSet<CellIndex>,
    transformation: Transformation,
    map: &mut Map,
) {
    for highlighted_cell in to_transform {
        transformation.apply(map.get_cell_mut(*highlighted_cell));
    }
}
