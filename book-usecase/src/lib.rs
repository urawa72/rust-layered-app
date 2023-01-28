use book_domain::repository::BookRepository;

pub struct BookUsecase {
    pub adapters: Box<dyn Adapters>,
}

impl BookUsecase {
    pub fn new(adapters: impl Adapters + 'static) -> Self {
        BookUsecase {
            adapters: Box::new(adapters),
        }
    }

    pub fn hello(&self) -> String {
        self.adapters.repository().get()
    }
}

pub trait Adapters: Sync + Send {
    fn repository(&self) -> Box<dyn BookRepository>;
}
