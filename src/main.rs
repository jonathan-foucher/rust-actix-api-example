mod routers;
mod models;
mod database;
mod schema;
use database::database_connection::create_db_pool;
use diesel::{pg::PgConnection, r2d2::{ConnectionManager, Pool}};
use std::{env, io::Result};
use actix_web::{middleware::Logger, web::{scope, Data}, App, HttpServer};
use crate::routers::movie_router::{get_all_movies, save_movie, delete_movie};

const DEFAULT_HTTP_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_pool: Pool<ConnectionManager<PgConnection>> = create_db_pool();

    let http_port: u16 = env::var("HTTP_PORT")
        .map(|http_port| http_port.parse::<u16>().unwrap_or(DEFAULT_HTTP_PORT))
        .unwrap_or(DEFAULT_HTTP_PORT);
    log::info!("Application is starting on port {}", http_port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(scope("/api")
                .app_data(Data::new(db_pool.clone()))
                .service(get_all_movies)
                .service(save_movie)
                .service(delete_movie)
            )
    })
    .bind(("127.0.0.1", http_port))?
    .run()
    .await
}
