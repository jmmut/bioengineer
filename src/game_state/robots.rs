
use crate::map::Map;
use crate::map::{is_walkable, CellIndex, TileType};
use crate::game_state::Task;
use std::collections::VecDeque;

#[derive(PartialEq)]
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
    for target in tasks.front().unwrap().to_transform.iter() {
        let movement = move_robot_to_position(current_pos, target, map);
        if movement.is_some() {
            return movement;
        }
    }
    return Option::None;
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
        if diff == 1 {
            Option::Some(vec![target_pos - current_pos])
        } else {
            for dir in dirs {
                let possible_new_pos = current_pos + *dir;
                let moving_to_dir_gets_us_closer = manhattan_distance(target_pos, possible_new_pos)
                    < diff
                    && is_position_walkable(map, possible_new_pos);
                if moving_to_dir_gets_us_closer {
                    let path = try_move(dirs, possible_new_pos, target_pos, map);
                    if let Option::Some(mut some_path) = path {
                        some_path.push(*dir);
                        return Option::Some(some_path);
                    }
                }
            }
            Option::None
        }
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
    use crate::map::transform_cells::Transformation;
    use crate::game_state::GameState;
    use std::collections::HashSet;

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
        let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
        assert_eq!(new_pos, Option::Some(CellIndex::new(0, 0, 1)));
    }

    #[test]
    fn test_move_robot_3d() {
        let cell_index_to_transform = CellIndex::new(5, 7, 10);
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
        let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
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
        for _ in 0..max_path_length {
            let dir = move_robot_to_tasks(initial_pos, &tasks, &map).unwrap();
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
        let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
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
        let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
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
        let new_pos = move_robot_to_tasks(initial_pos, &tasks, &map);
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
        let diff = move_robot_to_tasks(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(0, 0, 1)));
        initial_pos += diff.unwrap();
        let diff = move_robot_to_tasks(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(1, 0, 0)));
        initial_pos += diff.unwrap();
        let diff = move_robot_to_tasks(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(1, 0, 0)));
        initial_pos += diff.unwrap();
        let diff = move_robot_to_tasks(initial_pos, &tasks, &map);
        assert_eq!(diff, Option::Some(CellIndex::new(0, 0, 1)));
    }

    #[test]
    fn test_move_robot_temporarily_unreachable() {
        let mut game_state = GameState::new();
        game_state.task_queue = VecDeque::from([Task {
            to_transform: HashSet::from([CellIndex::new(0, 0, 0), CellIndex::new(0, 1, 0)]),
            transformation: Transformation::to(TileType::Stairs),
        }]);
        let initial_pos = CellIndex::new(0, 1, 0);
        game_state.map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![(CellIndex::new(0, 0, 0), TileType::FloorDirt)],
        );
        game_state.robots.first_mut().unwrap().position = initial_pos;

        {
            let assert_cell_is_ = assert_cell_is(&game_state);
            assert_cell_is_(CellIndex::new(0, 0, 0), TileType::FloorDirt);
            assert_cell_is_(CellIndex::new(0, 1, 0), TileType::FloorDirt);
        }
        game_state.transform_cells_if_robots_can_do_so();
        game_state.move_robots();
        {
            let assert_cell_is_ = assert_cell_is(&game_state);
            assert_cell_is_(CellIndex::new(0, 0, 0), TileType::FloorDirt);
            assert_cell_is_(CellIndex::new(0, 1, 0), TileType::Stairs);
        }
        game_state.transform_cells_if_robots_can_do_so();
        game_state.move_robots();
        {
            let assert_cell_is_ = assert_cell_is(&game_state);
            assert_cell_is_(CellIndex::new(0, 0, 0), TileType::Stairs);
            assert_cell_is_(CellIndex::new(0, 1, 0), TileType::Stairs);
        }
    }

    fn assert_cell_is<'a>(game_state: &'a GameState) -> impl Fn(CellIndex, TileType) + 'a {
        |cell_index: CellIndex, tile_type: TileType| {
            assert_eq!(game_state.map.get_cell(cell_index).tile_type, tile_type);
        }
    }
}
