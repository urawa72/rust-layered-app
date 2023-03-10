use actix_web::{get, web, App, HttpServer, Responder};

use book_app::{adapters::AdapterImpl, config::Config};
use book_database::{repository::BookRepositoryImpl, setup};
use book_usecase::BookUsecase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    let config = Config::load();

    // DI
    let db = setup(&config.database_url)
        .await
        .expect("Failed to setup database");
    let repo = BookRepositoryImpl::new(db);
    let adapter = AdapterImpl { repo };
    let usecase = BookUsecase::new(adapter);

    // Run server
    println!("Listening 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(usecase.clone()))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello")]
async fn hello(usecase: web::Data<BookUsecase<AdapterImpl>>) -> impl Responder {
    usecase.get_book().await
}
