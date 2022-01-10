use rusqlite::*;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option< Vec< u8 > >
}

pub struct DB {
    connection: Connection
}

impl DB {
    pub fn open( path: &str ) -> Result<DB>
    {
        let conn = Connection::open( path )?;

        Ok( DB{
            connection: conn
        })
    }
}