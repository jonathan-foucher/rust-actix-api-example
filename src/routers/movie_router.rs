use actix_web::{delete, error, get, post, web, web::Data, HttpResponse, Responder};
use diesel::{pg::PgConnection, prelude::*, r2d2::{ConnectionManager, Pool}};
use crate::models::{movie::Movie, schema::movie::dsl::*};

#[get("/movies")]
async fn get_all_movies(db_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> actix_web::Result<impl Responder> {
    println!("Get all movies");
    let movies: Vec<Movie> = web::block(move || {
        let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");
        movie.load(&mut db_conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(movies))
}

#[post("/movies")]
async fn save_movie(body: web::Json<Movie>) -> impl Responder {
   	println!("Post movie with id {}", body.id);
	println!("Post movie with title {}", body.title);
	println!("Post movie with release date {}", body.release_date);
    HttpResponse::Ok()
}

#[delete("/movies/{movieId}")]
async fn delete_movie(path: web::Path<i32>) -> impl Responder {
    let movie_id = path.into_inner();
    println!("Delete movie with id {}", movie_id);
    HttpResponse::Ok()
}
