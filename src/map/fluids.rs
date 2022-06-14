use crate::map::{CellCubeIterator, CellIndex};

fn advance_fluid(initial: Vec<i32>) -> Vec<i32> {
    let side_size = 3;
    let min_cell = CellIndex::new(0, 0, 0);
    let max_cell = CellIndex::new(side_size -1, 0, side_size -1);
    let iter = CellCubeIterator::new(min_cell, max_cell);
    let is_valid = |cell_index: CellIndex| {
        cell_index.x >= min_cell.x && cell_index.x <= max_cell.x &&
        cell_index.y >= min_cell.y && cell_index.y <= max_cell.y &&
        cell_index.z >= min_cell.z && cell_index.z <= max_cell.z
    };
    let get_index = |cell_index: CellIndex| {
        (cell_index.x * side_size + cell_index.z) as usize
    };
    let mut future = initial.clone();
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
            let current_pressure = initial[get_index(cell_index)];
            let mut flow = Vec::new();
            let mut add_flow_direction = |dir| {
                if initial[get_index(cell_index + dir)] < current_pressure {
                    flow.push(dir);
                }
            };
            add_flow_direction(xp);
            add_flow_direction(xn);
            add_flow_direction(zp);
            add_flow_direction(zn);
            if current_pressure > flow.len() as i32 {
                future[get_index(cell_index)] -= flow.len() as i32;
                for dir in flow {
                    future[get_index(cell_index + dir)] += 1;
                }
            }
        }
    }
    future
}


#[cfg(test)]
mod tests {
    use super::*;

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
        let computed = advance_fluid(cells);
        assert_eq!(computed, expected);
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
        let computed = advance_fluid(cells);
        assert_eq!(computed, expected);
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
        let computed = advance_fluid(cells);
        assert_eq!(computed, expected_1);

        #[rustfmt::skip]
        let expected_2 = vec![
            0, 2, 0,
            2, 2, 2,
            0, 2, 0,
        ];
        let computed = advance_fluid(computed);
        assert_eq!(computed, expected_2);
    }

}
