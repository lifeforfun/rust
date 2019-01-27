use actix_web::{server, App};

use self::handler::*;
mod handler;

fn main() {
    server::new(|| {
        App::new().resource("/{name}/{id}/home.html", |r| r.with(handler::home::index::index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}