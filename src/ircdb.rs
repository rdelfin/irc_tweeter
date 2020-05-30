use rusqlite::{params, Connection, Result, Row};

#[derive(Debug)]
pub struct Quote {
    pub id: i64,
    pub score: i64,
    pub quote: String,
}

impl Quote {
    fn from_row(row: &Row) -> Result<Quote> {
        Ok(Quote {
            id: row.get(0)?,
            score: row.get(1)?,
            quote: row.get(2)?,
        })
    }
}

pub struct IrcDb {
    conn: Connection,
}

impl IrcDb {
    pub fn new(db_file: &str) -> Result<IrcDb> {
        Ok(IrcDb {
            conn: Connection::open(db_file)?,
        })
    }

    pub fn get_random(&self, num: i64, max_length: i64) -> Result<Quote> {
        self.conn
            .prepare("SELECT * FROM quotes WHERE LENGTH(quote) < ? order by RANDOM() limit ?")?
            .query_row(params![max_length, num], Quote::from_row)
    }

    #[allow(dead_code)]
    pub fn get_all(&self) -> Result<Vec<Quote>> {
        self.conn
            .prepare("SELECT * FROM quotes")?
            .query_map(params![], Quote::from_row)?
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_count(&self) -> Result<i64> {
        self.conn
            .prepare("SELECT COUNT(*) FROM quotes")?
            .query_row(params![], |row| row.get(0))
    }
}
