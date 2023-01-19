use book_app::{adapters::AdapterImpl, config::Config};
use book_database::repository::BookRepositoryImpl;
use book_usecase::BookUsecase;

fn main() {
    println!("start");

    // DI
    let config = Config::load();
    println!("{:?}", config);
    let db = "dummy".to_string();
    let repo = BookRepositoryImpl::new(db);
    let adapter = AdapterImpl { repo };
    let usecase = BookUsecase::new(adapter);

    usecase.get_book();

    println!("end");
}
