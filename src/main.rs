use std::sync::Arc;

use actix_web::{web, HttpServer, App, middleware};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use models::AppState;

mod models;
mod handlers;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Arc::new(
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool."),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(handlers::index)
    })
    .workers(16)
    .bind("0.0.0.0:8100")?
    .run()
    .await
}