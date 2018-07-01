
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub mod index;
pub mod documents;

pub struct RoutesHandler {
    pub pool: Pool<PostgresConnectionManager>,
}

