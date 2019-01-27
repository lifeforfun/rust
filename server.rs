use actix_web::{server, App, Path, Responder};

fn index(info: Path<(String, u32)>) -> impl Responder {
    format!("Hello {}! id:{}", info.0, info.1)
}

fn main() {
    server::new(|| {
        App::new().resource("/{name}/{id}/index.html", |r| r.with(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}