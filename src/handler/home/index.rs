use actix_web::{Path, Responder};

pub fn index(info: Path<(String, u32)>) -> impl Responder {
    format!("Hello {}! id:123{}", info.0, info.1)
}