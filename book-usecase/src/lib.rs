pub mod get_book;

use book_domain::repository::BookRepository;

#[derive(Clone, Debug)]
pub struct BookUsecase<A> {
    pub adapter: A,
}

impl<A: Adapter> BookUsecase<A> {
    pub fn new(adapter: A) -> Self {
        BookUsecase { adapter }
    }
}

pub trait Adapter {
    type A: BookRepository;

    fn repository(&self) -> &Self::A;
}
