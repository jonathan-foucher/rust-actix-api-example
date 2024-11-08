use actix_web::{web, App, HttpServer};

use crate::routers::movie_router::{get_all_movies, save_movie, delete_movie};

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .service(get_all_movies)
                .service(save_movie)
                .service(delete_movie)
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
