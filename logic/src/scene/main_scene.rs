use crate::common::profiling::ScopedProfiler;
use crate::scene::{Scene, State};
use crate::screen::Screen;
use crate::world::World;

pub struct MainScene {
    pub world: World,
    pub screen: Screen,
}

impl Scene for MainScene {
    fn frame(&mut self) -> State {
        let _profiler =
            ScopedProfiler::new_named(self.world.game_state.profile, "whole toplevel frame");
        let gui_actions = self.screen.get_gui_actions(&self.world);
        let should_continue = self.world.update(gui_actions);
        self.screen.draw(&self.world);
        should_continue
    }
}
