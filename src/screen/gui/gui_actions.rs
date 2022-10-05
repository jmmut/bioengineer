use crate::screen::input::Input;
use crate::world::map::CellIndex;
use crate::world::{GameGoalState, TransformationTask};

pub struct GuiActions {
    pub input: Input,
    pub selected_cell_transformation: Option<TransformationTask>,
    pub robot_movement: Option<CellIndex>,
    pub go_to_robot: Option<CellIndex>,
    pub cancel_task: Option<usize>,
    pub do_now_task: Option<usize>,
    pub next_game_goal_state: Option<GameGoalState>,
}

impl GuiActions {
    pub fn should_continue(&self) -> bool {
        !self.input.quit
    }
}
