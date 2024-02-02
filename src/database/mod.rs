use diesel::pg::PgConnection;
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    println!("{:?}", manager);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
