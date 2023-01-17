use book_domain::repository::BookRepository;

// use crate::setup::Db;

#[derive(Clone)]
pub struct BookRepositoryImpl {
    // pub pool: Db,
    pub pool: String,
}

impl BookRepositoryImpl {
    // pub fn new(pool: Db) -> Self {
    //     BookRepositoryImpl { pool }
    // }
    pub fn new(pool: String) -> Self {
        BookRepositoryImpl { pool }
    }
}

impl BookRepository for BookRepositoryImpl {
    fn get(&self) {
        println!("hello world!")
    }
}
