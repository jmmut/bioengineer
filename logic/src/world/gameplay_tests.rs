#[cfg(test)]
mod game_goal_state_transition_tests {
    use crate::world::game_state::get_goal_air_cleaned;
    use crate::world::map::cell::DEFAULT_HEALTH;
    use crate::world::map::{Cell, CellIndex, TileType};
    use crate::world::networks::Networks;
    use crate::world::{age_tile, transition_goal_state, GameGoalState};

    #[test]
    fn test_starting() {
        let networks = Networks::new_default();
        let mut current_goal = GameGoalState::Started;

        transition_goal_state(&mut current_goal, &networks, 0, 100);

        assert_eq!(current_goal, GameGoalState::Started);
    }

    #[test]
    fn test_producing() {
        let mut networks = Networks::new_default();
        networks.set_production(get_goal_air_cleaned() / 2.0);
        let mut current_goal = GameGoalState::Started;

        transition_goal_state(&mut current_goal, &networks, 0, 100);

        assert_eq!(current_goal, GameGoalState::Started);
    }

    #[test]
    fn test_reaching_production() {
        let mut networks = Networks::new_default();
        networks.set_production(get_goal_air_cleaned());
        let mut current_goal = GameGoalState::Started;

        transition_goal_state(&mut current_goal, &networks, 0, 100);

        assert_eq!(current_goal, GameGoalState::ReachedProduction);
    }

    #[test]
    fn test_dismantling() {
        let mut networks = Networks::new_default();
        networks.add(
            CellIndex::default(),
            TileType::MachineAirCleaner,
            TileType::Air,
        );
        networks.set_production(get_goal_air_cleaned());
        let mut current_goal = GameGoalState::ReachedProduction;

        transition_goal_state(&mut current_goal, &networks, 0, 100);

        assert_eq!(current_goal, GameGoalState::ReachedProduction);
    }

    #[test]
    fn test_finishing() {
        let mut networks = Networks::new_default();
        networks.set_production(get_goal_air_cleaned());
        let mut current_goal = GameGoalState::ReachedProduction;

        transition_goal_state(&mut current_goal, &networks, 50, 100);

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

#[cfg(test)]
mod building_tests {
    use crate::screen::gui::GuiActions;
    use crate::world::map::cell::DEFAULT_HEALTH;
    use crate::world::map::transform_cells::Transformation;
    use crate::world::map::{CellIndex, TileType};
    use crate::world::{GameGoalState, TransformationTask, World};
    use std::collections::HashSet;

    fn gui_action_transform_tile(cell: CellIndex, to_tile: TileType) -> GuiActions {
        gui_action_transform_tiles([cell].into_iter(), to_tile)
    }
    fn gui_action_transform_tiles(
        cell: impl Iterator<Item = CellIndex>,
        to_tile: TileType,
    ) -> GuiActions {
        let mut gui_actions = GuiActions::default();
        gui_actions.selected_cell_transformation = Some(TransformationTask::new(
            HashSet::from_iter(cell.into_iter()),
            Transformation::to(to_tile),
        ));
        gui_actions
    }

    #[test]
    fn test_build_machine_next_to_ship() {
        let mut world = World::new();
        let cell = world.map.get_ship_position().unwrap() + CellIndex::new(0, 0, 1);
        let from_tile = TileType::Air;
        let to_tile = TileType::MachineAirCleaner;
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tile(cell, to_tile);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, to_tile);
    }

    #[test]
    fn test_build_tree_next_to_ship() {
        let mut world = World::new();
        let cell = world.map.get_ship_position().unwrap() + CellIndex::new(0, 0, 1);
        let from_tile = TileType::Air;
        let to_tile = TileType::TreeHealthy;
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tile(cell, to_tile);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, to_tile);
    }

    #[test]
    fn test_remove_machine() {
        let mut world = World::new();
        world.game_state.set_advance_every_frame();
        let cell = world.map.get_ship_position().unwrap() + CellIndex::new(0, 0, 1);
        let from_tile = TileType::Air;
        let to_tile = TileType::MachineAirCleaner;
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tile(cell, to_tile);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, to_tile);

        let to_tile = TileType::TreeHealthy;
        let gui_actions = gui_action_transform_tile(cell, to_tile);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, to_tile);
    }

    #[test]
    fn test_replace_ship_is_forbidden() {
        let mut world = World::new();
        world.game_state.set_advance_every_frame();
        let cell = world.map.get_ship_position().unwrap();
        let from_tile = TileType::MachineShip;
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tile(cell, TileType::MachineAirCleaner);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tile(cell, TileType::TreeHealthy);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);
    }

    #[test]
    fn test_replace_ship_is_forbidden_even_selecting_many() {
        let mut world = World::new();
        world.game_state.set_advance_every_frame();
        let cell = world.map.get_ship_position().unwrap();
        let adjacent_cell = cell + CellIndex::new(0, 0, 1);
        let from_tile = TileType::MachineShip;
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tiles(
            [cell, adjacent_cell].into_iter(),
            TileType::MachineAirCleaner,
        );
        world.update(gui_actions);
        world.update(GuiActions::default());
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);
    }

    #[test]
    fn test_trees_degrade() {
        let mut world = World::new();
        world.goal_state = GameGoalState::Started;
        world.game_state.set_advance_every_frame();
        let cell = world.map.get_ship_position().unwrap() + CellIndex::new(0, 0, 1);
        let from_tile = TileType::Air;
        let to_tile = TileType::TreeHealthy;
        assert_eq!(world.map.get_cell(cell).tile_type, from_tile);

        let gui_actions = gui_action_transform_tile(cell, to_tile);
        world.update(gui_actions);
        assert_eq!(world.map.get_cell(cell).tile_type, to_tile);
        for _ in 0..DEFAULT_HEALTH {
            world.update(GuiActions::default());
        }
        assert_eq!(world.map.get_cell(cell).tile_type, TileType::TreeSparse);
        for _ in 0..DEFAULT_HEALTH {
            world.update(GuiActions::default());
        }
        assert_eq!(world.map.get_cell(cell).tile_type, TileType::TreeDying);
        for _ in 0..DEFAULT_HEALTH {
            world.update(GuiActions::default());
        }
        assert_eq!(world.map.get_cell(cell).tile_type, TileType::TreeDead);
        for _ in 0..DEFAULT_HEALTH {
            world.update(GuiActions::default());
        }
        assert_eq!(world.map.get_cell(cell).tile_type, TileType::TreeDead);
    }
}
