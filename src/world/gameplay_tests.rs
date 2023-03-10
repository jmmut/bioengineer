#[cfg(test)]
mod game_goal_state_transition_tests {
    use crate::world::game_state::get_goal_air_cleaned;
    use crate::world::map::{Cell, CellIndex, TileType};
    use crate::world::networks::Networks;
    use crate::world::{transition_goal_state, GameGoalState, age_tile};
    use crate::world::map::cell::DEFAULT_HEALTH;

    #[test]
    fn test_starting() {
        let networks = Networks::new();
        let mut current_goal = GameGoalState::Started;

        transition_goal_state(&mut current_goal, &networks, 100);

        assert_eq!(current_goal, GameGoalState::Started);
    }

    #[test]
    fn test_producing() {
        let mut networks = Networks::new();
        networks.set_production(get_goal_air_cleaned() / 2.0);
        let mut current_goal = GameGoalState::Started;

        transition_goal_state(&mut current_goal, &networks, 100);

        assert_eq!(current_goal, GameGoalState::Started);
    }

    #[test]
    fn test_reaching_production() {
        let mut networks = Networks::new();
        networks.set_production(get_goal_air_cleaned());
        let mut current_goal = GameGoalState::Started;

        transition_goal_state(&mut current_goal, &networks, 100);

        assert_eq!(current_goal, GameGoalState::ReachedProduction);
    }

    #[test]
    fn test_dismantling() {
        let mut networks = Networks::new();
        networks.add(CellIndex::default(), TileType::MachineAirCleaner);
        networks.set_production(get_goal_air_cleaned());
        let mut current_goal = GameGoalState::ReachedProduction;

        transition_goal_state(&mut current_goal, &networks, 100);

        assert_eq!(current_goal, GameGoalState::ReachedProduction);
    }

    #[test]
    fn test_finishing() {
        let mut networks = Networks::new();
        networks.set_production(get_goal_air_cleaned());
        let mut current_goal = GameGoalState::ReachedProduction;

        transition_goal_state(&mut current_goal, &networks, 100);

        assert_eq!(current_goal, GameGoalState::Finished(100));
    }

    #[test]
    fn test_trees_decay() {
        let mut cell = Cell::new(TileType::TreeHealthy);
        cell.health = DEFAULT_HEALTH;
        age_tile(&mut cell, GameGoalState::Started);
        assert_eq!(cell.health, DEFAULT_HEALTH - 1);
    }

    #[test]
    fn test_trees_transition() {
        let mut cell = Cell::new(TileType::TreeHealthy);
        cell.health = 0;
        age_tile(&mut cell, GameGoalState::Started);
        assert_eq!(cell.health, DEFAULT_HEALTH);
        assert_eq!(cell.tile_type, TileType::TreeSparse);
    }

    #[test]
    fn test_trees_do_not_decay_when_air_is_clean() {
        let mut cell = Cell::new(TileType::TreeHealthy);
        cell.health = DEFAULT_HEALTH;
        age_tile(&mut cell, GameGoalState::ReachedProduction);
        assert_eq!(cell.health, DEFAULT_HEALTH);
    }
}
