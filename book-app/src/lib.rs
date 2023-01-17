use book_database::repository::BookRepositoryImpl;
use book_usecase::Adapter;

pub struct AdapterImpl {
    pub repo: BookRepositoryImpl,
}

impl Adapter for AdapterImpl {
    type R = BookRepositoryImpl;

    fn repository(&self) -> Self::R {
        self.repo.clone()
    }
}
