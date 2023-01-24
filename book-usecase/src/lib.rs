use book_domain::repository::BookRepository;

pub trait HaveBookRepository {
    type R: BookRepository;
    fn repository(&self) -> Self::R;
}

pub trait BookService: HaveBookRepository {
    fn get_book(&self) -> String {
        self.repository().get()
    }
}

impl<T: HaveBookRepository> BookService for T {}

pub trait HaveBookService {
    type S: BookService;
    fn book_service(&self) -> Self::S;
}
