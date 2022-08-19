use crate::db;
use crate::schema::authors;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "authors"]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "authors"]
pub struct Authors {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Authors {
    pub fn find_all() -> Result<Vec<Self>, Error> {
        let connection = db::connection();
        let authors = authors::table.load::<Authors>(&connection)?;
        Ok(authors)
    }

    pub fn find(id: i32) -> Result<Self, Error> {
        let connection = db::connection();
        let author = authors::table
            .filter(authors::id.eq(id))
            .first(&connection)?;
        Ok(author)
    }

    pub fn create(author: Author) -> Result<Self, Error> {
        let connection = db::connection();
        let author = Author::from(author);
        let author = diesel::insert_into(authors::table)
            .values(author)
            .get_result(&connection)?;
        Ok(author)
    }
}
