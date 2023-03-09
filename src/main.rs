use actix_web::{web, HttpServer, App, middleware};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

mod models;
mod handlers;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(handlers::index)
    })
    .workers(16)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}