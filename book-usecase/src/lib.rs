pub mod get_book;

use book_domain::repository::BookRepository;

pub struct BookUsecase<A>
where
    A: Adapter,
{
    pub adapter: A,
}

impl<A: Adapter> BookUsecase<A> {
    pub fn new(adapter: A) -> Self {
        BookUsecase { adapter }
    }
}

pub trait Adapter {
    type R: BookRepository;

    fn repository(&self) -> Self::R;
}
