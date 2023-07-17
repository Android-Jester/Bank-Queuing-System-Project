use actix_cors::Cors;
use actix_web::{http::KeepAlive, *};
use queuing_server::prelude::broadcaster::Broadcaster;
use queuing_server::prelude::*;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};

/// Main File for runnning server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // log::info!(
    //     "Web Server started at http://{}:3000",
    //     Ipv4Addr::UNSPECIFIED
    // );

    let data = Broadcaster::create();

    let queue_data_main: web::Data<Mutex<MainQueue>> =
        web::Data::new(Mutex::new(MainQueue::default()));
    let queue_data_sub: web::Data<Mutex<SubQueues>> =
        web::Data::new(Mutex::new(SubQueues::default()));

    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .supports_credentials()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(queue_data_main.clone())
            .app_data(queue_data_sub.clone())
            .app_data(web::Data::from(Arc::clone(&data)))
            .service(list_users)
            .configure(teller_config) /* Teller Actions */
            .configure(user_config) /* User Actions */
    })
    .keep_alive(KeepAlive::Os)
    .bind((Ipv4Addr::UNSPECIFIED, 3000))?
    .run()
    .await
}
