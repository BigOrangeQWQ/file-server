// use rocksdb::{BoundColumnFamily, ColumnFamilyDescriptor, DBCommon, MultiThreaded, Options, SingleThreaded, DB};

// static DBPATH: &str = "./data/rocksdb.db";



// #[derive(Clone)]
// pub struct Database {
//     db: DB,
// }

// impl Database {

//     pub fn new() -> Self {
//         // let db = DB::open_default(DBPATH).unwrap();
//         let mut cf_opts = Options::default();
//         cf_opts.set_max_write_buffer_number(16);
//         let cf = ColumnFamilyDescriptor::new("cf1", cf_opts);

//         let mut db_opts = Options::default();
//         db_opts.create_missing_column_families(true);
//         db_opts.create_if_missing(true);
        
//         let db = DB::open_cf_descriptors(&db_opts, DBPATH, vec![cf]).unwrap();
//         Self { db }
//     }

//     pub fn put(&self, key: &str, value: &str) {
//         self.db.put(key.as_bytes(), value.as_bytes()).unwrap();
//     }

//     pub fn get(&self, key: &str) -> Option<String> {
//         match self.db.get(key.as_bytes()) {
//             Ok(Some(value)) => Some(String::from_utf8(value).unwrap()),
//             Ok(None) => None,
//             Err(e) => None,
//         }
//     }

//     pub fn delete(&self, key: &str) {
//         self.db.delete(key.as_bytes()).unwrap();
//     }
// }


use std::time::Duration;

use redb::{Database, ReadableTable, TableDefinition};
// use salvo::http::cookie::time::Duration;
use tokio::time::Instant;

const TOKENTABLE: TableDefinition<&str, &str> = TableDefinition::new("tokens");
// const TIMETABLE: TableDefinition<&str, u64> = TableDefinition::new("times");
const DBPATH: &str = "./data/token.db";

pub struct TokenData {
    db: redb::Database,
}

impl TokenData {
    pub fn new() -> Self {
        // Database::create(DBPATH);
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

            // let mut table = write_txn.open_table(TIMETABLE).unwrap();
            // let _ = table.insert(key, (Instant::now().elapsed() + Duration::new(60 * 60 * 24 , 0)).as_secs());
            // println!("now {}", Instant::now().elapsed().as_secs());
            // println!("future{}", (Instant::now().elapsed() + Duration::new(60 * 60 * 24 , 0)).as_secs())

            // println!("inserted")
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
        }
    }
}