use actix_web::{web, App, HttpServer};

use crate::routers::movie_router::{get_all_movies, save_movie, delete_movie};

const DEFAULT_PORT: u16 = 8080;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    let http_port: u16 = std::env::var("HTTP_PORT")
        .map(|http_port| http_port.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);
    println!("Application is starting on port {}", http_port);

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .service(get_all_movies)
                .service(save_movie)
                .service(delete_movie)
            )
    })
    .bind(("127.0.0.1", http_port))?
    .run()
    .await
}
