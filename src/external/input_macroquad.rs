use crate::screen::input::{CellSelection, Input, InputSourceTrait, PixelPosition, PixelSelection};

use macroquad::input::{
    is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
    mouse_position, mouse_wheel, KeyCode, MouseButton,
};

pub struct InputMacroquad {
    previous_wheel_click_pos: (f32, f32),
    previous_left_click_pos: Option<PixelPosition>,
}

impl InputMacroquad {
    pub fn new() -> Self {
        InputMacroquad {
            previous_wheel_click_pos: (0.0, 0.0),
            previous_left_click_pos: Option::None,
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

    fn get_changed_height(&mut self) -> i32 {
        let total_diff = self.get_mouse_wheel_height_diff() + self.get_vertical_arrow_pressed();
        if total_diff > 0 {
            1
        } else if total_diff < 0 {
            -1
        } else {
            0
        }
    }

    fn get_mouse_wheel_height_diff(&mut self) -> i32 {
        let (mouse_x, mouse_y) = mouse_wheel();
        if mouse_x != 0.0 || mouse_y != 0.0 {
            // TODO how can I log in the js console_log?
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

    fn get_cell_selection(&mut self) -> CellSelection {
        let start_selection_this_frame = self.get_left_click_position();
        let end_selection = self.get_left_click_release();
        let (mouse_position_x, mouse_position_y) = mouse_position();
        let mouse_position = PixelPosition::new(mouse_position_x, mouse_position_y);
        match end_selection {
            None => match start_selection_this_frame {
                None => match self.previous_left_click_pos {
                    None => CellSelection::no_selection(),
                    Some(start) => CellSelection::in_progress(PixelSelection {
                        start,
                        end: mouse_position,
                    }),
                },
                Some(start) => CellSelection::started(PixelSelection {
                    start,
                    end: mouse_position,
                }),
            },
            Some(end) => CellSelection::finished(end),
        }
    }

    fn get_robot_movement(&mut self) -> Option<PixelPosition> {
        let right_click_pos = self.get_right_click_position();
        right_click_pos
    }
}

impl InputSourceTrait for InputMacroquad {
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
        }
    }
}
