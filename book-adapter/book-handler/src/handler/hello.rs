use actix_web::{get, /* web, */ Responder};

// use book_usecase::BookUsecase;

#[get("/hello")]
async fn hello(/*usecase: web::Data<BookUsecase<AdapterImpl>>*/) -> impl Responder {
    // usecase.get_book().await
    "hello"
}
