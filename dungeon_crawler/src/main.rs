// Basic running game. Full game not developed

mod map;
mod player;
mod map_builder;
use prelude::*;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*; 
    pub use crate::player::*; 
    pub use crate::map_builder::*;
}


fn main() -> BError {

    let context = BTermBuilder::simple80x50()
                    .with_title("Dungeon Crawler")
                    .with_fps_cap(30.0).build()?;

    // function from the BRACKET-LIB prelude                
    main_loop(context, State::new())

}



struct State {
    map: Map, // imported in the Prelude Mod above
    player: Player,
}


impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}


// GameState is a Trait in Bracket-Lib, you must Implement function TICK()
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
