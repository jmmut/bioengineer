use super::map::Map;
use crate::drawing::Drawing;
use crate::gui::GuiActions;
use crate::map::fluids::{FluidMode, Fluids};
use crate::map::transform_cells::Transformation;
use crate::map::{CellIndex, TileType, is_walkable};
use crate::now;
use std::collections::{HashSet, VecDeque};

const DEFAULT_PROFILE_ENABLED: bool = false;
const DEFAULT_ADVANCING_FLUIDS: bool = false;
const DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES: i32 = 1;
const DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES: i32 = 15;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub map: Map,
    pub drawing: Drawing,
    pub advancing_fluids: bool,
    pub advance_fluid_every_n_frames: i32,
    pub advance_robots_every_n_frames: i32,
    pub fluids: Fluids,
    pub profile: bool,
    pub robots: Vec<Robot>,
    pub task_queue: VecDeque<Task>,
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
            advance_robots_every_n_frames: DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES,
            fluids,
            profile,
            robots,
            task_queue: VecDeque::new(),
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(transformation) = gui_actions.selected_cell_transformation {
            self.queue_transformation(transformation);
        }
        if self.should_advance_robots_this_frame() {
            self.move_robots();
            self.transform_cells_if_robots_can_do_so();
        }
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
        self.task_queue.push_back(Task {
            to_transform: self.drawing.highlighted_cells.clone(),
            transformation,
        });
    }

    fn move_robots(&mut self) {
        for robot in &mut self.robots {
            let movement_opt = move_robot(robot.position, &self.task_queue, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
            }
        }
    }

    fn transform_cells_if_robots_can_do_so(&mut self) {
        for robot in &self.robots {
            let task_opt = self.task_queue.front_mut();
            if let Option::Some(task) = task_opt {
                let transformation_here = task.to_transform.take(&robot.position);
                if transformation_here.is_some() {
                    task.transformation.apply(self.map.get_cell_mut(robot.position));
                    if task.to_transform.len() == 0 {
                        self.task_queue.pop_front();
                    }
                }
            }
        }
    }

    fn should_advance_robots_this_frame(&mut self) -> bool {
        let should_process_frame = self.frame_index % self.advance_robots_every_n_frames == 0;
        should_process_frame
    }

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
type CellIndexDiff = CellIndex;

fn move_robot(current_pos: CellIndex, tasks: &VecDeque<Task>, map: &Map) -> Option<CellIndexDiff> {
    if tasks.is_empty() {
        return Option::None;
    }
    let target = *tasks.front().unwrap().to_transform.iter().next().unwrap();
    let mut dirs = Vec::new();
    if target.x > current_pos.x {
        dirs.push(CellIndexDiff::new(1, 0, 0));
    } else if target.x < current_pos.x {
        dirs.push(CellIndexDiff::new(-1, 0, 0));
    }
    if target.y > current_pos.y {
        dirs.push(CellIndexDiff::new(0, 1, 0));
    } else if target.y < current_pos.y {
        dirs.push(CellIndexDiff::new(0, -1, 0));
    }
    if target.z > current_pos.z {
        dirs.push(CellIndexDiff::new(0, 0, 1));
    } else if target.z < current_pos.z {
        dirs.push(CellIndexDiff::new(0, 0, -1));
    }

    let path = try_move(&dirs, current_pos, target, map);
    path.and_then(|p| p.last().map(|c| *c))
}

fn try_move(
    dirs: &Vec<CellIndexDiff>,
    current_pos: CellIndex,
    target_pos: CellIndex,
    map: &Map,
) -> Option<Vec<CellIndexDiff>> {
    if current_pos == target_pos {
        Option::Some(Vec::new())
    } else {
        let diff = manhattan_distance(target_pos, current_pos);
        for dir in dirs {
            let possible_new_pos = current_pos + *dir;
            let moving_to_dir_gets_us_closer = manhattan_distance(target_pos, possible_new_pos)
                < diff
                && is_position_walkable(map, possible_new_pos);
            if moving_to_dir_gets_us_closer {
                let mut path = try_move(dirs, possible_new_pos, target_pos, map);
                if let Option::Some(mut some_path) = path {
                    some_path.push(*dir);
                    return Option::Some(some_path);
                }
            }
        }
        Option::None
    }
}

fn is_position_walkable(map: &Map, possible_new_pos: CellIndex) -> bool {
    is_walkable(
        map.get_cell_optional(possible_new_pos)
            .map(|cell| cell.tile_type)
            .unwrap_or(TileType::Unset),
    )
}

fn manhattan_distance(pos: CellIndex, other_pos: CellIndex) -> i32 {
    i32::abs(pos.x - other_pos.x) + i32::abs(pos.y - other_pos.y) + i32::abs(pos.z - other_pos.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Cell, TileType};

    #[test]
    fn test_move_robot_basic() {
        let cell_index_to_transform = CellIndex::new(0, 0, 10);
        let tasks = VecDeque::from([Task {
            to_transform: HashSet::from([cell_index_to_transform]),
            transformation: Transformation::to(TileType::MachineAssembler),
        }]);
        let initial_pos = CellIndex::new(0, 0, 0);
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![
                (initial_pos, TileType::FloorDirt),
                (cell_index_to_transform, TileType::FloorRock),
            ],
        );
        let new_pos = move_robot(initial_pos, &tasks, &map);
        assert_eq!(new_pos, Option::Some(CellIndex::new(0, 0, 1)));
    }

    #[test]
    fn test_move_robot_3d() {
        let cell_index_to_transform = CellIndex::new(5, 7, 10);
        let tasks =  VecDeque::from([Task {
            to_transform: HashSet::from([cell_index_to_transform]),
            transformation: Transformation::to(TileType::MachineAssembler),
        }]);
        let initial_pos = CellIndex::new(0, 0, 0);
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![
                (initial_pos, TileType::FloorDirt),
                (cell_index_to_transform, TileType::FloorRock),
            ],
        );
        let new_pos = move_robot(initial_pos, &tasks, &map);
        assert_eq!(new_pos, Option::Some(CellIndex::new(1, 0, 0)));
    }

    #[test]
    fn test_move_robot_full_path() {
        let cell_index_to_transform = CellIndex::new(5, 7, 10);
        let tasks = VecDeque::from([Task {
            to_transform: HashSet::from([cell_index_to_transform]),
            transformation: Transformation::to(TileType::MachineAssembler),
        }]);
        let mut initial_pos = CellIndex::new(0, 0, 0);
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![
                (initial_pos, TileType::FloorDirt),
                (cell_index_to_transform, TileType::FloorRock),
            ],
        );
        let max_path_length = manhattan_distance(cell_index_to_transform, initial_pos);
        for i in 0..max_path_length {
            let dir = move_robot(initial_pos, &tasks, &map).unwrap();
            initial_pos += dir;
        }
        assert_eq!(initial_pos, cell_index_to_transform);
    }

    #[test]
    fn test_move_robot_no_tasks() {
        let cell_index_to_transform = CellIndex::new(0, 0, 10);
        let tasks = VecDeque::new();
        let initial_pos = CellIndex::new(0, 0, 0);
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![
                (initial_pos, TileType::FloorDirt),
                (cell_index_to_transform, TileType::FloorRock),
            ],
        );
        let new_pos = move_robot(initial_pos, &tasks, &map);
        assert_eq!(new_pos, Option::None);
    }

    #[test]
    fn test_move_robot_no_movement() {
        let initial_pos = CellIndex::new(0, 0, 0);
        let cell_index_to_transform = initial_pos;
        let tasks = VecDeque::new();
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![(cell_index_to_transform, TileType::FloorRock)],
        );
        let new_pos = move_robot(initial_pos, &tasks, &map);
        assert_eq!(new_pos, Option::None);
    }

    #[test]
    fn test_move_robot_single_movement() {
        let cell_index_to_transform = CellIndex::new(0, 0, 1);
        let tasks = VecDeque::from([Task {
            to_transform: HashSet::from([cell_index_to_transform]),
            transformation: Transformation::to(TileType::MachineAssembler),
        }]);
        let initial_pos = CellIndex::new(0, 0, 0);
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![
                (initial_pos, TileType::FloorDirt),
                (cell_index_to_transform, TileType::FloorRock),
            ],
        );
        let new_pos = move_robot(initial_pos, &tasks, &map);
        assert_eq!(new_pos, Option::Some(cell_index_to_transform));
    }

    #[test]
    fn test_move_robot_around_obstacles() {
        let cell_index_to_transform = CellIndex::new(2, 0, 2);
        let tasks = VecDeque::from([Task {
            to_transform: HashSet::from([cell_index_to_transform]),
            transformation: Transformation::to(TileType::MachineAssembler),
        }]);
        let mut initial_pos = CellIndex::new(0, 0, 0);
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![
                (CellIndex::new(1, 0, 0), TileType::WallRock),
                (CellIndex::new(1, 0, 2), TileType::WallRock),
            ],
        );
        let diff = move_robot(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(0, 0, 1)));
        initial_pos += diff.unwrap();
        let diff = move_robot(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(1, 0, 0)));
        initial_pos += diff.unwrap();
        let diff = move_robot(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(1, 0, 0)));
        initial_pos += diff.unwrap();
        let diff = move_robot(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(0, 0, 1)));
    }
}
