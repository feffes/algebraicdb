mod manager;
mod read;
mod wal;
mod write;

pub(crate) use manager::spawn_snapshotter;
pub(crate) use read::load_db_data;
pub(crate) use wal::{TransactionNumber, WriteAheadLog, WriteToWal};
pub(crate) use write::initialize_data_dir;
pub(self) use write::snapshot;

// Data-directory layout:
// - <data_dir>
// | - wal                    (the write-ahead log)
// | - current                (contains the current transaction number, acts as an atomic pointer to the folder)
// | - <transaction_number>   (a folder containing a snapshot of the database at the given transaction)
// | | - type_map             (a file containing all type definitions for the database)
// | | - tables               (a folder containing the raw data of all tables)
// | | | - <table_name>       (raw data of the table)
pub(self) const WAL_FILE_NAME: &str = "wal";
pub(self) const TNUM_FILE_NAME: &str = "tnum";
pub(self) const TMP_EXTENSION: &str = "tmp";
pub(self) const TABLES_DIR_NAME: &str = "tables";
pub(self) const TYPE_MAP_FILE_NAME: &str = "type_map";

// All top-level data dir files
pub(self) const DATA_DIR_FILES: &[&str] = &[WAL_FILE_NAME, TNUM_FILE_NAME];
