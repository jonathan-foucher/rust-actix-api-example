use actix_web::{delete, error, get, post, web, web::{Data, Json}, HttpResponse, Responder, Result};
use diesel::{delete, insert_into, pg::PgConnection, prelude::*, r2d2::{ConnectionManager, Pool}, upsert::excluded};
use crate::models::movie::Movie;
use crate::schema::movie::dsl::*;

#[get("/movies")]
async fn get_all_movies(db_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> Result<impl Responder> {
    log::info!("Get all movies");

    let movies: Vec<Movie> = web::block(move || {
        let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");
        movie.load(&mut db_conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(movies))
}

#[post("/movies")]
async fn save_movie(db_pool: Data<Pool<ConnectionManager<PgConnection>>>, body: Json<Movie>) -> Result<impl Responder> {
   	log::info!("Post movie id={}, title='{}' and relase_date={}", body.id, body.title, body.release_date);

    let result: Movie = web::block(move || {
        let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");
        let _ = insert_into(movie).values(&*body)
            .on_conflict(id)
            .do_update()
            .set((
                    title.eq(excluded(title)),
                    release_date.eq(excluded(release_date))
                ))
            .execute(&mut db_conn);

        movie.filter(id.eq(body.id)).first(&mut db_conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/movies/{movieId}")]
async fn delete_movie(db_pool: Data<Pool<ConnectionManager<PgConnection>>>, path: web::Path<i32>) -> Result<impl Responder> {
    let movie_id = path.into_inner();
    log::info!("Delete movie with id {}", movie_id);

    let result = web::block(move || {
        let mut db_conn = db_pool.get().expect("Could not get a database connection from the pool");
        delete(movie.filter(id.eq(movie_id))).execute(&mut db_conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(if result == 0 {HttpResponse::NoContent()} else {HttpResponse::Ok()})
}
