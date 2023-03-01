use std::sync::Arc;
use actix_web::{web, App, HttpServer};

use app::config::Config;
use database::{repository::BookRepositoryImpl, setup};
use usecase::{Adapters, BookUsecase};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    let config = Config::load();

    // DI
    let db = setup(&config.database_url)
        .await
        .expect("Failed to setup database");
    let repo = BookRepositoryImpl::new(db);
    let adapters = AdaptersImpl { repo };
    let usecase = Arc::new(BookUsecase::new(adapters));

    // Run server
    println!("Listening 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(usecase.clone()))
            .service(rest::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Clone)]
struct AdaptersImpl {
    pub repo: BookRepositoryImpl,
}

impl Adapters for AdaptersImpl {
    fn repository(&self) -> Box<dyn model::repository::BookRepository> {
        Box::new(self.repo.clone())
    }
}