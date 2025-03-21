use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tracing_actix_web::TracingLogger;

mod model;
mod schema;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
