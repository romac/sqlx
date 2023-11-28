use futures_core::future::BoxFuture;

use crate::{Sqlite, SqliteConnection};
use sqlx_core::error::Error;
use sqlx_core::transaction::TransactionManager;

/// Implementation of [`TransactionManager`] for SQLite.
pub struct SqliteTransactionManager;

impl TransactionManager for SqliteTransactionManager {
    type Database = Sqlite;

    fn begin(conn: &mut SqliteConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.begin())
    }

    fn begin_concurrent(
        conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.begin_concurrent())
    }

    fn commit(conn: &mut SqliteConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.commit())
    }

    fn rollback(conn: &mut SqliteConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.rollback())
    }

    fn start_rollback(conn: &mut SqliteConnection) {
        conn.worker.start_rollback().ok();
    }
}
