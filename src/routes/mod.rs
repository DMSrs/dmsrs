
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub mod documents;
pub mod index;
pub mod tags;

pub struct RoutesHandler {
    pub pool: Pool<PostgresConnectionManager>,
}

