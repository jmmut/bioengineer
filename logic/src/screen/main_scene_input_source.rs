use crate::screen::main_scene_input::{
    CellSelectionType, Input, PixelCellSelection, PixelSelection, ZoomChange,
};
use juquad::input::input_trait::InputTrait;
use juquad::PixelPosition;
use mq_basics::{KeyCode, MouseButton, Vec2};

pub struct MainSceneInputSource {
    input_source: Box<dyn InputTrait>,
    previous_wheel_click_pos: Vec2,
    previous_left_click_pos: Option<PixelPosition>,
    previous_right_click_pos_with_control: Option<PixelPosition>,
}

impl MainSceneInputSource {
    pub fn new(input_source: Box<dyn InputTrait>) -> Self {
        MainSceneInputSource {
            input_source,
            previous_wheel_click_pos: Vec2::new(0.0, 0.0),
            previous_left_click_pos: Option::None,
            previous_right_click_pos_with_control: Option::None,
        }
    }
    pub fn get_input(&mut self) -> Input {
        Input {
            quit: self.input_source.is_key_pressed(KeyCode::Escape),
            regenerate_map: self.input_source.is_key_pressed(KeyCode::M),
            reload_ui_skin: self.input_source.is_key_pressed(KeyCode::U),
            toggle_profiling: self.input_source.is_key_pressed(KeyCode::P),
            toggle_fluids: self.input_source.is_key_pressed(KeyCode::L),
            single_fluid: self.input_source.is_key_pressed(KeyCode::N),
            change_height_rel: self.get_changed_height(),
            move_map_horizontally: self.get_horizontal_move(),
            cell_selection: self.get_cell_selection(),
            robot_movement: self.get_robot_movement(),
            reset_quantities: self.input_source.is_key_pressed(KeyCode::R),
            zoom_change: self.get_zoom(),
            go_to_ship: self.input_source.is_key_down(KeyCode::G),
        }
    }
}

impl MainSceneInputSource {
    fn get_horizontal_move(&mut self) -> PixelPosition {
        let mut diff = PixelPosition::new(0.0, 0.0);
        let control_down = self.is_control_down();
        if !control_down
            && self
                .input_source
                .is_mouse_button_pressed(MouseButton::Right)
        {
            self.previous_wheel_click_pos = self.input_source.mouse_position()
        } else if !control_down && self.input_source.is_mouse_button_down(MouseButton::Right) {
            let current_pos = self.input_source.mouse_position();
            diff.x = self.previous_wheel_click_pos.x - current_pos.x;
            diff.y = self.previous_wheel_click_pos.y - current_pos.y;

            self.previous_wheel_click_pos = current_pos;
        } else {
            let speed_x = 10.0;
            let speed_y = 5.0;
            if self.input_source.is_key_down(KeyCode::Q) {
                diff.x += -speed_x;
                diff.y += -speed_y;
            }
            if self.input_source.is_key_down(KeyCode::D) {
                diff.x += speed_x;
                diff.y += speed_y;
            }
            if self.input_source.is_key_down(KeyCode::A) {
                diff.x += -speed_x;
                diff.y += speed_y;
            }
            if self.input_source.is_key_down(KeyCode::E) {
                diff.x += speed_x;
                diff.y += -speed_y;
            }
            if self.is_shift_down() {
                diff *= 2.0;
            }
        }
        diff
    }

    fn get_left_click_position(&mut self) -> Option<PixelPosition> {
        if self.input_source.is_mouse_button_pressed(MouseButton::Left) {
            let position = self.input_source.mouse_position();
            let position = PixelPosition::new(position.x, position.y);
            self.previous_left_click_pos = Option::Some(position);
            Option::Some(position)
        } else {
            Option::None
        }
    }

    fn get_right_click_position_with_control(&mut self) -> Option<PixelPosition> {
        if self.is_control_down() {
            let pos = self.get_right_click_position();
            if self.previous_right_click_pos_with_control.is_none() {
                self.previous_right_click_pos_with_control = pos;
            }
            pos
        } else {
            None
        }
    }

    fn get_right_click_position(&mut self) -> Option<PixelPosition> {
        if self
            .input_source
            .is_mouse_button_pressed(MouseButton::Right)
        {
            let position = self.input_source.mouse_position();
            let position = PixelPosition::new(position.x, position.y);
            Option::Some(position)
        } else {
            Option::None
        }
    }
    fn get_left_click_release(&mut self) -> Option<PixelSelection> {
        if self
            .input_source
            .is_mouse_button_released(MouseButton::Left)
            && self.previous_left_click_pos.is_some()
        {
            let start = self.previous_left_click_pos.unwrap();
            self.previous_left_click_pos = Option::None;
            let position = self.input_source.mouse_position();
            Option::Some(PixelSelection {
                start,
                end: PixelPosition::new(position.x, position.y),
            })
        } else {
            Option::None
        }
    }

    fn get_right_click_release_with_control(&mut self) -> Option<PixelSelection> {
        if self.is_control_down() {
            let selection = self.get_right_click_release();
            if selection.is_some() {
                self.previous_right_click_pos_with_control = None;
            }
            selection
        } else {
            None
        }
    }
    fn get_right_click_release(&mut self) -> Option<PixelSelection> {
        if self
            .input_source
            .is_mouse_button_released(MouseButton::Right)
            && self.previous_right_click_pos_with_control.is_some()
        {
            let start = self.previous_right_click_pos_with_control.unwrap();
            let position = self.input_source.mouse_position();
            Option::Some(PixelSelection {
                start,
                end: position,
            })
        } else {
            Option::None
        }
    }

    fn get_changed_height(&mut self) -> i32 {
        let total_diff = self.get_changed_height_with_wheel() + self.get_vertical_arrow_pressed();
        let mut change_height_rel = if total_diff > 0 {
            1
        } else if total_diff < 0 {
            -1
        } else {
            0
        };
        if self.is_shift_down() {
            change_height_rel *= 3;
        }
        change_height_rel
    }

    fn get_changed_height_with_wheel(&mut self) -> i32 {
        return if self.is_control_down() || self.is_super_down() {
            // this is zoom, not height change
            0
        } else {
            self.get_mouse_wheel_diff()
        };
    }

    fn get_mouse_wheel_diff(&mut self) -> i32 {
        let mouse = self.input_source.mouse_wheel();
        if mouse.x != 0.0 || mouse.y != 0.0 {
            // TODO how can I log in the js console_log? try info!()
            // eprintln!("mouse wheel: {}, {}", mouse_x, mouse_y);
        }
        let scroll = if mouse.y > 0.0 {
            1
        } else if mouse.y < 0.0 {
            -1
        } else {
            0
        };
        scroll
    }

    fn get_vertical_arrow_pressed(&mut self) -> i32 {
        (if self.input_source.is_key_pressed(KeyCode::Up)
            || self.input_source.is_key_pressed(KeyCode::W)
        {
            1
        } else {
            0
        }) + if self.input_source.is_key_pressed(KeyCode::Down)
            || self.input_source.is_key_pressed(KeyCode::S)
        {
            -1
        } else {
            0
        }
    }

    fn get_cell_selection_type(&self) -> CellSelectionType {
        let modifier = self.is_control_down();
        if modifier {
            if self.input_source.is_mouse_button_pressed(MouseButton::Left)
                || self.input_source.is_mouse_button_down(MouseButton::Left)
                || self
                    .input_source
                    .is_mouse_button_released(MouseButton::Left)
            {
                CellSelectionType::Add
            } else if self
                .input_source
                .is_mouse_button_pressed(MouseButton::Right)
                || self.input_source.is_mouse_button_down(MouseButton::Right)
                || self
                    .input_source
                    .is_mouse_button_released(MouseButton::Right)
            {
                CellSelectionType::Remove
            } else {
                CellSelectionType::Exclusive
            }
        } else {
            CellSelectionType::Exclusive
        }
    }

    fn is_control_down(&self) -> bool {
        self.input_source.is_key_down(KeyCode::LeftControl)
            || self.input_source.is_key_down(KeyCode::RightControl)
    }
    fn is_super_down(&self) -> bool {
        self.input_source.is_key_down(KeyCode::LeftSuper)
            || self.input_source.is_key_down(KeyCode::RightSuper)
    }

    fn is_shift_down(&self) -> bool {
        self.input_source.is_key_down(KeyCode::LeftShift)
            || self.input_source.is_key_down(KeyCode::RightShift)
    }

    fn get_cell_selection(&mut self) -> PixelCellSelection {
        let start_selection_this_frame = self
            .get_left_click_position()
            .or(self.get_right_click_position_with_control());
        let end_selection = self
            .get_left_click_release()
            .or(self.get_right_click_release_with_control());
        let mouse_position = self.input_source.mouse_position();
        let addition = self.get_cell_selection_type();
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
        if self.input_source.is_key_down(KeyCode::LeftControl)
            || self.input_source.is_key_down(KeyCode::RightControl)
        {
            None
        } else {
            let right_click_pos = self.get_right_click_position();
            right_click_pos
        }
    }

    fn get_zoom(&mut self) -> ZoomChange {
        if self.is_control_down() || self.is_super_down() {
            let wheel_diff = self.get_mouse_wheel_diff();
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
