//! Schema migrations (docs/DESIGN.md §1 row `storage/db.rs, schema.sql,
//! migrations.rs`; §6 "Tables (`schema.sql`)").
//!
//! The schema is an ordered list of SQL steps, one per version, embedded in
//! the binary. `schema_version` records every version that has been applied;
//! the current version is the largest one recorded. Running the migrations
//! again applies nothing (idempotent), and a database written by a newer
//! build is refused rather than silently reinterpreted
//! ([`StorageError::SchemaTooNew`]).
//!
//! Each step runs inside its own SQLite transaction together with the
//! `schema_version` row, so a crash mid-step leaves the previous version
//! intact. No step reads a clock: `schema_version` carries only the version
//! number (BRIEF_v2 §4 "no timestamps decide anything").

use rusqlite::{Connection, OptionalExtension, params};

use super::db::StorageError;

/// One schema step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Migration {
    /// Version this step brings the database to. Steps are listed in
    /// strictly increasing order starting at 1.
    pub version: u32,
    /// SQL executed as one batch, inside a transaction.
    pub sql: &'static str,
}

/// Every schema step, oldest first. v1 is `schema.sql`.
pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    sql: include_str!("schema.sql"),
}];

/// The version a fully migrated database reports.
pub const LATEST_VERSION: u32 = match MIGRATIONS.last() {
    Some(m) => m.version,
    None => 0,
};

/// The version recorded in `schema_version`, or 0 for a database with no
/// schema yet (including one where the table itself does not exist).
pub fn current_version(conn: &Connection) -> Result<u32, StorageError> {
    let has_table: Option<String> = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .optional()?;
    if has_table.is_none() {
        return Ok(0);
    }
    let max: Option<i64> =
        conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        })?;
    match max {
        None => Ok(0),
        Some(v) => u32::try_from(v).map_err(|_| StorageError::Corrupt {
            what: "schema_version".to_owned(),
        }),
    }
}

/// Applies every step above the recorded version and returns the resulting
/// version. Safe to call on every open.
pub fn run(conn: &mut Connection) -> Result<u32, StorageError> {
    let mut version = current_version(conn)?;
    if version > LATEST_VERSION {
        return Err(StorageError::SchemaTooNew {
            found: version,
            supported: LATEST_VERSION,
        });
    }
    for step in MIGRATIONS {
        if step.version <= version {
            continue;
        }
        let txn = conn.transaction()?;
        txn.execute_batch(step.sql)?;
        // `schema.sql` creates the table in v1; for safety every step may
        // assume it exists.
        txn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
        )?;
        txn.execute(
            "INSERT OR IGNORE INTO schema_version (version) VALUES (?1)",
            params![i64::from(step.version)],
        )?;
        txn.commit()?;
        version = step.version;
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_ordered_from_one() {
        for (i, m) in MIGRATIONS.iter().enumerate() {
            assert_eq!(usize::try_from(m.version).unwrap(), i + 1);
        }
        assert_eq!(LATEST_VERSION, u32::try_from(MIGRATIONS.len()).unwrap());
    }

    #[test]
    fn fresh_db_reports_zero_then_latest_and_is_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        assert_eq!(current_version(&conn).unwrap(), 0);
        assert_eq!(run(&mut conn).unwrap(), LATEST_VERSION);
        assert_eq!(current_version(&conn).unwrap(), LATEST_VERSION);
        assert_eq!(run(&mut conn).unwrap(), LATEST_VERSION);
        let rows: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, i64::from(LATEST_VERSION));
        for table in ["kv", "meta", "schema_version"] {
            let n: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(n, 1, "table {table} missing");
        }
    }

    #[test]
    fn newer_schema_is_refused() {
        let mut conn = Connection::open_in_memory().unwrap();
        run(&mut conn).unwrap();
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (?1)",
            params![i64::from(LATEST_VERSION) + 1],
        )
        .unwrap();
        assert!(matches!(
            run(&mut conn),
            Err(StorageError::SchemaTooNew { found, supported })
                if found == LATEST_VERSION + 1 && supported == LATEST_VERSION
        ));
    }

    #[test]
    fn negative_version_is_corrupt_not_panic() {
        let mut conn = Connection::open_in_memory().unwrap();
        run(&mut conn).unwrap();
        conn.execute("DELETE FROM schema_version", []).unwrap();
        conn.execute("INSERT INTO schema_version (version) VALUES (-7)", [])
            .unwrap();
        assert!(matches!(
            current_version(&conn),
            Err(StorageError::Corrupt { .. })
        ));
    }
}
