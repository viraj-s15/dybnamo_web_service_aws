mod api;
mod model;
mod repo;

use api::task::get_task;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App.new()
            .wrap(logger)
            // .app_data() Implement some shared state between the app
            .service(get_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
