use super::map::Map;
use crate::drawing::Drawing;
use crate::gui::GuiActions;
use crate::map::fluids::{FluidMode, Fluids};
use crate::map::transform_cells::Transformation;
use crate::map::CellIndex;
use crate::now;
use std::collections::HashSet;

const DEFAULT_PROFILE_ENABLED: bool = false;
const DEFAULT_ADVANCING_FLUIDS: bool = false;
const DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES: i32 = 1;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub map: Map,
    pub drawing: Drawing,
    pub advancing_fluids: bool,
    pub advance_fluid_every_n_frames: i32,
    pub fluids: Fluids,
    pub profile: bool,
    pub robots: Vec<Robot>,
    pub task_queue: Vec<Task>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut map = Map::new();
        map.regenerate();
        let profile = DEFAULT_PROFILE_ENABLED;
        let mut fluids = Fluids::new(FluidMode::InStages);
        fluids.set_profile(profile);
        let ship_position = map.get_ship_position();
        let robots = match ship_position {
            Option::None => vec![],
            Option::Some(position) => vec![Robot { position }],
        };
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            map,
            drawing: Drawing::new(),
            advancing_fluids: DEFAULT_ADVANCING_FLUIDS,
            advance_fluid_every_n_frames: DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES,
            fluids,
            profile,
            robots,
            task_queue: Vec::new(),
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(transformation) = gui_actions.selected_cell_transformation {
            self.queue_transformation(transformation);
        }
        self.move_robots();
        self.transform_cells_if_robots_can_do_so();
        if gui_actions.input.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }
        if self.should_advance_fluids_this_frame(&gui_actions) {
            self.fluids.advance(&mut self.map);
        }

        if gui_actions.input.regenerate_map {
            self.map.regenerate();
        }
    }

    fn queue_transformation(&mut self, transformation: Transformation) {
        self.task_queue.push(Task {
            to_transform: self.drawing.highlighted_cells.clone(),
            transformation,
        });
    }

    fn move_robots(&mut self) {}

    fn transform_cells_if_robots_can_do_so(&mut self) {}

    fn should_advance_fluids_this_frame(&mut self, gui_actions: &GuiActions) -> bool {
        if gui_actions.input.single_fluid {
            return true;
        } else {
            if self.advancing_fluids {
                let should_process_frame =
                    self.frame_index % self.advance_fluid_every_n_frames == 0;
                return should_process_frame;
            }
        }
        return false;
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

#[derive(PartialEq)]
pub struct Robot {
    pub position: CellIndex,
}

pub struct Task {
    pub to_transform: HashSet<CellIndex>,
    pub transformation: Transformation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Cell, TileType};

    #[test]
    fn test_move_robot_basic() {
        let task = Task {
            to_transform: HashSet::from([CellIndex::new(0, 0, 10)]),
            transformation: Transformation::to(TileType::MachineAssembler),
        };
        let map = Map::_new_from_tiles(Cell::new(TileType::FloorRock), vec![]);
    }
}
