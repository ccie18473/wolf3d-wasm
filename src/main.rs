pub mod engine;
pub mod game;
pub mod graphics;

extern crate good_web_game as ggez;
extern crate serde_json;

mod prelude {
    pub use crate::engine::*;
    pub use crate::game::*;
    pub use crate::graphics::*;
    pub use cgmath::{Point2, Vector2};
    pub use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
    pub use ggez::graphics::{Color, Image, Rect};
    pub use ggez::{Context, GameResult};
    pub use serde_json::Value;
    pub use std::cmp::Ordering::{Greater, Less};
    pub use std::ops::Add;
}

use prelude::*;

struct MainState {
    game: Game,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (w, h) = ggez::graphics::drawable_size(ctx);

        let map_file = include_bytes!("Wolf3dLevel_1.json");
        let map = serde_json::from_slice(map_file).unwrap();

        let game = Game::new(ctx, &map, w as usize, h as usize);

        let s = MainState { game };

        Ok(s)
    }
}
impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        const DESIRED_FPS: f32 = 30.0;

        let delta = 1.0 / DESIRED_FPS;

        self.game.update(delta);

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, Color::BLUE);

        self.game.canvas.draw(ctx)?;
        self.game.canvas.draw_fps(ctx)?;

        ggez::graphics::present(ctx)?;

        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.game.player.handle_inputs(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.game.player.handle_inputs(keycode, false);
    }
    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32) {
        self.game.new_size(ctx, w as usize, h as usize);

        let coordinates = ggez::graphics::Rect::new(0.0, 0.0, w, h);

        ggez::graphics::set_screen_coordinates(ctx, coordinates).expect("Can't resize the window");
    }
}
pub fn main() -> GameResult {
    let conf = ggez::conf::Conf::default()
        .window_resizable(true)
        .window_title("wolf3d v1.0.0, 2022".to_string());

    ggez::start(conf, |mut context| {
        Box::new(MainState::new(&mut context).unwrap())
    })
}
