use crate::map::{CellCubeIterator, CellIndex, Map};

fn advance_fluid(map: &mut Map) {
    let min_cell = map.min_cell();
    let max_cell = map.max_cell();
    let iter = CellCubeIterator::new(min_cell, max_cell);
    let is_valid = |cell_index: CellIndex| {
        cell_index.x >= min_cell.x
            && cell_index.x <= max_cell.x
            && cell_index.y >= min_cell.y
            && cell_index.y <= max_cell.y
            && cell_index.z >= min_cell.z
            && cell_index.z <= max_cell.z
    };
    let xp = CellIndex::new(1, 0, 0);
    let xn = CellIndex::new(-1, 0, 0);
    let zp = CellIndex::new(0, 0, 1);
    let zn = CellIndex::new(0, 0, -1);
    for cell_index in iter {
        let xp_valid = is_valid(cell_index + xp);
        let xn_valid = is_valid(cell_index + xn);
        let zp_valid = is_valid(cell_index + zp);
        let zn_valid = is_valid(cell_index + zn);
        if xp_valid && xn_valid && zp_valid && zn_valid {
            let current_pressure = map.get_cell(cell_index).pressure;
            let mut flow = Vec::new();
            let mut add_flow_direction = |dir| {
                if map.get_cell(cell_index + dir).pressure < current_pressure {
                    flow.push(dir);
                }
            };
            add_flow_direction(xp);
            add_flow_direction(xn);
            add_flow_direction(zp);
            add_flow_direction(zn);
            if current_pressure > flow.len() as i32 {
                map.get_cell_mut(cell_index).next_pressure -= flow.len() as i32;
                for dir in flow {
                    map.get_cell_mut(cell_index + dir).next_pressure += 1;
                }
            }
        }
    }
    let iter = CellCubeIterator::new(min_cell, max_cell);
    for cell_index in iter {
        let cell = map.get_cell_mut(cell_index);
        cell.pressure += cell.next_pressure;
        cell.next_pressure = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Map;

    fn assert_steps_2x2(maps: Vec<Vec<i32>>) {
        assert!(maps.len() >= 2);
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(2, 0, 2);
        for i in 1..maps.len() {
            let initial = maps[i - 1].clone();
            let expected = maps[i].clone();
            let mut map = Map::new_from_pressures(initial, min_cell, max_cell);
            advance_fluid(&mut map);
            let computed = map.get_pressures(min_cell, max_cell);
            assert_eq!(computed, expected);
        }
    }
    #[test]
    fn test_basic_fluid() {
        #[rustfmt::skip]
        let cells = vec![
            0, 0, 0,
            0, 5, 0,
            0, 0, 0,
        ];
        #[rustfmt::skip]
        let expected = vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0,
        ];
        assert_steps_2x2(vec![cells, expected]);
    }

    #[test]
    fn test_no_movement() {
        #[rustfmt::skip]
        let cells = vec![
            0, 0, 0,
            0, 3, 0,
            0, 0, 0,
        ];
        #[rustfmt::skip]
        let expected = vec![
            0, 0, 0,
            0, 3, 0,
            0, 0, 0,
        ];
        assert_steps_2x2(vec![cells, expected]);
    }

    #[test]
    fn test_2_steps() {
        #[rustfmt::skip]
        let cells = vec![
            0, 0, 0,
            0, 10, 0,
            0, 0, 0,
        ];
        #[rustfmt::skip]
        let expected_1 = vec![
            0, 1, 0,
            1, 6, 1,
            0, 1, 0,
        ];
        #[rustfmt::skip]
        let expected_2 = vec![
            0, 2, 0,
            2, 2, 2,
            0, 2, 0,
        ];
        assert_steps_2x2(vec![cells, expected_1, expected_2]);
    }
}
