use crate::game_state::TransformationTask;
use crate::input::Input;
use crate::world::map::transform_cells::Transformation;
use crate::world::map::CellIndex;

pub struct GuiActions {
    pub input: Input,
    pub selected_cell_transformation: Option<TransformationTask>,
    pub robot_movement: Option<CellIndex>,
    pub go_to_robot: Option<CellIndex>,
    pub cancel_task: Option<usize>,
    pub do_now_task: Option<usize>,
}

impl GuiActions {
    pub fn should_continue(&self) -> bool {
        !self.input.quit
    }
}
