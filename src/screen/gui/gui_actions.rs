use crate::screen::drawing_state::SubCellIndex;
use crate::screen::input::CellSelection;
use crate::world::map::CellIndex;
use crate::world::{GameGoalState, TransformationTask};

pub struct GuiActions {
    // pub input: Input,
    pub cell_selection: CellSelection,
    pub selected_cell_transformation: Option<TransformationTask>,
    pub robot_movement: Option<CellIndex>,
    pub go_to_robot: Option<CellIndex>,
    pub cancel_task: Option<usize>,
    pub do_now_task: Option<usize>,
    pub next_game_goal_state: Option<GameGoalState>,
    pub regenerate_map: bool,
    pub toggle_profiling: bool,
    pub toggle_fluids: bool,
    pub single_fluid: bool,
    pub reset_quantities: bool,
    pub quit: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally_diff: SubCellIndex,
}

impl GuiActions {
    pub fn should_continue(&self) -> bool {
        !self.quit
    }
}
