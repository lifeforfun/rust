use actix_web::{Responder};

pub fn index(_info: String) -> impl Responder {
    format!("Hello World!")
}