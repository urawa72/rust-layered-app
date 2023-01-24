use actix_web::{web, App, HttpServer};

use book_app::config::Config;
use book_database::{repository::BookRepositoryImpl, setup};
use book_usecase::{HaveBookRepository, HaveBookService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    let config = Config::load();

    // DI
    let db = setup(&config.database_url)
        .await
        .expect("Failed to setup database");
    let repo = BookRepositoryImpl::new(db);
    let service = DummyService { repo };

    // Run server
    println!("Listening 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .service(book_handler::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Clone)]
struct DummyService {
    pub repo: BookRepositoryImpl,
}

impl HaveBookRepository for DummyService {
    type R = BookRepositoryImpl;
    fn repository(&self) -> Self::R {
        self.repo.clone()
    }
}

impl HaveBookService for DummyService {
    type S = Self;
    fn book_service(&self) -> Self::S {
        self.clone()
    }
}
