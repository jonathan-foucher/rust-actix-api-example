use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::models::movie::Movie;

#[get("/movies")]
async fn get_all_movies() -> impl Responder {
    println!("Get all movies");
    HttpResponse::Ok().body("Get all movies")
}

#[post("/movies")]
async fn save_movie(movie: web::Json<Movie>) -> impl Responder {
   	println!("Post movie with id {}", movie.id);
	println!("Post movie with title {}", movie.title);
	println!("Post movie with release date {}", movie.release_date);
    HttpResponse::Ok()
}

#[delete("/movies/{movieId}")]
async fn delete_movie(path: web::Path<u32>) -> impl Responder {
    let movie_id = path.into_inner();
    println!("Delete movie with id {}", movie_id);
    HttpResponse::Ok()
}
