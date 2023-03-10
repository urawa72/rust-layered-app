use book_domain::repository::BookRepository;

use crate::{Adapter, BookUsecase};

impl<A: Adapter> BookUsecase<A> {
    pub async fn get_book(&self) -> String {
        self.adapter.repository().get()
    }
}
