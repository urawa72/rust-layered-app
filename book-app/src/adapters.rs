use book_database::repository::BookRepositoryImpl;
use book_usecase::Adapter;

#[derive(Clone, Debug)]
pub struct AdapterImpl {
    pub repo: BookRepositoryImpl,
}

impl Adapter for AdapterImpl {
    type A = BookRepositoryImpl;

    fn repository(&self) -> &Self::A {
        &self.repo
    }
}
