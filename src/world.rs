pub mod fluids;
pub mod game_state;
pub mod map;
mod networks;
pub mod robots;

use std::collections::{HashSet, VecDeque};

use fluids::FluidMode;
use fluids::Fluids;
use game_state::get_goal_air_cleaned;
use game_state::GameState;
use map::transform_cells::Transformation;
use map::CellIndex;
use map::Map;
use networks::Networks;
use robots::Robot;
use robots::{
    is_tile_actionable, move_robot_to_position, move_robot_to_tasks, reachable_positions,
};

use crate::screen::gui::gui_actions::GuiActions;

type AgeInMinutes = i64;

pub struct World {
    pub map: Map,
    pub fluids: Fluids,
    pub robots: Vec<Robot>,
    pub task_queue: VecDeque<Task>,
    pub networks: Networks,
    pub game_state: GameState,
    pub goal_state: GameGoalState,
    pub age_in_minutes: AgeInMinutes,
}

#[derive(Clone)]
pub enum Task {
    Transform(TransformationTask),
    Movement(CellIndex),
}

#[derive(Clone)]
pub struct TransformationTask {
    pub to_transform: HashSet<CellIndex>,
    pub transformation: Transformation,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameGoalState {
    Started,
    Finished(AgeInMinutes),
    PostFinished,
}

impl World {
    pub fn new() -> Self {
        let game_state = GameState::new();
        let mut map = Map::new();
        map.regenerate();
        let ship_position = map.get_ship_position();
        let mut fluids = Fluids::new(FluidMode::InStages);
        fluids.set_profile(game_state.profile);
        let robots = match ship_position {
            Option::None => vec![],
            Option::Some(position) => vec![Robot { position }],
        };
        World {
            map,
            fluids,
            robots,
            task_queue: VecDeque::new(),
            networks: Networks::new(),
            game_state,
            goal_state: GameGoalState::Started,
            age_in_minutes: 0,
        }
    }

    /// returns if the game should do another iteration
    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        self.update_with_gui_actions(&gui_actions);
        self.advance_frame();
        gui_actions.should_continue()
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        self.game_state.update_with_gui_actions(gui_actions);

        self.update_task_queue(gui_actions);

        if self.game_state.should_advance_fluids_this_frame() {
            self.fluids.advance(&mut self.map);
        }

        if gui_actions.input.regenerate_map {
            self.map.regenerate();
        }
        self.networks.update();
        self.update_goal_state(gui_actions);
    }

    fn update_task_queue(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(do_now_task) = gui_actions.do_now_task {
            let task = self.task_queue.remove(do_now_task);
            self.task_queue.push_front(task.unwrap());
        }
        if let Option::Some(cancel_task) = gui_actions.cancel_task {
            self.task_queue.remove(cancel_task);
        }

        if let Option::Some(transformation_task) = gui_actions.selected_cell_transformation.clone()
        {
            self.queue_transformation(transformation_task);
        }
        if let Option::Some(target_cell) = gui_actions.robot_movement {
            self.queue_movement(target_cell);
        }

        if self.game_state.should_advance_robots_this_frame() {
            self.advance_robots_task_queue();
        }
    }

    fn queue_transformation(&mut self, transformation_task: TransformationTask) {
        self.task_queue
            .push_back(Task::Transform(transformation_task));
    }
    fn queue_movement(&mut self, destination: CellIndex) {
        self.task_queue.push_back(Task::Movement(destination));
    }

    fn advance_robots_task_queue(&mut self) {
        if let Option::Some(task) = self.task_queue.pop_front() {
            match task {
                Task::Transform(transform) => {
                    let remaining = self.transform_cells_if_robots_can_do_so(transform);
                    if let Option::Some(remaining_task) = remaining {
                        self.task_queue.push_front(Task::Transform(remaining_task));
                    }
                    self.move_robots();
                }
                Task::Movement(movement) => {
                    let remaining = self.move_robots_to_position(&movement);
                    if let Option::Some(remaining_task) = remaining {
                        self.task_queue.push_front(Task::Movement(remaining_task));
                    }
                }
            }
        }
    }

    fn transform_cells_if_robots_can_do_so(
        &mut self,
        transform: TransformationTask,
    ) -> Option<TransformationTask> {
        let mut transform_opt = Option::Some(transform);
        for robot in &self.robots.clone() {
            if let Option::Some(transform) = transform_opt {
                transform_opt = self.transform_single_cell_if_robot_can_do_so(robot, transform);
            }
        }
        return transform_opt;
    }

    /// Returns None: task finished. Some: task partially advanced.
    fn transform_single_cell_if_robot_can_do_so(
        &mut self,
        robot: &Robot,
        mut transform: TransformationTask,
    ) -> Option<TransformationTask> {
        for reachable_pos_diff in reachable_positions() {
            let reachable_position = robot.position + reachable_pos_diff;
            let transformation_here = transform.to_transform.take(&reachable_position);
            if transformation_here.is_some() {
                if is_tile_actionable(
                    self.map.get_cell(robot.position).tile_type,
                    &robot.position,
                    &reachable_position,
                ) {
                    let cell = self.map.get_cell_mut(reachable_position);
                    transform.transformation.apply(cell);
                    self.networks.add(reachable_position, cell.tile_type);
                    if transform.to_transform.len() == 0 {
                        // no need to reinsert this
                        return Option::None;
                    }
                    // we only want to transform 1 cell, so do an early return with the remaining
                    // positions of this task
                    return Option::Some(transform);
                } else {
                    transform.to_transform.insert(reachable_position);
                }
            }
        }
        // No actions done. Task still pending.
        return Option::Some(transform);
    }

    fn move_robots(&mut self) {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_tasks(robot.position, &self.task_queue, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
            }
        }
    }

    /// returns Some: more movement needed. None: destination reached.
    fn move_robots_to_position(&mut self, movement_target: &CellIndex) -> Option<CellIndex> {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_position(robot.position, movement_target, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
                if robot.position == *movement_target {
                    return Option::None;
                }
            } else {
                return Option::None;
            }
        }
        return Option::Some(*movement_target);
    }

    fn update_goal_state(&mut self, gui_actions: &GuiActions) {
        if gui_actions.input.reset_quantities {
            self.networks.reset();
        }
        if self.goal_state == GameGoalState::Started {
            if self.networks.get_total_air_cleaned() > get_goal_air_cleaned() {
                self.goal_state = GameGoalState::Finished(self.age_in_minutes);
            }
        } else {
            self.goal_state = gui_actions.next_game_goal_state.unwrap_or(self.goal_state);
        }
    }

    fn advance_frame(&mut self) {
        self.game_state.advance_frame();
        if self.game_state.frame_index % 60 == 0 {
            self.age_in_minutes += 1;
        }
    }

    pub fn get_age_str(&self) -> String {
        format_age(self.age_in_minutes)
    }
}

pub fn format_age(age_in_minutes: i64) -> String {
    const MINUTES_PER_HOUR: i64 = 60;
    const HOURS_PER_DAY: i64 = 24;
    const DAYS_PER_YEAR: i64 = 365;
    const HOURS_PER_YEAR: i64 = HOURS_PER_DAY * DAYS_PER_YEAR;

    if age_in_minutes < MINUTES_PER_HOUR {
        format!("{}", format_time(age_in_minutes, "minute"))
    } else if age_in_minutes < MINUTES_PER_HOUR * HOURS_PER_DAY {
        format!(
            "{}, {}",
            format_time(age_in_minutes / MINUTES_PER_HOUR, "hour"),
            format_time(age_in_minutes % MINUTES_PER_HOUR, "minute")
        )
    } else if age_in_minutes < MINUTES_PER_HOUR * HOURS_PER_DAY * DAYS_PER_YEAR {
        format!(
            "{}, {}, {}",
            format_time(age_in_minutes / MINUTES_PER_HOUR / HOURS_PER_DAY, "day"),
            format_time(age_in_minutes / MINUTES_PER_HOUR % HOURS_PER_DAY, "hour"),
            format_time(age_in_minutes % MINUTES_PER_HOUR, "minute")
        )
    } else {
        format!(
            "{}, {}, {}, {}",
            format_time(age_in_minutes / MINUTES_PER_HOUR / HOURS_PER_YEAR, "year"),
            format_time(
                age_in_minutes / MINUTES_PER_HOUR / HOURS_PER_DAY % DAYS_PER_YEAR,
                "day"
            ),
            format_time(age_in_minutes / MINUTES_PER_HOUR % HOURS_PER_DAY, "hour"),
            format_time(age_in_minutes % MINUTES_PER_HOUR, "minute")
        )
    }
}

fn format_time(value: i64, unit: &str) -> String {
    format!("{} {}{}", value, unit, single_str(value))
}

fn single_str(number: i64) -> String {
    if number == 1 {
        "".to_string()
    } else {
        "s".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_age() {
        assert_eq!(format_age(0), "0 minutes");
        assert_eq!(format_age(1), "1 minute");
        assert_eq!(format_age(10), "10 minutes");
        assert_eq!(format_age((1) * 60 + 0), "1 hour, 0 minutes");
        assert_eq!(format_age((17) * 60 + 0), "17 hours, 0 minutes");
        assert_eq!(format_age((26) * 60 + 0), "1 day, 2 hours, 0 minutes");
        assert_eq!(
            format_age((24 * 3 + 17) * 60 + 0),
            "3 days, 17 hours, 0 minutes"
        );
        assert_eq!(
            format_age((365 * 24 + 3 * 24 + 1) * 60 + 0),
            "1 year, 3 days, 1 hour, 0 minutes"
        );
        assert_eq!(
            format_age((1000000 * 365 * 24 + 24 + 10) * 60 + 0),
            "1000000 years, 1 day, 10 hours, 0 minutes"
        );
    }
}
