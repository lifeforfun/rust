extern crate ggez;

use self::ggez::error::GameResult;
use self::ggez::event::EventHandler;
use self::ggez::graphics::DrawMode;
use self::ggez::graphics::Point2;
use self::ggez::graphics::Rect;
use self::ggez::*;
use std::fs::File;

#[derive(Debug)]
struct Ball {
    c: Point2,
    r: f32,
}

#[derive(Debug)]
struct MainState {
    rect: Rect,
    ball: Ball,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            rect: Rect::new(0.0, 0.0, 200.0, 50.0),
            ball: Ball {
                c: Point2::new(300.0, 300.0),
                r: 10.0,
            },
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.rect.x += 1.0;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let MainState { rect, ball } = self;
        graphics::rectangle(ctx, DrawMode::Fill, *rect)?;
        graphics::circle(ctx, DrawMode::Fill, ball.c, ball.r, 0.0001)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn init() -> Result<(), GameError> {
    let mut conf_file = File::open("game_conf.toml")?;
    let config = conf::Conf::from_toml_file(&mut conf_file)?;
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", config)?;
    let mut main_state = MainState::new(ctx)?;
    event::run(ctx, &mut main_state)?;
    Ok(())
}

pub fn test() {
    match init() {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    }
}
