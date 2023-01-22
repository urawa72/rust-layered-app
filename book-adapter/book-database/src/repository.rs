use book_domain::repository::BookRepository;

use crate::Db;

#[derive(Clone, Debug)]
pub struct BookRepositoryImpl {
    pub pool: Db,
}

impl BookRepositoryImpl {
    pub fn new(pool: Db) -> Self {
        BookRepositoryImpl { pool }
    }
}

impl BookRepository for BookRepositoryImpl {
    fn get(&self) -> String {
        "hello!".to_string()
    }
}
