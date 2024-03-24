use std::fs::remove_file;

use chrono::Utc;
use redb::{Database, ReadableTable, TableDefinition};

use crate::util::next_day_time;

const TOKENTABLE: TableDefinition<&str, &str> = TableDefinition::new("tokens");
const TIMETABLE: TableDefinition<&str, i64> = TableDefinition::new("times");
const DBPATH: &str = "./data/token.db";

pub struct TokenData {
    db: redb::Database,
}

impl TokenData {
    pub fn new() -> Self {
        
        Self {
            db: Database::create(DBPATH).unwrap(),
        }
    }

    pub fn put(&self, key: &str, value: &str) {
        // self.db.put(TABLE, key, value).unwrap();
        let write_txn = self.db.begin_write().unwrap();
        {
            let mut table = write_txn.open_table(TOKENTABLE).unwrap();
            let _ = table.insert(key, value);

            let mut table = write_txn.open_table(TIMETABLE).unwrap();
            let _ = table.insert(key, next_day_time());
        }
        write_txn.commit().unwrap();
    }

    pub fn get(&self, key: &str) -> String {
        let read_txn = self.db.begin_read().unwrap();
        let table = read_txn.open_table(TOKENTABLE).unwrap();
        let value = table.get(key).unwrap().unwrap().value().to_string();
        // println!("got value: {}", value);
        value
    }

    pub fn delete_key(&self, key: &str) {
        let write_txn = self.db.begin_write().unwrap();
        {
            let mut table = write_txn.open_table(TOKENTABLE).unwrap();
            let _ = table.remove(key);

            let mut table = write_txn.open_table(TIMETABLE).unwrap();
            let _ = table.remove(key);
        }
    }

    pub fn delete_timeout_key(&self) {
        let read_txn = self.db.begin_read().unwrap();

        if let Ok(table) = read_txn.open_table(TIMETABLE){
            table.iter().unwrap().for_each(
                |result: Result<(redb::AccessGuard<'_, &str>, redb::AccessGuard<'_, i64>), redb::StorageError>| {
                    if let Ok((key, value)) = result {
                        if value.value() < Utc::now().timestamp() {
                            remove_file(format!("data/{}", self.get(key.value()))).unwrap();
                            self.delete_key(key.value());
                        }
                    }
                }
            );
        }
        else {
            tracing::info!("time table not found");
        }
    }
}
