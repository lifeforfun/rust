use actix_web::{server, App};

use self::handler::*;
mod handler;

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.with(home::index::index))
    }).bind("127.0.0.1:0080")
        .unwrap()
        .run();
}