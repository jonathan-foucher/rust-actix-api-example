use diesel::{pg::PgConnection, r2d2::{ConnectionManager, Pool}};
use std::env;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    println!("Connecting to the database");
    let database_url = env::var("DB_URL").expect("DB_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
}
