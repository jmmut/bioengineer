use logic::screen::main_scene_input::{
    CellSelectionType, Input, MainSceneInputTrait, PixelCellSelection, PixelPosition,
    PixelSelection, ZoomChange,
};

use macroquad::input::{
    is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
    is_mouse_button_released, mouse_position, mouse_wheel, KeyCode, MouseButton,
};

pub struct InputMacroquad {
    previous_wheel_click_pos: (f32, f32),
    previous_left_click_pos: Option<PixelPosition>,
    previous_right_click_pos_with_control: Option<PixelPosition>,
}

impl MainSceneInputTrait for InputMacroquad {
    fn get_input(&mut self) -> Input {
        Input {
            quit: is_key_pressed(KeyCode::Escape),
            regenerate_map: is_key_pressed(KeyCode::M),
            toggle_profiling: is_key_pressed(KeyCode::P),
            toggle_fluids: is_key_pressed(KeyCode::Space),
            single_fluid: is_key_pressed(KeyCode::N),
            change_height_rel: self.get_changed_height(),
            move_map_horizontally: self.get_horizontal_move(),
            cell_selection: self.get_cell_selection(),
            robot_movement: self.get_robot_movement(),
            reset_quantities: is_key_pressed(KeyCode::R),
            zoom_change: self.get_zoom(),
        }
    }
}

impl InputMacroquad {
    pub fn new() -> Self {
        InputMacroquad {
            previous_wheel_click_pos: (0.0, 0.0),
            previous_left_click_pos: Option::None,
            previous_right_click_pos_with_control: Option::None,
        }
    }

    pub fn get_horizontal_move(&mut self) -> PixelPosition {
        let mut diff = PixelPosition::new(0.0, 0.0);
        if is_mouse_button_pressed(MouseButton::Middle) {
            self.previous_wheel_click_pos = mouse_position()
        } else if is_mouse_button_down(MouseButton::Middle) {
            let current_pos = mouse_position();
            diff.x = current_pos.0 - self.previous_wheel_click_pos.0;
            diff.y = current_pos.1 - self.previous_wheel_click_pos.1;

            self.previous_wheel_click_pos = current_pos;
        }
        diff
    }

    pub fn get_left_click_position(&mut self) -> Option<PixelPosition> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (position_x, position_y) = mouse_position();
            let position = PixelPosition::new(position_x, position_y);
            self.previous_left_click_pos = Option::Some(position);
            Option::Some(position)
        } else {
            Option::None
        }
    }

    pub fn get_right_click_position_with_control(&mut self) -> Option<PixelPosition> {
        if Self::is_control_down() {
            let pos = self.get_right_click_position();
            if self.previous_right_click_pos_with_control.is_none() {
                self.previous_right_click_pos_with_control = pos;
            }
            pos
        } else {
            None
        }
    }

    pub fn get_right_click_position(&mut self) -> Option<PixelPosition> {
        if is_mouse_button_pressed(MouseButton::Right) {
            let (position_x, position_y) = mouse_position();
            let position = PixelPosition::new(position_x, position_y);
            Option::Some(position)
        } else {
            Option::None
        }
    }
    pub fn get_left_click_release(&mut self) -> Option<PixelSelection> {
        if is_mouse_button_released(MouseButton::Left) && self.previous_left_click_pos.is_some() {
            let start = self.previous_left_click_pos.unwrap();
            self.previous_left_click_pos = Option::None;
            let (position_x, position_y) = mouse_position();
            Option::Some(PixelSelection {
                start,
                end: PixelPosition::new(position_x, position_y),
            })
        } else {
            Option::None
        }
    }

    pub fn get_right_click_release_with_control(&mut self) -> Option<PixelSelection> {
        if Self::is_control_down() {
            let selection = self.get_right_click_release();
            if selection.is_some() {
                self.previous_right_click_pos_with_control = None;
            }
            selection
        } else {
            None
        }
    }
    pub fn get_right_click_release(&mut self) -> Option<PixelSelection> {
        if is_mouse_button_released(MouseButton::Right)
            && self.previous_right_click_pos_with_control.is_some()
        {
            let start = self.previous_right_click_pos_with_control.unwrap();
            let (position_x, position_y) = mouse_position();
            Option::Some(PixelSelection {
                start,
                end: PixelPosition::new(position_x, position_y),
            })
        } else {
            Option::None
        }
    }

    fn get_changed_height(&mut self) -> i32 {
        let total_diff = self.get_changed_height_with_wheel() + self.get_vertical_arrow_pressed();
        if total_diff > 0 {
            1
        } else if total_diff < 0 {
            -1
        } else {
            0
        }
    }

    fn get_changed_height_with_wheel(&mut self) -> i32 {
        return if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            // this is zoom, not height change
            0
        } else {
            self.get_mouse_wheel_height_diff()
        };
    }

    fn get_mouse_wheel_height_diff(&mut self) -> i32 {
        let (mouse_x, mouse_y) = mouse_wheel();
        if mouse_x != 0.0 || mouse_y != 0.0 {
            // TODO how can I log in the js console_log? try info!()
            // eprintln!("mouse wheel: {}, {}", mouse_x, mouse_y);
        }
        let change_height_rel = if mouse_y > 0.0 {
            1
        } else if mouse_y < 0.0 {
            -1
        } else {
            0
        };
        change_height_rel
    }

    fn get_vertical_arrow_pressed(&mut self) -> i32 {
        (if is_key_pressed(KeyCode::Up) { 1 } else { 0 }
            + if is_key_pressed(KeyCode::Down) { -1 } else { 0 })
    }

    fn get_cell_selection_type() -> CellSelectionType {
        let modifier = Self::is_control_down();
        if modifier {
            if is_mouse_button_pressed(MouseButton::Left)
                || is_mouse_button_down(MouseButton::Left)
                || is_mouse_button_released(MouseButton::Left)
            {
                CellSelectionType::Add
            } else if is_mouse_button_pressed(MouseButton::Right)
                || is_mouse_button_down(MouseButton::Right)
                || is_mouse_button_released(MouseButton::Right)
            {
                CellSelectionType::Remove
            } else {
                CellSelectionType::Exclusive
            }
        } else {
            CellSelectionType::Exclusive
        }
    }

    fn is_control_down() -> bool {
        is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
    }

    fn get_cell_selection(&mut self) -> PixelCellSelection {
        let start_selection_this_frame = self
            .get_left_click_position()
            .or(self.get_right_click_position_with_control());
        let end_selection = self
            .get_left_click_release()
            .or(self.get_right_click_release_with_control());
        let (mouse_position_x, mouse_position_y) = mouse_position();
        let mouse_position = PixelPosition::new(mouse_position_x, mouse_position_y);
        let addition = Self::get_cell_selection_type();
        let click_start = self
            .previous_left_click_pos
            .or(self.previous_right_click_pos_with_control);
        match end_selection {
            None => match start_selection_this_frame {
                None => match click_start {
                    None => PixelCellSelection::no_selection(),
                    Some(start) => PixelCellSelection::in_progress(
                        PixelSelection {
                            start,
                            end: mouse_position,
                        },
                        addition,
                    ),
                },
                Some(start) => PixelCellSelection::started(
                    PixelSelection {
                        start,
                        end: mouse_position,
                    },
                    addition,
                ),
            },
            Some(end) => PixelCellSelection::finished(end, addition),
        }
    }

    fn get_robot_movement(&mut self) -> Option<PixelPosition> {
        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            None
        } else {
            let right_click_pos = self.get_right_click_position();
            right_click_pos
        }
    }

    fn get_zoom(&mut self) -> ZoomChange {
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            let wheel_diff = self.get_mouse_wheel_height_diff();
            let change = match wheel_diff {
                1 => ZoomChange::ZoomIn,
                -1 => ZoomChange::ZoomOut,
                0 => ZoomChange::None,
                _ => panic!(
                    "expected get_mouse_wheel_height_diff to return 0, 1 or -1, returned {}",
                    wheel_diff
                ),
            };
            change
        } else {
            ZoomChange::None
        }
    }
}
