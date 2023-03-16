use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::info;

use crate::features::task_assignment::interface;
use simple_logger;
mod core;
mod features;

#[get("/")]
async fn hello() -> impl Responder {
    info!("Hello There sorry for the trouble");

    HttpResponse::Accepted().body(String::from("Wrong way buddy"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    info!("Server Starting . . . . .");

    HttpServer::new(move || {
        info!("Server Service Beginning . . . . .");
        App::new()
            .configure(interface::task_interface_config)
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
