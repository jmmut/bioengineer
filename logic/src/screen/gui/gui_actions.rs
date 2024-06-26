use crate::scene::GameLoopState;
use crate::screen::drawing_state::SubCellIndex;
use crate::screen::main_scene_input::{CellSelection, ZoomChange};
use crate::world::map::CellIndex;
use crate::world::{GameGoalState, TransformationTask};

pub struct GuiActions {
    pub cell_selection: CellSelection,
    pub selected_cell_transformation: Option<TransformationTask>,
    pub go_to_robot: Option<CellIndex>,
    pub cancel_task: Option<usize>,
    pub next_game_goal_state: Option<GameGoalState>,
    pub regenerate_map: bool,
    pub toggle_profiling: bool,
    pub toggle_fluids: bool,
    pub single_fluid: bool,
    pub reset_quantities: bool,
    pub quit: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally_diff: SubCellIndex,
    pub zoom_change: ZoomChange,
}

impl GuiActions {
    pub fn should_continue(&self) -> GameLoopState {
        if self.quit {
            GameLoopState::ShouldFinish
        } else {
            GameLoopState::ShouldContinue
        }
    }
}

impl Default for GuiActions {
    fn default() -> Self {
        Self {
            cell_selection: CellSelection::no_selection(),
            selected_cell_transformation: None,
            go_to_robot: None,
            cancel_task: None,
            next_game_goal_state: None,
            regenerate_map: false,
            toggle_profiling: false,
            toggle_fluids: false,
            single_fluid: false,
            reset_quantities: false,
            quit: false,
            change_height_rel: 0,
            move_map_horizontally_diff: Default::default(),
            zoom_change: ZoomChange::ZoomIn,
        }
    }
}
