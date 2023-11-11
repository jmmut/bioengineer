use crate::world::map::Map;
use crate::world::map::{is_walkable_horizontal, is_walkable_vertical, CellIndex, TileType};
use crate::world::{Task, TransformationTask};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::vec::IntoIter;

#[derive(PartialEq, Copy, Clone)]
pub struct Robot {
    pub position: CellIndex,
}
pub type CellIndexDiff = CellIndex;

pub fn move_robot_to_tasks(
    current_pos: CellIndex,
    tasks: &VecDeque<Task>,
    map: &Map,
) -> Option<CellIndexDiff> {
    if tasks.is_empty() {
        return Option::None;
    }
    if let Option::Some(Task::Transform(transform_task)) = tasks.front() {
        for target in order_by_closest_target(transform_task, current_pos) {
            let movement = move_robot_to_position(current_pos, &target, map);
            if movement.is_some() {
                return movement;
            }
        }
    }
    return Option::None;
}

fn order_by_closest_target(
    task: &TransformationTask,
    current_pos: CellIndex,
) -> IntoIter<CellIndex> {
    let mut cells: Vec<CellIndex> = task.to_transform.iter().cloned().collect();
    cells.sort_by(|task_pos_1, task_pos_2| -> Ordering {
        let floor_cost_1 = different_floor_cost(&current_pos, task_pos_1);
        let distance_1 = manhattan_distance(current_pos, *task_pos_1) + floor_cost_1;
        let floor_cost_2 = different_floor_cost(&current_pos, task_pos_2);
        let distance_2 = manhattan_distance(current_pos, *task_pos_2) + floor_cost_2;
        distance_1.cmp(&distance_2)
    });
    cells.into_iter()
}

fn different_floor_cost(pos_1: &CellIndex, pos_2: &CellIndex) -> i32 {
    (pos_1.y - pos_2.y).abs() * 100
}

type SourceToOrigin = CellIndex;
type CellAndSource = (CellIndex, SourceToOrigin);

enum PathResult {
    Some(Path),
    Almost(Path),
    None,
    TooFar,
}

struct AStart {
    origin: CellIndex,
    visit_pending: VecDeque<CellAndSource>,
    already_visited: HashMap<CellIndex, SourceToOrigin>,
}
type Path = Vec<CellIndex>;

impl AStart {
    pub fn new(origin: CellIndex) -> Self {
        Self {
            origin,
            visit_pending: VecDeque::from([(origin, origin)]),
            already_visited: HashMap::new(),
        }
    }

    pub fn find_path_to(&mut self, target: &CellIndex, map: &Map) -> PathResult {
        while let Some((current, source)) = self.visit_pending.pop_front() {
            if self.already_visited.contains_key(&current) {
                continue;
            } else if self.already_visited.len() > 100000 {
                return PathResult::TooFar;
            } else if current == *target {
                self.already_visited.insert(current, source);
                return PathResult::Some(self.construct_path(current));
            } else {
                self.already_visited.insert(current, source);
                let mut walkable_adjacent = VecDeque::new();
                for diff in reachable_positions() {
                    let adjacent = current + diff;
                    if !self.already_visited.contains_key(&adjacent) {
                        let walkable = is_position_walkable(map, &current, &adjacent);
                        let actionable = is_position_actionable(map, &current, &adjacent);
                        if adjacent == *target && actionable {
                            self.already_visited.insert(adjacent, current);
                            return if walkable {
                                PathResult::Some(self.construct_path(adjacent))
                            } else {
                                PathResult::Almost(self.construct_path(adjacent))
                            };
                        } else if walkable {
                            walkable_adjacent.push_front((adjacent, current));
                        }
                    }
                }
                self.visit_pending.append(&mut walkable_adjacent);
            }
        }
        PathResult::None
    }

    fn construct_path(&self, mut pos: CellIndex) -> Path {
        let mut path = Vec::new();
        while pos != self.origin {
            path.push(pos);
            match self.already_visited.get(&pos) {
                None => panic!(
                    "the source position should exist! pos: {}, already_visited: {:?}",
                    pos, self.already_visited
                ),
                Some(source) => {
                    pos = *source;
                }
            }
        }
        path
    }
}

pub fn move_robot_to_position(
    current_pos: CellIndex,
    target_pos: &CellIndex,
    map: &Map,
) -> Option<CellIndexDiff> {
    let mut a_start = AStart::new(current_pos);
    let path = a_start.find_path_to(target_pos, map);
    match path {
        PathResult::Almost(path) | PathResult::Some(path) => {
            path.last().map(|first_step| *first_step - current_pos)
        }
        PathResult::None => None,
        PathResult::TooFar => None,
    }
}

pub fn _move_robot_to_position_old(
    current_pos: CellIndex,
    target_pos: &CellIndex,
    map: &Map,
) -> Option<CellIndexDiff> {
    let mut dirs = Vec::new();

    match target_pos.x.cmp(&current_pos.x) {
        Ordering::Greater => dirs.push(CellIndexDiff::new(1, 0, 0)),
        Ordering::Less => dirs.push(CellIndexDiff::new(-1, 0, 0)),
        Ordering::Equal => {}
    }
    match target_pos.y.cmp(&current_pos.y) {
        Ordering::Greater => dirs.push(CellIndexDiff::new(0, 1, 0)),
        Ordering::Less => dirs.push(CellIndexDiff::new(0, -1, 0)),
        Ordering::Equal => {}
    }
    match target_pos.z.cmp(&current_pos.z) {
        Ordering::Greater => dirs.push(CellIndexDiff::new(0, 0, 1)),
        Ordering::Less => dirs.push(CellIndexDiff::new(0, 0, -1)),
        Ordering::Equal => {}
    }

    let path = _try_move(&dirs, current_pos, *target_pos, map);
    let movement = path.and_then(|p| p.last().copied());
    movement
}

fn _try_move(
    directions: &[CellIndexDiff],
    current_pos: CellIndex,
    target_pos: CellIndex,
    map: &Map,
) -> Option<Vec<CellIndexDiff>> {
    if current_pos == target_pos {
        Option::Some(Vec::new())
    } else {
        let diff = manhattan_distance(target_pos, current_pos);
        for dir in directions {
            let possible_new_pos = current_pos + *dir;
            let walkable = is_position_walkable(map, &current_pos, &possible_new_pos);
            if walkable {
                let moving_to_dir_gets_us_closer =
                    manhattan_distance(target_pos, possible_new_pos) < diff;
                if moving_to_dir_gets_us_closer {
                    let path = _try_move(directions, possible_new_pos, target_pos, map);
                    if let Option::Some(mut some_path) = path {
                        some_path.push(*dir);
                        return Option::Some(some_path);
                    }
                }
            } else if diff == 1 {
                // returning an incomplete but valid path
                return Option::Some(Vec::new());
            }
        }
        Option::None
    }
}

fn is_position_walkable(map: &Map, origin: &CellIndex, possible_new_pos: &CellIndex) -> bool {
    let target_tile = map
        .get_cell_optional(*possible_new_pos)
        .map(|cell| cell.tile_type)
        .unwrap_or(TileType::Unset);
    let direction: CellIndexDiff = *possible_new_pos - *origin;
    if direction == CellIndexDiff::new(0, 1, 0) || direction == CellIndexDiff::new(0, -1, 0) {
        let origin_tile = map
            .get_cell_optional(*origin)
            .map(|cell| cell.tile_type)
            .unwrap_or(TileType::Unset);
        is_walkable_vertical(target_tile, origin_tile)
    } else {
        is_walkable_horizontal(target_tile)
    }
}

fn manhattan_distance(pos: CellIndex, other_pos: CellIndex) -> i32 {
    i32::abs(pos.x - other_pos.x) + i32::abs(pos.y - other_pos.y) + i32::abs(pos.z - other_pos.z)
}

#[allow(unused)]
fn manhattan_length(pos: &CellIndexDiff) -> i32 {
    i32::abs(pos.x) + i32::abs(pos.y) + i32::abs(pos.z)
}

pub fn reachable_positions() -> Vec<CellIndexDiff> {
    vec![
        CellIndexDiff::new(0, 0, 0),
        CellIndexDiff::new(1, 0, 0),
        CellIndexDiff::new(-1, 0, 0),
        CellIndexDiff::new(0, 1, 0),
        CellIndexDiff::new(0, -1, 0),
        CellIndexDiff::new(0, 0, 1),
        CellIndexDiff::new(0, 0, -1),
    ]
}

pub fn is_position_actionable(map: &Map, origin_pos: &CellIndex, target_pos: &CellIndex) -> bool {
    map.get_cell_optional(*origin_pos).map_or(false, |cell| {
        is_tile_actionable(cell.tile_type, origin_pos, target_pos)
    })
}

pub fn is_tile_actionable(
    origin: TileType,
    origin_pos: &CellIndex,
    target_pos: &CellIndex,
) -> bool {
    if manhattan_distance(*origin_pos, *target_pos) <= 1 {
        if is_vertical_direction(origin_pos, target_pos) {
            origin == TileType::Stairs
        } else {
            true
        }
    } else {
        false
    }
}

pub fn down() -> CellIndexDiff {
    CellIndexDiff::new(0, -1, 0)
}

pub fn up() -> CellIndexDiff {
    CellIndexDiff::new(0, 1, 0)
}

pub fn is_vertical_direction(origin_pos: &CellIndex, target_pos: &CellIndex) -> bool {
    let direction: CellIndexDiff = *target_pos - *origin_pos;
    direction == up() || direction == down()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::map::transform_cells::Transformation;
    use crate::world::map::{Cell, TileType};
    use std::collections::HashSet;

    mod movement {
        use super::*;

        #[test]
        fn test_move_robot_basic() {
            let initial_pos = CellIndex::new(10, 0, 0);
            let target_pos = CellIndex::new(10, 0, 10);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(target_pos, TileType::FloorRock)],
            );
            let new_pos = move_robot_to_position(initial_pos, &target_pos, &map);
            assert_eq!(new_pos, Option::Some(CellIndex::new(0, 0, 1)));
        }

        #[test]
        fn test_move_robot_3d() {
            let initial_pos = CellIndex::new(10, 0, 0);
            let target_pos = CellIndex::new(5, 7, 10);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::Stairs),
                vec![
                    (initial_pos, TileType::Stairs),
                    (target_pos, TileType::Stairs),
                ],
            );
            let new_pos = move_robot_to_position(initial_pos, &target_pos, &map);
            let possible_solutions = [
                Option::Some(CellIndex::new(-1, 0, 0)),
                Option::Some(CellIndex::new(0, 1, 0)),
                Option::Some(CellIndex::new(0, 0, 1)),
            ];
            assert!(
                possible_solutions.contains(&new_pos),
                "solution: {:?}, is not one of the correct ones: {:?}",
                new_pos,
                possible_solutions
            );
        }

        #[test]
        fn test_move_robot_full_path() {
            let mut initial_pos = CellIndex::new(10, 0, 0);
            let target_pos = CellIndex::new(5, 7, 10);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::Stairs),
                vec![
                    (initial_pos, TileType::Stairs),
                    (target_pos, TileType::Stairs),
                ],
            );
            let max_path_length = manhattan_distance(target_pos, initial_pos);
            for _ in 0..max_path_length {
                let dir = move_robot_to_position(initial_pos, &target_pos, &map).unwrap();
                initial_pos += dir;
            }
            assert_eq!(initial_pos, target_pos);
        }

        #[test]
        fn test_move_robot_no_movement() {
            let initial_pos = CellIndex::new(10, 0, 0);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(initial_pos, TileType::FloorRock)],
            );
            let new_pos = move_robot_to_position(initial_pos, &initial_pos, &map);
            assert_eq!(new_pos, Option::None);
        }

        #[test]
        fn test_move_robot_single_movement() {
            let initial_pos = CellIndex::new(10, 0, 0);
            let target_pos = CellIndex::new(10, 0, 1);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (initial_pos, TileType::FloorDirt),
                    (target_pos, TileType::FloorRock),
                ],
            );
            let movement = move_robot_to_position(initial_pos, &target_pos, &map);
            assert_eq!(movement, Option::Some(target_pos - initial_pos));
        }

        #[test]
        fn test_move_robot_around_obstacles() {
            let mut initial_pos = CellIndex::new(10, 0, 0);
            let target_pos = CellIndex::new(12, 0, 2);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (CellIndex::new(11, 0, 0), TileType::WallRock),
                    (CellIndex::new(11, 0, 2), TileType::WallRock),
                ],
            );
            let diff = move_robot_to_position(initial_pos, &target_pos, &map);
            assert_eq!(diff, Option::Some(CellIndex::new(0, 0, 1)));
            initial_pos += diff.unwrap();
            let diff = move_robot_to_position(initial_pos, &target_pos, &map);
            assert_eq!(diff, Option::Some(CellIndex::new(1, 0, 0)));
            initial_pos += diff.unwrap();
            let diff = move_robot_to_position(initial_pos, &target_pos, &map);
            assert_eq!(diff, Option::Some(CellIndex::new(1, 0, 0)));
            initial_pos += diff.unwrap();
            let diff = move_robot_to_position(initial_pos, &target_pos, &map);
            assert_eq!(diff, Option::Some(CellIndex::new(0, 0, 1)));
        }

        #[test]
        fn test_robot_can_not_move_through_floors() {
            let current_pos = CellIndex::new(10, 0, 0);
            let target_vertical = CellIndex::new(10, 1, 0);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (current_pos, TileType::FloorDirt),
                    (target_vertical, TileType::FloorDirt),
                ],
            );
            let moved = move_robot_to_position(current_pos, &target_vertical, &map);
            assert_eq!(moved, Option::None);
        }

        #[test]
        fn test_robot_can_not_move_through_walls() {
            let current_pos = CellIndex::new(5, 0, 0);
            let target = CellIndex::new(10, 0, 0);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::WallRock),
                vec![(current_pos, TileType::FloorDirt)],
            );
            let moved = move_robot_to_position(current_pos, &target, &map);
            assert_eq!(moved, Option::None);
        }
    }

    mod tasks {
        use super::*;
        use crate::screen::gui::GuiActions;
        use crate::screen::main_scene_input::{CellSelection, ZoomChange};
        use crate::world::World;

        #[test]
        fn test_move_robot_basic_task() {
            let initial_pos = CellIndex::new(0, 0, 0);
            let cell_index_to_transform = CellIndex::new(0, 0, 10);
            let tasks = VecDeque::from([Task::Transform(TransformationTask {
                to_transform: HashSet::from([cell_index_to_transform]),
                transformation: Transformation::to(TileType::MachineAssembler),
            })]);
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (initial_pos, TileType::FloorDirt),
                    (cell_index_to_transform, TileType::FloorRock),
                ],
            );
            let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
            assert_eq!(new_pos, Option::Some(CellIndex::new(0, 0, 1)));
        }

        #[test]
        fn test_move_robot_no_tasks() {
            let initial_pos = CellIndex::new(0, 0, 0);
            let cell_index_to_transform = CellIndex::new(0, 0, 10);
            let tasks = VecDeque::new();
            let map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (initial_pos, TileType::FloorDirt),
                    (cell_index_to_transform, TileType::FloorRock),
                ],
            );
            let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
            assert_eq!(new_pos, Option::None);
        }

        #[test]
        fn test_move_robot_temporarily_unreachable() {
            let mut world = World::new();
            let initial_pos = CellIndex::new(0, 1, 0);
            let below_pos = CellIndex::new(0, 0, 0);
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([below_pos, initial_pos]),
                transformation: Transformation::to(TileType::Stairs),
            };
            world.task_queue = VecDeque::from([Task::Transform(transformation_task.clone())]);
            world.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(below_pos, TileType::FloorDirt)],
            );
            world.robots.first_mut().unwrap().position = initial_pos;

            assert_positions_equal(&world, TileType::FloorDirt, TileType::FloorDirt);

            let transformation_task =
                world.transform_cells_if_robots_can_do_so(transformation_task);
            world.move_robots();

            assert_positions_equal(&world, TileType::FloorDirt, TileType::Stairs);

            world.transform_cells_if_robots_can_do_so(transformation_task.unwrap());
            world.move_robots();

            assert_positions_equal(&world, TileType::Stairs, TileType::Stairs);
        }

        fn assert_positions_equal(world: &World, tile: TileType, second_tile: TileType) {
            let index = CellIndex::new(0, 0, 0);
            assert_eq!(world.map.get_cell(index).tile_type, tile);
            let other_index = CellIndex::new(0, 1, 0);
            assert_eq!(world.map.get_cell(other_index).tile_type, second_tile);
        }

        #[test]
        fn test_move_robot_to_nearest_task() {
            let initial_pos = CellIndex::new(0, 0, 0);
            let closest_target = CellIndex::new(-1, 0, 2);
            let farthest_target = CellIndex::new(2, 0, 2);
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([farthest_target, closest_target]),
                transformation: Transformation::to(TileType::Stairs),
            };
            let mut iter = order_by_closest_target(&transformation_task, initial_pos);
            assert_eq!(iter.next().unwrap(), closest_target);
            assert_eq!(iter.next().unwrap(), farthest_target);
        }

        #[test]
        fn test_can_do_vertical_task_with_stairs() {
            vertical_task(TileType::Stairs, TileType::MachineAssembler);
        }

        #[test]
        fn test_can_not_do_vertical_task_without_stairs() {
            vertical_task(TileType::FloorDirt, TileType::FloorDirt);
        }

        fn vertical_task(initial_tile: TileType, expected_target_transformation: TileType) {
            let initial_pos = CellIndex::new(0, 1, 0);
            let target = CellIndex::new(0, 0, 0);
            let mut world = World::new();
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([target]),
                transformation: Transformation::to(TileType::MachineAssembler),
            };
            world.task_queue = VecDeque::from([Task::Transform(transformation_task.clone())]);
            world.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(target, TileType::FloorDirt), (initial_pos, initial_tile)],
            );
            world.robots.first_mut().unwrap().position = initial_pos;
            world.transform_cells_if_robots_can_do_so(transformation_task);
            assert_eq!(
                world.map.get_cell(target).tile_type,
                expected_target_transformation
            );
        }

        #[test]
        #[ignore]
        fn do_not_do_the_closest_task_if_it_is_blocked_by_farther_tasks() {
            panic!("unimplemented");
        }

        #[test]
        fn almost_reaching_a_task() {
            let mut world = World::new();
            let initial_pos = CellIndex::new(0, 1, 0);
            let below_pos = CellIndex::new(0, 0, 0);
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([below_pos, initial_pos]),
                transformation: Transformation::to(TileType::Stairs),
            };
            world.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(below_pos, TileType::FloorDirt)],
            );
            world.robots.first_mut().unwrap().position = CellIndex::new(1, 1, 0);

            assert_positions_equal(&world, TileType::FloorDirt, TileType::FloorDirt);

            let gui_actions = mock_gui_action(Some(transformation_task));
            world.update_with_gui_actions(&gui_actions);

            assert_positions_equal(&world, TileType::FloorDirt, TileType::Stairs);

            let gui_actions = mock_gui_action(None);
            world.update_with_gui_actions(&gui_actions);

            assert_positions_equal(&world, TileType::Stairs, TileType::Stairs);
        }

        fn mock_gui_action(task: Option<TransformationTask>) -> GuiActions {
            GuiActions {
                selected_cell_transformation: task,
                quit: false,
                regenerate_map: false,
                toggle_profiling: false,
                toggle_fluids: false,
                single_fluid: false,
                change_height_rel: 0,
                move_map_horizontally_diff: Default::default(),
                cell_selection: CellSelection::no_selection(),
                reset_quantities: false,
                go_to_robot: None,
                cancel_task: None,
                do_now_task: None,
                next_game_goal_state: None,
                zoom_change: ZoomChange::None,
            }
        }

        #[test]
        fn c_path() {
            let mut world = World::new();
            let initial_pos = CellIndex::new(1, 1, 0);
            let below_pos = CellIndex::new(0, 0, 0);
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([below_pos]),
                transformation: Transformation::to(TileType::MachineAssembler),
            };
            let stairs_pos = CellIndex::new(2, 1, 0);
            world.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (stairs_pos, TileType::Stairs),
                    (CellIndex::new(2, 0, 0), TileType::Stairs),
                ],
            );
            world.robots.first_mut().unwrap().position = initial_pos;

            let gui_actions = mock_gui_action(Some(transformation_task));
            world.update_with_gui_actions(&gui_actions);

            assert_eq!(world.robots.first_mut().unwrap().position, stairs_pos);
        }

        #[test]
        fn fork_path() {
            let mut world = World::new();
            let initial_pos = CellIndex::new(1, 1, 0);
            let stairs_pos = CellIndex::new(0, 1, 0);
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([initial_pos + down(), initial_pos + up()]),
                transformation: Transformation::to(TileType::MachineAssembler),
            };
            world.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![
                    (stairs_pos + down(), TileType::Stairs),
                    (stairs_pos, TileType::Stairs),
                    (stairs_pos + up(), TileType::Stairs),
                ],
            );
            world.robots.first_mut().unwrap().position = initial_pos;

            let gui_actions = mock_gui_action(Some(transformation_task));
            world.update_with_gui_actions(&gui_actions);

            assert_eq!(world.robots.first_mut().unwrap().position, stairs_pos);
        }
    }
}
