use crate::game_state::{Task, TransformationTask};
use crate::map::Map;
use crate::map::{is_walkable_horizontal, is_walkable_vertical, CellIndex, TileType};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::vec::IntoIter;

#[derive(PartialEq, Copy, Clone)]
pub struct Robot {
    pub position: CellIndex,
}
type CellIndexDiff = CellIndex;

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
        let distance_1 = manhattan_distance(current_pos, *task_pos_1);
        let distance_2 = manhattan_distance(current_pos, *task_pos_2);
        distance_1.cmp(&distance_2)
    });
    cells.into_iter()
}

pub fn move_robot_to_position(
    current_pos: CellIndex,
    target_pos: &CellIndex,
    map: &Map,
) -> Option<CellIndexDiff> {
    let mut dirs = Vec::new();
    if target_pos.x > current_pos.x {
        dirs.push(CellIndexDiff::new(1, 0, 0));
    } else if target_pos.x < current_pos.x {
        dirs.push(CellIndexDiff::new(-1, 0, 0));
    }
    if target_pos.y > current_pos.y {
        dirs.push(CellIndexDiff::new(0, 1, 0));
    } else if target_pos.y < current_pos.y {
        dirs.push(CellIndexDiff::new(0, -1, 0));
    }
    if target_pos.z > current_pos.z {
        dirs.push(CellIndexDiff::new(0, 0, 1));
    } else if target_pos.z < current_pos.z {
        dirs.push(CellIndexDiff::new(0, 0, -1));
    }

    let path = try_move(&dirs, current_pos, *target_pos, map);
    let movement = path.and_then(|p| p.last().map(|c| *c));
    movement
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
            let walkable = is_position_walkable(map, &possible_new_pos, &current_pos);
            if walkable {
                let moving_to_dir_gets_us_closer =
                    manhattan_distance(target_pos, possible_new_pos) < diff;
                if moving_to_dir_gets_us_closer {
                    let path = try_move(dirs, possible_new_pos, target_pos, map);
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

/*
fn is_position_walkable(map: &Map, possible_new_pos: &CellIndex, origin: &CellIndex) -> bool {
    is_walkable_horizontal(
        map.get_cell_optional(*possible_new_pos)
            .map(|cell| cell.tile_type)
            .unwrap_or(TileType::Unset),
    )
}

 */

fn is_position_walkable(map: &Map, possible_new_pos: &CellIndex, origin: &CellIndex) -> bool {
    let target_tile = map
        .get_cell_optional(*possible_new_pos)
        .map(|cell| cell.tile_type)
        .unwrap_or(TileType::Unset);
    let direction: CellIndexDiff = *possible_new_pos - *origin;
    if *direction == *CellIndexDiff::new(0, 1, 0) || *direction == *CellIndexDiff::new(0, -1, 0) {
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

pub fn is_position_actionable(
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

pub fn is_vertical_direction(origin_pos: &CellIndex, target_pos: &CellIndex) -> bool {
    let direction: CellIndexDiff = *target_pos - *origin_pos;
    *direction == *CellIndexDiff::new(0, 1, 0) || *direction == *CellIndexDiff::new(0, -1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::GameState;
    use crate::map::transform_cells::Transformation;
    use crate::map::{Cell, TileType};
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
            assert_eq!(new_pos, Option::Some(CellIndex::new(-1, 0, 0)));
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
    }

    mod tasks {
        use super::*;

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
            let mut game_state = GameState::new();
            let initial_pos = CellIndex::new(0, 1, 0);
            let below_pos = CellIndex::new(0, 0, 0);
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([below_pos, initial_pos]),
                transformation: Transformation::to(TileType::Stairs),
            };
            game_state.task_queue = VecDeque::from([Task::Transform(transformation_task.clone())]);
            game_state.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(below_pos, TileType::FloorDirt)],
            );
            game_state.robots.first_mut().unwrap().position = initial_pos;

            assert_positions_equal(&game_state, TileType::FloorDirt, TileType::FloorDirt);

            let transformation_task =
                game_state.transform_cells_if_robots_can_do_so(transformation_task);
            game_state.move_robots();

            assert_positions_equal(&game_state, TileType::FloorDirt, TileType::Stairs);

            game_state.transform_cells_if_robots_can_do_so(transformation_task.unwrap());
            game_state.move_robots();

            assert_positions_equal(&game_state, TileType::Stairs, TileType::Stairs);
        }

        fn assert_positions_equal(game_state: &GameState, tile: TileType, second_tile: TileType) {
            let index = CellIndex::new(0, 0, 0);
            assert_eq!(game_state.map.get_cell(index).tile_type, tile);
            let other_index = CellIndex::new(0, 1, 0);
            assert_eq!(game_state.map.get_cell(other_index).tile_type, second_tile);
        }

        #[test]
        fn test_move_robot_to_nearest_task() {
            let initial_pos = CellIndex::new(0, 0, 0);
            let closest_target = CellIndex::new(-1, 0, 2);
            let farthest_target = CellIndex::new(2, 0, 2);
            let mut transformation_task = TransformationTask {
                to_transform: HashSet::from([farthest_target, closest_target]),
                transformation: Transformation::to(TileType::Stairs),
            };
            let mut iter = order_by_closest_target(&mut transformation_task, initial_pos);
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
            let mut game_state = GameState::new();
            let transformation_task = TransformationTask {
                to_transform: HashSet::from([target]),
                transformation: Transformation::to(TileType::MachineAssembler),
            };
            game_state.task_queue = VecDeque::from([Task::Transform(transformation_task.clone())]);
            game_state.map = Map::_new_from_tiles(
                Cell::new(TileType::FloorDirt),
                vec![(target, TileType::FloorDirt), (initial_pos, initial_tile)],
            );
            game_state.robots.first_mut().unwrap().position = initial_pos;
            game_state.transform_cells_if_robots_can_do_so(transformation_task);
            assert_eq!(
                game_state.map.get_cell(target).tile_type,
                expected_target_transformation
            );
        }

        #[test]
        #[ignore]
        fn do_not_do_the_closest_task_if_it_is_blocked_by_farther_tasks() {
            panic!("unimplemented");
        }
    }
}
