use redb::{Database, ReadableTable, TableDefinition};

const TOKENTABLE: TableDefinition<&str, &str> = TableDefinition::new("tokens");
// const TIMETABLE: TableDefinition<&str, u64> = TableDefinition::new("times");
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
